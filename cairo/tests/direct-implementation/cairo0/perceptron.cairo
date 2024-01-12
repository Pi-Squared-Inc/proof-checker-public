%builtins range_check

from starkware.cairo.common.math_cmp import is_le

func perceptron{range_check_ptr: felt}() -> (z: felt) {
    alloc_locals;
    let x1 = 1;
    let x2 = 2;
    let x3 = 3;
    let x4 = 4;
    let x5 = 5;

    // res + w_i * x_i
    let res = 0;
    local res1 = res + (1 * x1);
    local res2 = res1 + (2 * x2);
    local res3 = res2 + (1 * x3);
    local res4 = res3 + (3 * x4);
    local res5 = res4 + (1 * x5);

    // res - b

    if (is_le(0, res5 - 3) != 0) {
        return (z=res5 - 3);
    } else {
        return (z=0);
    }
}

func main{range_check_ptr: felt}() {
    let (result) = perceptron();
    assert result = 22;
    return ();
}
