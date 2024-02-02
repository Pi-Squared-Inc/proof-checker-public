#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
#![no_std] // std support is experimental

use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let amount: u64 = 10;
    let mut balance_from: u64 = 1000000;
    let mut balance_to: u64 = 200;
    let mut _ret: u64 = 0;

    let mut i = 0;
    while i < 5000 {
        if amount > balance_from {
            _ret = 0;
        } else {
            balance_from = balance_from - amount;
            balance_to = balance_to + amount;
            _ret = 1;
        }

        // commit the result to ZK journal
        env::commit(&_ret);
        i += 1;
    }

    env::commit(&env::get_cycle_count());
}
