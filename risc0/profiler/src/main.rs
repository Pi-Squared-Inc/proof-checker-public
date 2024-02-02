use zk_host::methods::GUEST_ELF;

use risc0_zkvm::{default_executor, ExecutorEnv, Profiler};

use std::fs::File;
use std::io::BufReader;

fn main() {
    println!("Setting up env...");

    if std::env::args().len() != 4 {
        panic!("Expected 4 arguments. Received {}.", std::env::args().len())
    }

    let gamma_filepath = std::env::args().nth(1).expect("No gamma file path given");
    let gamma_file = File::open(gamma_filepath).expect("The gamma file was not found");
    let gamma_reader = BufReader::new(gamma_file);

    let claims_filepath = std::env::args().nth(2).expect("No claim file path given");
    let claims_file = File::open(claims_filepath).expect("The claims file was not found");
    let claims_reader = BufReader::new(claims_file);

    let proof_filepath = std::env::args().nth(3).expect("No proof file path given");
    let proof_file = File::open(proof_filepath).expect("The proof file was not found");
    let proof_reader = BufReader::new(proof_file);

    let mut profiler = Profiler::new("profile", GUEST_ELF).unwrap();

    // First, we construct an executor environment
    let env = {
        let mut builder = ExecutorEnv::builder();
        builder.trace_callback(profiler.make_trace_callback());
        builder
            .read_fd(10, gamma_reader)
            .read_fd(11, claims_reader)
            .stdin(proof_reader)
            .build()
            .unwrap()
    };

    // Next, we make an executor, loading the (renamed) ELF binary.
    let exec = default_executor();
    exec.execute_elf(env, GUEST_ELF).unwrap();

    println!("Outputting the profile...");
    profiler.finalize();
    let report = profiler.encode_to_vec();
    std::fs::write("profile", &report).unwrap();
}
