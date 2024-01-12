fn batch_transfer() -> u32 {
    let amount = 10_u32;
    let mut balance_from = 1000000_u32;
    let mut balance_to = 200_u32;
    let mut _ret = 0_u32;

    let mut i = 0_u32;
    loop {
        if i >= 5000 {
            break;
        } else {
            _ret = if amount > balance_from {
                0_u32
            } else {
                balance_from = balance_from - amount;
                balance_to = balance_to + amount;
                1_u32
            };
        }
        i = i + 1
    };
    _ret
}

fn main() -> u32 {
    return batch_transfer();
}

// Unit tests module
#[cfg(test)]
mod tests {
    use super::batch_transfer;

    #[test]
    #[available_gas(1000000000000000000)]
    fn test_batch_transfer() {
        assert(batch_transfer() == 1_u32, 'Err: batch_transfer() value!');
    }
}
