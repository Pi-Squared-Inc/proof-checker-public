use zk_prover::methods::{CHECKER_ELF, CHECKER_ID, SDC_ELF, SDC_ID};

use risc0_zkvm::{default_prover, serde::from_slice, ExecutorEnv, Receipt};

use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::Write;
use std::time::Instant;

use serde::{Deserialize, Serialize};

fn main() {
    let now = Instant::now();

    println!("Setting up env...");

    if std::env::args().len() != 5 {
        panic!("Expected 5 arguments. Received {}.", std::env::args().len())
    }

    let gamma_target_filepath = std::env::args().nth(1).expect("No gamma file path given");
    let gamma_target_file =
        File::open(gamma_target_filepath).expect("The gamma file was not found");
    let gamma_target_reader = BufReader::new(gamma_target_file);

    let claims_target_filepath = std::env::args().nth(2).expect("No claim file path given");
    let claims_target_file =
        File::open(claims_target_filepath).expect("The claims file was not found");
    let claims_target_reader = BufReader::new(claims_target_file);

    let checker_input_dir = std::env::args().nth(3).expect("No input directory given");

    let output_filepath = std::env::args().nth(4).expect("No output file path given");
    let mut output_file =
        File::create(output_filepath.clone()).expect("Unable to create output file");

    // Collect Proof Checker Inputs
    let mut inputs = Vec::new();
    let dir = std::fs::read_dir(checker_input_dir).unwrap();
    let mut dir_vec: Vec<Result<fs::DirEntry, std::io::Error>> = dir.collect();
    dir_vec.sort_by(|a, b| a.as_ref().unwrap().path().cmp(&b.as_ref().unwrap().path()));
    for entry in dir_vec {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            let path_as_str = path.to_str().unwrap();
            if path_as_str.contains(".target.") {
                continue;
            }

            if path_as_str.ends_with(".ml-gamma") {
                let gamma = fs::read(path.clone()).expect("Unable to read file");
                let mut claim_path = path.clone();
                claim_path.set_extension("ml-claim");
                let claim = fs::read(claim_path).expect("Unable to read file");
                let mut proof_path = path.clone();
                proof_path.set_extension("ml-proof");
                let proof = fs::read(proof_path).expect("Unable to read file");
                inputs.push((gamma, claim, proof));
            }
        }
    }

    // First, we construct an executor environment
    let mut env = ExecutorEnv::builder();

    let setuptime = now.elapsed().as_millis();
    println!("Setup took {} ms", setuptime);

    let mut proof_checker_receipts = Vec::new();
    for i in 0..inputs.len() {
        let (gamma, claim, proof) = &inputs[i];
        let ini = Instant::now();

        // Then, we add the specific files for the proof checker
        let env_checker = env
            .read_fd(10, gamma.as_slice())
            .read_fd(11, claim.as_slice())
            .stdin(proof.as_slice())
            .build()
            .unwrap();

        // Next, we make a prover.
        let prover = default_prover();

        println!("============================================");
        println!("Checking the proof and generating the receipt...");

        // Run the prover on the ELF binary to produce a receipt.
        let receipt = prover.prove_elf(env_checker, CHECKER_ELF).unwrap();
        println!(
            "Proved in {} ms",
            Instant::now().duration_since(ini).as_millis()
        );
        proof_checker_receipts.push(receipt);
    }

    let checker_provetime = now.elapsed().as_millis();
    println!("============================================");

    // Prepare the SDC inputs and environment
    println!("Setting SDC env...");

    let mut obligations: Vec<(Vec<u8>, Vec<u8>)> = Vec::new();
    for i in 0..inputs.len() {
        let (gamma, claim, _) = &inputs[i];
        obligations.push((gamma.clone(), claim.clone()));
    }

    let env_sdc = env
        .read_fd(12, gamma_target_reader)
        .read_fd(13, claims_target_reader)
        .write(&obligations)
        .expect("Could not write obligations to file")
        .build()
        .unwrap();

    // Next, we make a prover.
    let prover = default_prover();

    println!("Checking sdc proof and generating the receipt...");

    // Run the prover on the ELF binary to produce a receipt.
    let sdc_receipt = prover.prove_elf(env_sdc, SDC_ELF).unwrap();

    // Small fetcher that returns the next chunk of given size from journal
    let mut current_index: usize = 0;
    let mut next_journal_chunk = |size: usize| -> &[u8] {
        let ret = &sdc_receipt.journal.bytes[current_index..current_index + size];
        current_index += size;
        return ret;
    };

    // Get the host's size of a usize pointer
    let size_of_usize = std::mem::size_of::<usize>();
    let _: usize = from_slice(next_journal_chunk(size_of_usize)).unwrap();

    // Get the result of the execution
    let ret: usize = from_slice(next_journal_chunk(size_of_usize)).unwrap();
    println!("Result: {:?}", ret != 0);
    println!("============================================");

    /*Final receipt structure:
        checker_id: Vec<u32>
            Coitains a list of u32 values that represent the IMAGE_ID of the
            checkers that were used to prove each execution and should be used
            to verify them in the future.
        sdc_id: Vec<u32>
            Contains a list of u32 values that represent the IMAGE_ID of the SDC
            that was used to prove that the dependence among the decomposed
            proof file checks and should be used to verify it in the future.
        receipts: Vec<Receipt>
            Contains a list of Receipts that represent the proofs of execution
            of the checkers.
            Is composed by the following info:
                - Cycles: usize
                    Total of Cycles spent in to check the proof
                - Gamma length: usize
                    Length of the gamma input
                - Gamma: [u8]
                    The gamma proof instructions input
                - Claims: [u8]
                    The claims proof instructions input
        sdc_receipt: Receipt
            Contains a Receipt that represents the proof of the dependence among
            the decomposed proof file checks.
            Is composed by the following info:
                - Cycles: usize
                    Total of Cycles spent in to check the proof
                - Result: usize
                    If the all the sub-proofs depencencies were satisfied or not
                - Gamma length: usize
                    Length of the gamma input
                - Gamma: [u8]
                    The gamma proof instructions input
                - Claim length: usize
                    Length of the claims input
                - Claims: [u8]
                    The claims proof instructions input
                - Obligations_encoded: [u8]
                    The obligations proof instructions input
                    The oblications is a Vec<(Vec<u8>, Vec<u8>)> that contains
                    the gamma and claims of each sub-proof.
    */

    #[derive(Serialize, Deserialize)]
    struct Pi2Receipts {
        checker_id: Vec<u32>,
        sdc_id: Vec<u32>,
        receipts: Vec<Receipt>,
        sdc_receipt: Receipt,
    }

    let receipts = Pi2Receipts {
        checker_id: CHECKER_ID.to_vec(),
        sdc_id: SDC_ID.to_vec(),
        receipts: proof_checker_receipts,
        sdc_receipt: sdc_receipt,
    };

    let encoded: Vec<u8> = bincode::serialize(&receipts).unwrap();
    output_file
        .write_all(&encoded)
        .expect("Unable to write to file");

    println!(
        "Proof Checker and SDC receipts written to {}",
        output_filepath
    );
    println!(
        "Running and proving all proof-checker executions + ZK certficates generation {} ms",
        checker_provetime
    );
    println!(
        "Running and proving sdc execution + ZK certficate generation {} ms",
        now.elapsed().as_millis() - checker_provetime
    );
    println!(
        "Total time executing and proving took {} ms",
        now.elapsed().as_millis()
    )
}
