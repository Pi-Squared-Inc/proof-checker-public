// Modified from the transfer function at
//   https://docs.soliditylang.org/en/v0.8.17/contracts.html

int balanceSender = 12345;
int balanceTo = 200;
int amount = 10;
int ret = 0;

[[circuit]]int transfer(int addressTo) {

  int i = 0;
  while (i < 5000) {
    if (amount > balanceSender) {
      ret = 0; // transfer is not successful
    } else {
      balanceSender = balanceSender - amount;
      balanceTo = balanceTo + amount;
      ret = 1; // transfer is successful
    }
    i = i + 1;
  }

  return 0;
}