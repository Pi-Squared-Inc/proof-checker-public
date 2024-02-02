use core::array::ArrayTrait;
use debug::PrintTrait;
use pattern::Pattern;
use term::Term;
use stack::StackStructure;

mod pattern;
mod term;
mod stack;

mod test_impreflex;
mod test_transfer_goal;
mod test_batch_transfer_goal;
mod test_perceptron_goal;
mod test_svm_goal;

use test_impreflex::impreflex_goal;
use test_transfer_goal::transfer_goal;
use test_batch_transfer_goal::batch_transfer_goal;
use test_perceptron_goal::perceptron_goal;
use test_svm_goal::svm_goal;

use verifier::verify;

mod verifier;


// Main
fn main() {
    // 'Reading proofs ... '.print();
    // let gamma: Array<u8> = array![];
    // let claims: Array<u8> = array![];
    // let proofs: Array<u8> = array![];

    // 'Checking proofs ... '.print();
    // verify(gamma, claims, proofs);

    let (gamma, claims, proofs) = impreflex_goal();
    'Checking proofs ... '.print();
    verify(gamma, claims, proofs);

    let (gamma, claims, proofs) = transfer_goal();
    'Checking proofs ... '.print();
    verify(gamma, claims, proofs);

    let (gamma, claims, proofs) = batch_transfer_goal();
    'Checking proofs ... '.print();
    verify(gamma, claims, proofs);

    let (gamma, claims, proofs) = perceptron_goal();
    'Checking proofs ... '.print();
    verify(gamma, claims, proofs);

    let (gamma, claims, proofs) = svm_goal();
    'Checking proofs ... '.print();
    verify(gamma, claims, proofs);
}

