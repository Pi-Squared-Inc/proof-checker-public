#![deny(warnings)]
use checker::verify;
use std::fs;

pub fn main() {
    for _ in 0..100000 {
        let (gamma_reader, claims_reader, proof_reader) = match std::env::args().len() {
            3 => (
                fs::read(std::env::args().nth(1).unwrap()).unwrap(),
                fs::read("/dev/null").unwrap(),
                fs::read(std::env::args().nth(2).unwrap()).unwrap(),
            ),
            4 => (
                fs::read(std::env::args().nth(1).unwrap()).unwrap(),
                fs::read(std::env::args().nth(2).unwrap()).unwrap(),
                fs::read(std::env::args().nth(3).unwrap()).unwrap(),
            ),
            _ => panic!("Usage: checker gamma-file [claims-file] proof-file"),
        };

        verify(&gamma_reader, &claims_reader, &proof_reader);
    }
}
