#include "lib.hpp"
#include <array>
#include <iostream>

typedef std::array<int, MAX_SIZE> assumption_type;
typedef std::array<int, MAX_SIZE> claim_type;
typedef std::array<int, MAX_SIZE> proof_type;

[[circuit]] int foo(assumption_type a, claim_type c, proof_type p) noexcept {
  return Pattern::verify(a, c, p);
}