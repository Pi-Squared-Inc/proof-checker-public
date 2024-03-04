use risc0_zkvm::{serde::from_slice, Receipt};

use std::fs;
use std::time::Instant;

use serde::{Deserialize, Serialize};

fn verify_checker(
    obligations: Vec<(Vec<u8>, Vec<u8>)>,
    receipts: Vec<Receipt>,
    checker_id: [u32; 8],
) {
    let mut sum_cycles: usize = 0;
    let mut sum_verification_time: u128 = 0;

    // For loop to check inputs and verify checker_id
    let mut receipt_iter = receipts.iter();
    for (gamma, claims) in obligations {
        let now = Instant::now();
        let receipt_checker = receipt_iter.next().unwrap();

        // Small fetcher that returns the next chunk of given size from journal
        let mut current_index: usize = 0;
        let mut next_journal_chunk = |size: usize| -> &[u8] {
            let ret = &receipt_checker.journal.bytes[current_index..current_index + size];
            current_index += size;
            return ret;
        };

        // Get the host's size of a usize pointer
        let size_of_usize = std::mem::size_of::<usize>();
        let total_cycles: usize = from_slice(next_journal_chunk(size_of_usize)).unwrap();

        let gamma_length: usize = from_slice(next_journal_chunk(size_of_usize)).unwrap();
        let gamma_from_receipt = next_journal_chunk(gamma_length);
        let claims_from_receipt = &receipt_checker.journal.bytes[current_index..];

        assert!(gamma == gamma_from_receipt);
        assert!(claims == claims_from_receipt);

        receipt_checker.verify(checker_id).unwrap();

        let verification_time = now.elapsed().as_millis();
        println!("Total cycles {}", total_cycles);
        println!("There exists a proof of {:?} |- {:?}.", gamma, claims);
        println!("Verified in {} ms", verification_time);
        println!("============================================");

        sum_cycles += total_cycles;
        sum_verification_time += verification_time;
    }

    println!("Sum of Total cycles {}", sum_cycles);
    println!("Sum of Verification time {} ms", sum_verification_time);
}

fn verify_sdc(
    gamma_input: Vec<u8>,
    claims_input: Vec<u8>,
    receipt: &Receipt,
    sdc_id: [u32; 8],
) -> Vec<(Vec<u8>, Vec<u8>)> {
    let now = Instant::now();

    // Small fetcher that returns the next chunk of given size from journal
    let mut current_index: usize = 0;
    let mut next_journal_chunk = |size: usize| -> &[u8] {
        let ret = &receipt.journal.bytes[current_index..current_index + size];
        current_index += size;
        return ret;
    };

    // Get the host's size of a usize pointer
    let size_of_usize = std::mem::size_of::<usize>();
    let total_cycles: usize = from_slice(next_journal_chunk(size_of_usize)).unwrap();
    let sdc_result: usize = from_slice(next_journal_chunk(size_of_usize)).unwrap();

    let gamma_length: usize = from_slice(next_journal_chunk(size_of_usize)).unwrap();
    let gamma_from_receipt = next_journal_chunk(gamma_length);

    let claims_length: usize = from_slice(next_journal_chunk(size_of_usize)).unwrap();
    let claims_from_receipt = next_journal_chunk(claims_length);

    let obligations_encoded = &receipt.journal.bytes[current_index..];
    let obligations: Vec<(Vec<u8>, Vec<u8>)> = bincode::deserialize(obligations_encoded).unwrap();

    assert!(sdc_result != 0);
    assert!(gamma_input == gamma_from_receipt);
    assert!(claims_input == claims_from_receipt);

    receipt.verify(sdc_id).unwrap();

    println!("Total cycles {}", total_cycles);
    println!(
        "There exists a proof of {:?} |- {:?}.",
        gamma_input, claims_input
    );
    println!("Verified in {} ms", now.elapsed().as_millis());
    println!("============================================");

    return obligations;
}

fn main() {
    let now = Instant::now();

    println!("Setting up env...");

    if std::env::args().len() != 4 {
        panic!("Expected 4 arguments. Received {}.", std::env::args().len())
    }

    let gamma_filepath = std::env::args().nth(1).expect("No gamma file path given");
    let gamma_input = fs::read(gamma_filepath).expect("The gamma file was not found");

    let claims_filepath = std::env::args().nth(2).expect("No claim file path given");
    let claims_input = fs::read(claims_filepath).expect("The claims file was not found");

    let receipt_filepath = std::env::args().nth(3).expect("No receipt file path given");
    let receipt_input = fs::read(receipt_filepath).expect("The receipt file was not found");

    #[derive(Serialize, Deserialize)]
    struct Pi2Receipts {
        checker_id: Vec<u32>,
        sdc_id: Vec<u32>,
        receipts: Vec<Receipt>,
        sdc_receipt: Receipt,
    }

    let pi2_receipts: Pi2Receipts = bincode::deserialize(&receipt_input).unwrap();
    let mut checker_id: [u32; 8] = [0; 8];
    for i in 0..8 {
        checker_id[i] = pi2_receipts.checker_id[i];
    }

    let mut sdc_id: [u32; 8] = [0; 8];
    for i in 0..8 {
        sdc_id[i] = pi2_receipts.sdc_id[i];
    }

    println!("Verifying the SDC proof and generating the receipt...");
    let obligations = verify_sdc(gamma_input, claims_input, &pi2_receipts.sdc_receipt, sdc_id);

    println!("Verifying the proof-checker proof and generating the receipt...");
    verify_checker(obligations, pi2_receipts.receipts, checker_id);

    println!(
        "SDC and Proof Checker was verified in {} ms",
        now.elapsed().as_millis()
    );
}
