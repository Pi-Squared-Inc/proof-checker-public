// Modified from the transfer function at
//   https://docs.soliditylang.org/en/v0.8.17/contracts.html

int balanceSender = 100;
int amount = 10;
int balanceTo = 200;
int ret = 0;

[[circuit]]int transfer() {

  if (amount > balanceSender) {
    ret = 0; // transfer is not successful
  } else {
    balanceSender = balanceSender - amount;
    balanceTo = balanceTo + amount;
    ret = 1; // transfer is successful
  }

  return ret;
}
