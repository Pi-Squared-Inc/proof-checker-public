#include "../tests/unit_tests.hpp"
#include <array>
#include <iostream>

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
