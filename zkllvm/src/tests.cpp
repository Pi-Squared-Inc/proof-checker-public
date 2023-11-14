#include "../tests/unit_tests.hpp"
#include <array>
#include <iostream>

#ifndef DEBUG
typedef std::array<int, 1> assumption_type;
typedef std::array<int, 1> claim_type;
typedef std::array<int, 1> proof_type;

[[circuit]] int foo(assumption_type a, claim_type c, proof_type p) {

  int x = 1;
  int y = 2;
  test_efresh(x, y);
  test_sfresh(x, y);
  test_wellformedness_fresh();
  test_positivity();
  test_wellformedness_positive();
  test_instantiate();
  test_publish();
  test_construct_phi_implies_phi();
  test_phi_implies_phi_impl();
  test_universal_quantification();
  test_no_remaining_claims();

  return c[0];
}
#else
int main() {
  int x = 1;
  int y = 2;

  std::cout << "Executing test_efresh(" << x << ", " << y << ")" << std::endl;
  test_efresh(x, y);
  std::cout << std::endl;

  std::cout << "Executing test_sfresh(" << x << ", " << y << ")" << std::endl;
  test_sfresh(x, y);
  std::cout << std::endl;

  std::cout << "Executing test_wellformedness_fresh()" << std::endl;
  test_wellformedness_fresh();
  std::cout << std::endl;

  std::cout << "Executing test_positivity()" << std::endl;
  test_positivity();
  std::cout << std::endl;

  std::cout << "Executing test_wellformedness_positive()" << std::endl;
  test_wellformedness_positive();
  std::cout << std::endl;

  std::cout << "Executing test_instantiate()" << std::endl;
  test_instantiate();
  std::cout << std::endl;

  std::cout << "Executing test_publish()" << std::endl;
  test_publish();
  std::cout << std::endl;

  std::cout << "Executing test_construct_phi_implies_phi()" << std::endl;
  test_construct_phi_implies_phi();
  std::cout << std::endl;

  std::cout << "Executing test_phi_implies_phi_impl()" << std::endl;
  test_phi_implies_phi_impl();
  std::cout << std::endl;

  std::cout << "Executing test_universal_quantification()" << std::endl;
  test_universal_quantification();
  std::cout << std::endl;

  std::cout << "Executing test_no_remaining_claims()" << std::endl;
  test_no_remaining_claims();
  std::cout << std::endl;

  return 0;
}
#endif
