use zk_host::methods::{GUEST_ELF, GUEST_ID};

use risc0_zkvm::{default_executor_from_elf, serde::from_slice, ExecutorEnv};

use std::fs::File;
use std::io::BufReader;
use std::time::Instant;

fn main() {
    let now = Instant::now();

    println!("Setting up env...");

    if std::env::args().len() != 4 {
        panic!("Expected 4 arguments. Received {}.", std::env::args().len())
    }

    let gamma_filepath = std::env::args().nth(1).expect("No claim file path given");
    let gamma_file = File::open(gamma_filepath).expect("The gamma file was not found");
    let gamma_reader = BufReader::new(gamma_file);

    let claims_filepath = std::env::args().nth(2).expect("No claim file path given");
    let claims_file = File::open(claims_filepath).expect("The claims file was not found");
    let claims_reader = BufReader::new(claims_file);

    let proof_filepath = std::env::args().nth(3).expect("No proof file path given");
    let proof_file = File::open(proof_filepath).expect("The proof file was not found");
    let proof_reader = BufReader::new(proof_file);

    // First, we construct an executor environment
    let env = ExecutorEnv::builder()
        .read_fd(10, gamma_reader)
        .read_fd(11, claims_reader)
        .stdin(proof_reader)
        .build()
        .unwrap();

    // Next, we make an executor, loading the (renamed) ELF binary.
    let mut exec = default_executor_from_elf(env, GUEST_ELF).unwrap();

    println!("Checking the proof...");

    // Run the executor to produce a session.
    let session = exec.run().unwrap();

    println!("Ran in {} s", now.elapsed().as_secs());

    println!("Generating the certificate...");

    // Prove the session to produce a receipt.
    let receipt = session.prove().unwrap();

    // Small fetcher that returns the next chunk of given size from journal
    let mut current_index: usize = 0;
    let mut next_journal_chunk = |size: usize| -> &[u8] {
        let ret = &receipt.journal[current_index..current_index + size];
        current_index += size;
        return ret;
    };

    // TODO: Implement code for transmitting or serializing the receipt for
    // other parties to verify here

    // Optional: Verify receipt to confirm that recipients will also be able to
    // verify your receipt
    receipt.verify(GUEST_ID).unwrap();

    println!(
        "Running execution + ZK certficate generation + verification took {} s",
        now.elapsed().as_millis()
    );

    // Get the host's size of a usize pointer
    let size_of_usize = std::mem::size_of::<usize>();
    let total_cycles: usize = from_slice(next_journal_chunk(size_of_usize)).unwrap();
    println!("Total cycles {}", total_cycles);

    let gamma_length: usize = from_slice(next_journal_chunk(size_of_usize)).unwrap();
    let gamma = next_journal_chunk(gamma_length);
    let claims = &receipt.journal[current_index..];
    println!("There exists a proof of {:?} |- {:?}.", gamma, claims);
}
