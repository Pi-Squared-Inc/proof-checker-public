fn svm5() -> u32 {
    let x1 = 1_u32;
    let x2 = 2_u32;
    let x3 = 3_u32;
    let x4 = 4_u32;
    let x5 = 5_u32;

    // ret + w_i * x_i
    let mut ret = 0_32;
    ret = ret + (1 * x1);
    ret = ret + (2 * x2);
    ret = ret + (1 * x3);
    ret = ret + (3 * x4);
    ret = ret + (1 * x5);

    // ret - b
    let ret = if 0 < ret - 3 {
        1_u32
    } else {
        4_294_967_295_u32 // max u32 value used to represent -1
    };
    ret
}

// Unit tests module
#[cfg(test)]
mod tests {
    use super::svm5;

    #[test]
    #[available_gas(100000)]
    fn test_svm5() {
        assert(svm5() == 1_u32, 'Err: svm5() value!');
    }
}
