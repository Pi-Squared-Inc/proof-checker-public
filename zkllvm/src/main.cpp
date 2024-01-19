#include "lib.hpp"
#include <array>
#include <iostream>

typedef std::array<int, ALEN> assumption_type;
typedef std::array<int, CLEN> claim_type;
typedef std::array<int, PLEN> proof_type;

void pad_assumption(assumption_type &a, std::array<int, MAXLEN> &pa) noexcept {
  if (ALEN > MAXLEN) {
    exit(1);
  }

  for (int i = 0; i < ALEN; i++) {
    pa[i] = a[i];
  }

  for (int i = ALEN; i < MAXLEN; i++) {
    pa[i] = 138;
  }
}

void pad_claim(claim_type &c, std::array<int, MAXLEN> &pc) noexcept {
  if (CLEN > MAXLEN) {
    exit(1);
  }

  for (int i = 0; i < CLEN; i++) {
    pc[i] = c[i];
  }

  for (int i = CLEN; i < MAXLEN; i++) {
    pc[i] = 138;
  }
}

void pad_proof(proof_type &p, std::array<int, MAXLEN> &pp) noexcept {
  if (PLEN > MAXLEN) {
    exit(1);
  }

  for (int i = 0; i < PLEN; i++) {
    pp[i] = p[i];
  }

  for (int i = PLEN; i < MAXLEN; i++) {
    pp[i] = 138;
  }
}

[[circuit]] int foo(assumption_type a, claim_type c, proof_type p) noexcept {
  std::array<int, MAXLEN> pa;
  std::array<int, MAXLEN> pc;
  std::array<int, MAXLEN> pp;

  pad_assumption(a, pa);
  pad_claim(c, pc);
  pad_proof(p, pp);

  return Pattern::verify(pa, pc, pp);
}
