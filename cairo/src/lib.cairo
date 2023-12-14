use debug::PrintTrait;
use pattern::Pattern;
use term::Term;
use stack::Stack;

mod pattern;
mod term;
mod stack;

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

