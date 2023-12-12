// addressTo, amount, balanceSender, balanceTo, i
fn transfer5000() -> u32 {
    let x1 = 12345_u32;
    let x2 = 10_u32;
    let mut x3 = 100_u32;
    let mut x4 = 200_u32;

    let mut ret = 0_u32;
    let mut x5 = 0_u32;
    loop {
        if x5 >= 5000 {
            break;
        } else {
            ret = if x2 > x3 {
                0_u32
            } else {
                x3 = x3 - x2;
                x4 = x4 + x2;
                1_u32
            };
        }
        x5 = x5 + 1
    };
    ret
}

// Unit tests module
#[cfg(test)]
mod tests {
    use super::transfer5000;

    #[test]
    #[available_gas(1000000000000000000)]
    fn test_transfer5000() {
        assert(transfer5000() == 0_u32, 'Err: transfer5000() value!');
    }
}
