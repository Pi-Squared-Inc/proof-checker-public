use debug::PrintTrait;

use verifier::verify;

mod verifier;

// Main
fn main() {
    'Reading proofs ... '.print();
    let gamma = ArrayTrait::<u8>::new();
    let claims = ArrayTrait::<u8>::new();
    let proofs = ArrayTrait::<u8>::new();
    'Checking proofs ... '.print();
    verify(gamma, claims, proofs);
}

