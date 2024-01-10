fn transfer() -> u32 {
    let amount = 10_u32;
    let mut balance_from = 100_u32;
    let mut balance_to = 200_u32;

    let _ret = if amount > balance_from {
        0_u32
    } else {
        balance_from = balance_from - amount;
        balance_to = balance_to + amount;
        1_u32
    };
    _ret
}

fn main() -> u32 {
    transfer()
}

// Unit tests module
#[cfg(test)]
mod tests {
    use super::transfer;

    #[test]
    #[available_gas(100000)]
    fn test_transfer() {
        assert(transfer() == 1_u32, 'Err: transfer() value!');
    }
}
