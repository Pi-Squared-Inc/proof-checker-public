#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
#![no_std]  // std support is experimental

risc0_zkvm::guest::entry!(main);

pub fn main() {
 
}
