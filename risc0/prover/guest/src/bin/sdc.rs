#![no_main]

use risc0_zkvm::guest::env;
risc0_zkvm::guest::entry!(main);

use std::io::Read;
use bincode;

extern crate sdc;
use sdc::sdc;

pub fn main() {
    let mut _begin: usize = 0;
    let mut _end: usize = 0;

    // TODO: Add a custom profile for logging these instead of debug
    // that decreases real performance
    #[cfg(debug_assertions)] {
        _begin = env::get_cycle_count();
    }
    let gamma_target = &mut Vec::new();
    let _ = env::FdReader::new(12).read_to_end(gamma_target).unwrap();
    let gamma_target_versioned = gamma_target.clone();
    gamma_target.drain(0..3);

    let claims_target = &mut Vec::new();
    let _ = env::FdReader::new(13).read_to_end(claims_target).unwrap();
    let claims_target_versioned = claims_target.clone();
    claims_target.drain(0..3);

    let obligations_versioned = env::read::<Vec<(Vec<u8>, Vec<u8>)>>();
    let mut obligations = obligations_versioned.clone();
    for i in 0..obligations.len() {
        obligations[i].0.drain(0..3);
        obligations[i].1.drain(0..3);
    }

    #[cfg(debug_assertions)] {
        _end = env::get_cycle_count();

        // cycles spent reading input files
        env::log("I/O cycles");
        env::log(&(_end - _begin).to_string());
    }
    #[cfg(debug_assertions)]
    {
        _begin = env::get_cycle_count();
    }

    let result = sdc(&gamma_target, &claims_target, &obligations);

    #[cfg(debug_assertions)]
    {
        _end = env::get_cycle_count();

        // cycles spent verifying the theorem
        env::log("Cycles spent verifying the theorem...");
        env::log(&(_end - _begin).to_string());
    }

    // cycles spent throughout (we keep this for reference always)
    // we commit it because we do not need to convert to string
    env::commit(&env::get_cycle_count());

    let result_to_commit = result as usize;
    env::commit(&result_to_commit);

    env::commit(&gamma_target_versioned.len());
    env::commit_slice(gamma_target_versioned.as_slice());

    env::commit(&claims_target_versioned.len());
    env::commit_slice(claims_target_versioned.as_slice());

    let obligations_encoded = bincode::serialize(&obligations_versioned).unwrap(); 
    env::commit_slice(obligations_encoded.as_slice());
}
