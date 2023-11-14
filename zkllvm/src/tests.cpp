#include "../tests/unit_tests.hpp"
#include <array>
#include <iostream>

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
