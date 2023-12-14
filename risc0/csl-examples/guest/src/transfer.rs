#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
#![no_std] // std support is experimental

use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let amount: u64 = 10;
    let mut _balance_from: u64 = 1000000;
    let mut _balance_to: u64 = 200;
    let mut _ret: u64 = 0;

    if amount > _balance_from {
        _ret = 0;
    } else {
        _balance_from = _balance_from - amount;
        _balance_to = _balance_to + amount;
        _ret = 1;
    }

    // commit the result to ZK journal
    env::commit(&_ret);

    env::commit(&env::get_cycle_count());
}
