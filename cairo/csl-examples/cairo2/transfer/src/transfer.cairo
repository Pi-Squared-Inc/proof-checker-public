// addressTo, amount, balanceSender, balanceTo, ret
fn transfer() -> u32 {
    let x1 = 12345_u32;
    let x2 = 10_u32;
    let mut x3 = 100_u32;
    let mut x4 = 200_u32;

    let ret = if x2 > x3 {
        0_u32
    } else {
        x3 = x3 - x2;
        x4 = x4 + x2;
        1_u32
    };
    ret
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
