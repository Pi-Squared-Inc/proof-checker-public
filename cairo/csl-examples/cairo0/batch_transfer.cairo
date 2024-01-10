%builtins range_check

from starkware.cairo.common.math_cmp import is_le

func transfer{range_check_ptr: felt}(balanceFrom: felt, balanceTo: felt, amount: felt, times: felt) -> (res: felt) {
    alloc_locals;
    if (is_le(1, times) != 0) {
        if (is_le(amount, balanceFrom) != 0) {
            local newBalanceFrom = balanceFrom - amount;
            local newBalanceTo = balanceTo + amount;
            return transfer(newBalanceFrom, newBalanceTo, amount, times-1);
        } else {
            return (res=0);
        }
    } else {
        return (res=1);
    }
}

func main{range_check_ptr: felt}() {
    let (result) = transfer(1000000, 200, 10, 5000);
    assert result = 1;
    return ();
}
