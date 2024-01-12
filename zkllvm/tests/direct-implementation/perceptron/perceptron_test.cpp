[[circuit]] int perceptron(int x1, int x2, int x3, int x4, int x5) {

  // ret + w_i * x_i
  int ret = 0;
  ret = ret + (1 * x1);
  ret = ret + (2 * x2);
  ret = ret + (1 * x3);
  ret = ret + (3 * x4);
  ret = ret + (1 * x5);

  // ret - b
if (0 < ret - 3) {
  ret = ret - 3;
} else {
  ret = 0;
}

  return ret;
}
