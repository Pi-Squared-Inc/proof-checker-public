%builtins range_check

from starkware.cairo.common.math_cmp import is_le

func transfer{range_check_ptr: felt}() -> (z: felt) {
    alloc_locals;
    let amount = 10;
    let balance_sender = 1000000;
    let balance_to = 200;
    
    if (is_le(amount, balance_sender) != 0 ) {
        local balance_sender_new = balance_sender - amount;
        local balance_to_new = balance_to + amount;
        return (z=1);
    } else {
        return (z=0);
    }
}

func main{range_check_ptr: felt}() {
   let (result) = transfer();
   assert result = 1;
   return ();
}
