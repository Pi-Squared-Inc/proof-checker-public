#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
#![no_std]  // std support is experimental


use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

extern crate alloc;
#[allow(unused_imports)]
use alloc::string::{String, ToString};

pub fn main() {
    let x1: u64 = 1;
    let x2: u64 = 2;
    let x3: u64 = 3;
    let x4: u64 = 4;
    let x5: u64 = 5;
    #[allow(unused_assignments)]
    let mut ret: u64 = 0;

   // ret + w_i * x_i
    ret = 0;
    ret = ret + (1 * x1);
    ret = ret + (2 * x2);
    ret = ret + (1 * x3);
    ret = ret + (3 * x4);
    ret = ret + (1 * x5);

    // ret - b
    if 3 < ret {
        ret = ret - 3;
    } else {
        ret = 0;
    }

    // commit the result to ZK journal
    env::commit(&ret);

    env::commit(&env::get_cycle_count());
}
