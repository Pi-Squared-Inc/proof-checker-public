#![no_main]

use risc0_zkvm::guest::env;
risc0_zkvm::guest::entry!(main);

use std::io::Read;

extern crate checker;
use checker::verify;

pub fn main() {
    let mut _begin: usize = 0;
    let mut _end: usize = 0;

    // TODO: Add a custom profile for logging these instead of debug
    // that decreases real performance
    #[cfg(debug_assertions)] {
        _begin = env::get_cycle_count();
    }
    let gamma_buffer = &mut Vec::new();
    let _ = env::FdReader::new(10).read_to_end(gamma_buffer).unwrap();
    let claims_buffer = &mut Vec::new();
    let _ = env::FdReader::new(11).read_to_end(claims_buffer).unwrap();
    let proof_buffer = &mut Vec::new();
    let _ = env::stdin().read_to_end(proof_buffer).unwrap();
    #[cfg(debug_assertions)] {
        _end = env::get_cycle_count();

        // cycles spent reading input files
        env::log("I/O cycles");
        env::log(&(_end - _begin).to_string());
    }

    #[cfg(debug_assertions)] {
        _begin = env::get_cycle_count();
    }
    verify(gamma_buffer, claims_buffer, proof_buffer);
    #[cfg(debug_assertions)] {
        _end = env::get_cycle_count();

        // cycles spent verifying the theorem
        env::log("Cycles spent verifying the theorem...");
        env::log(&(_end - _begin).to_string());
    }

    // Certificate generation
    // Type: &[u8]
    /* Structure: [bytes_for_cycle_counts...,
        bytes_for_gamma_length...,
        bytes_for_gamma...,
        bytes_for_claims...]
        Keep in mind that each entry is serialized into streams of bytes
        (hence using...) that we need to deserialize into their respective
        datatypes: usize, usize, [Pattern], [Pattern]
        The first two are handled by RISC0 internal function,
        Gamma and claims are left as bytestreams for now (we could use the
        checker to deserialize them).
        We are sending gamma length so that we know where it ends.
    */

    // cycles spent throughout (we keep this for reference always)
    // we commit it because we do not need to convert to string
    env::commit(&env::get_cycle_count());

    // output gamma length
    env::commit(&gamma_buffer.len());
    // output gamma
    env::commit_slice(gamma_buffer.as_slice());

    // output claims
    env::commit_slice(claims_buffer.as_slice())
}
