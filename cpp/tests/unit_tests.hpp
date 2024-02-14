#include "../src/lib.hpp"

#define MAX_SIZE 27001

void test_pattern_construction() {
  std::array<int, MAX_SIZE> proof;
  proof[0] = 6; // Size
  proof[1] = (int)Instruction::SVar;
  proof[2] = 0;
  proof[3] = 137; // Instruction::CleanMetaVar;
  proof[4] = 0;
  proof[5] = (int)Instruction::ESubst;
  proof[6] = 0;
  proof[7] = 138; // Instruction::NO_OP;

  auto stack = Pattern::Stack();
  auto memory = Pattern::Memory();
  auto claims = Pattern::Claims();

  Pattern::execute_instructions(proof, stack, memory, claims, Pattern::ExecutionPhase::Gamma);

  assert(stack.size() == 1);
  auto expected_pattern = Pattern::Term::Pattern_(
      Pattern::esubst(Pattern::metavar_unconstrained(0), 0, Pattern::svar(0)));
  assert(stack[0] == expected_pattern);
}

void test_efresh(int a, int b) {

  auto evar = Pattern::evar(a);

  auto left = Pattern::exists(a, evar.clone());
  assert(left->pattern_e_fresh(a));

  auto right = Pattern::exists(b, evar.clone());
  assert(!right->pattern_e_fresh(a));

  auto implication = Pattern::implies(left.clone(), right.clone());
  assert(!implication->pattern_e_fresh(a));

  auto mvar = Pattern::metavar_s_fresh(a, b, IdList(b), IdList(b));
  auto metaapp = Pattern::app(left.clone(), mvar.clone());
  assert(!metaapp->pattern_e_fresh(b));

  auto esubst = Pattern::esubst(right.clone(), a, left.clone());
  assert(esubst->pattern_e_fresh(a));

  auto ssubst = Pattern::ssubst(right.clone(), a, left.clone());
  assert(!ssubst->pattern_e_fresh(a));

#if DEBUG
  evar->print();
  std::cout << std::endl;
  left->print();
  std::cout << std::endl;
  right->print();
  std::cout << std::endl;
  implication->print();
  std::cout << std::endl;
  mvar->print();
  std::cout << std::endl;
  metaapp->print();
  std::cout << std::endl;
  esubst->print();
  std::cout << std::endl;
  ssubst->print();
  std::cout << std::endl;

#endif

}

void test_sfresh(int a, int b) {

  auto svar = Pattern::svar(a);

  auto left = Pattern::mu(a, svar.clone());
  assert(left->pattern_s_fresh(a));

  auto right = Pattern::mu(b, svar.clone());
  assert(!right->pattern_s_fresh(a));

  auto implication = Pattern::implies(left.clone(), right.clone());
  assert(!implication->pattern_s_fresh(a));

  auto mvar = Pattern::metavar_s_fresh(a, b, IdList(b), IdList(b));

  auto metaapp = Pattern::app(left.clone(), mvar.clone());
  assert(!metaapp->pattern_s_fresh(a));

  auto metaapp2 = Pattern::app(left.clone(), mvar.clone());
  assert(metaapp2->pattern_s_fresh(b));

  auto esubst = Pattern::esubst(right.clone(), a, left.clone());
  assert(!esubst->pattern_s_fresh(a));

  auto ssubst = Pattern::ssubst(right.clone(), a, left.clone());
  assert(ssubst->pattern_s_fresh(a));

#if DEBUG
  svar->print();
  std::cout << std::endl;
  left->print();
  std::cout << std::endl;
  right->print();
  std::cout << std::endl;
  implication->print();
  std::cout << std::endl;
  mvar->print();
  std::cout << std::endl;
  metaapp->print();
  std::cout << std::endl;
  metaapp2->print();
  std::cout << std::endl;
  esubst->print();
  std::cout << std::endl;
  ssubst->print();
  std::cout << std::endl;
#endif

}

void test_wellformedness_fresh() {
  auto phi0_s_fresh_0 = Pattern::metavar_s_fresh(0, 0, IdList(0), IdList(0));
  assert(phi0_s_fresh_0->pattern_well_formed());

  auto phi1_e_fresh = IdList();
  phi1_e_fresh.push_back(1);
  phi1_e_fresh.push_back(2);
  phi1_e_fresh.push_back(0);
  auto phi1 =
      Pattern::metavar(1, phi1_e_fresh, IdList(), IdList(),
          IdList(), IdList(2));

  assert(!phi1->pattern_well_formed());

}

void test_wellformedness_esubst_ssubst() {
  auto phi0_x1_s1 = Pattern::esubst(
      Pattern::metavar_unconstrained(0), 1, Pattern::symbol(1));
  assert(phi0_x1_s1->pattern_well_formed());

  auto s0_x1_s1 = Pattern::esubst(
      Pattern::symbol(0), 1, Pattern::symbol(1));
  assert(!s0_x1_s1->pattern_well_formed());

  auto phi0_x1_x1 = Pattern::esubst(
      Pattern::metavar_unconstrained(0), 1, Pattern::evar(1));
  assert(!phi0_x1_x1->pattern_well_formed());

  auto phi0_fresh_x1_s1 = Pattern::esubst(
      Pattern::metavar_e_fresh(0, 1, IdList(), IdList()), 1, Pattern::symbol(1));
  assert(!phi0_fresh_x1_s1->pattern_well_formed());

  auto phi0_X1_s1 = Pattern::ssubst(
      Pattern::metavar_unconstrained(0), 1, Pattern::symbol(1));
  assert(phi0_X1_s1->pattern_well_formed());

  auto phi0_X1_x1 = Pattern::ssubst(
      Pattern::metavar_unconstrained(0), 1, Pattern::svar(1));
  assert(!phi0_X1_x1->pattern_well_formed());

  auto s0_X1_s1 = Pattern::ssubst(
      Pattern::symbol(0), 1, Pattern::symbol(1));
  assert(!s0_X1_s1->pattern_well_formed());

  auto phi0_fresh_X1_s1 = Pattern::ssubst(
      Pattern::metavar_s_fresh(0, 1, IdList(), IdList()), 1, Pattern::symbol(1));
  assert(!phi0_fresh_X1_s1->pattern_well_formed());
}

void test_positivity() {

  auto X0 = Pattern::svar(0);
  auto X1 = Pattern::svar(1);
  auto X2 = Pattern::svar(2);
  auto c1 = Pattern::symbol(1);
  auto neg_X1 = Pattern::negate(X1.clone());

  // EVar
  auto evar1 = Pattern::evar(1);
  assert(evar1->pattern_positive(1));
  assert(evar1->pattern_negative(1));
  assert(evar1->pattern_positive(2));
  assert(evar1->pattern_negative(2));

  // SVar
  assert(X1->pattern_positive(1));
  assert(!X1->pattern_negative(1));
  assert(X1->pattern_positive(2));
  assert(X1->pattern_negative(2));

  // Symbol
  assert(c1->pattern_positive(1));
  assert(c1->pattern_negative(1));
  assert(c1->pattern_positive(2));
  assert(c1->pattern_negative(2));

  // Application
  auto appX1X2 = Pattern::app(X1.clone(), X2.clone());
  assert(appX1X2->pattern_positive(1));
  assert(appX1X2->pattern_positive(2));
  assert(appX1X2->pattern_positive(3));
  assert(!appX1X2->pattern_negative(1));
  assert(!appX1X2->pattern_negative(2));
  assert(appX1X2->pattern_negative(3));

  // Implication
  auto impliesX1X2 = Pattern::implies(X1.clone(), X2.clone());
  assert(!impliesX1X2->pattern_positive(1));
  assert(impliesX1X2->pattern_positive(2));
  assert(impliesX1X2->pattern_positive(3));
  assert(impliesX1X2->pattern_negative(1));
  assert(!impliesX1X2->pattern_negative(2));
  assert(impliesX1X2->pattern_negative(3));

  auto impliesX1X1 = Pattern::implies(X1.clone(), X1.clone());
  assert(!impliesX1X1->pattern_positive(1));
  assert(!impliesX1X1->pattern_negative(1));

  // Exists
  auto existsX1X2 = Pattern::exists(1, X2.clone());
  assert(existsX1X2->pattern_positive(1));
  assert(existsX1X2->pattern_positive(2));
  assert(existsX1X2->pattern_positive(3));
  assert(existsX1X2->pattern_negative(1));
  assert(!existsX1X2->pattern_negative(2));
  assert(existsX1X2->pattern_negative(3));

  // Mu
  auto muX1x1 = Pattern::mu(1, evar1.clone());
  assert(muX1x1->pattern_positive(1));
  assert(muX1x1->pattern_positive(2));
  assert(muX1x1->pattern_negative(1));
  assert(muX1x1->pattern_negative(2));

  auto muX1X1 = Pattern::mu(1, X1.clone());
  assert(muX1X1->pattern_positive(1));
  assert(muX1X1->pattern_negative(1));

  auto muX1X2 = Pattern::mu(1, X2.clone());
  auto muX1impliesX2X1 =
      Pattern::mu(1, Pattern::implies(X2.clone(), X1.clone()));
  assert(muX1X2->pattern_positive(1));
  assert(muX1X2->pattern_positive(2));
  assert(muX1X2->pattern_positive(3));
  assert(muX1X2->pattern_negative(1));
  assert(!muX1X2->pattern_negative(2));
  assert(muX1impliesX2X1->pattern_negative(2));
  assert(muX1X2->pattern_negative(3));

  // MetaVar
  auto metavarUncons1 = Pattern::metavar_unconstrained(1);
  assert(!metavarUncons1->pattern_positive(1));
  assert(!metavarUncons1->pattern_positive(2));
  assert(!metavarUncons1->pattern_negative(1));
  assert(!metavarUncons1->pattern_negative(2));

  // Do not imply positivity from freshness
  auto pos = IdList();
  auto neg = IdList();
  auto metavarSFresh11__ = Pattern::metavar_s_fresh(1, 1, IdList(), IdList());
  auto metavarSFresh1111 = Pattern::metavar_s_fresh(1, 1, IdList(1), IdList(1));
  auto metavarSFresh111_ =  Pattern::metavar_s_fresh(1, 1, IdList(1), IdList());
  auto metavarSFresh11_1 = Pattern::metavar_s_fresh(1, 1, IdList(), IdList(1));

  assert(metavarSFresh11__->pattern_positive(1));
  assert(metavarSFresh11__->pattern_negative(1));
  assert(metavarSFresh1111->pattern_positive(1));
  assert(metavarSFresh1111->pattern_negative(1));
  assert(metavarSFresh111_->pattern_positive(1));
  assert(metavarSFresh111_->pattern_negative(1));
  assert(metavarSFresh11_1->pattern_positive(1));
  assert(metavarSFresh11_1->pattern_negative(1));

  assert(!metavarSFresh11__->pattern_positive(2));
  assert(!metavarSFresh11__->pattern_negative(2));

  // ESubst
  auto esubstMetaVarUnconsX0 =
      Pattern::esubst(Pattern::metavar_unconstrained(0), 0, X0.clone());
  auto esubstMetaVarSFreshX1 = Pattern::esubst(
      Pattern::metavar_s_fresh(0, 1, IdList(1), IdList()), 0, X1.clone());
  auto esubstMetaVarUnconsX1 =
      Pattern::esubst(Pattern::metavar_unconstrained(0), 0, X1.clone());

  assert(!esubstMetaVarUnconsX0->pattern_positive(0));
  assert(!esubstMetaVarUnconsX1->pattern_positive(0));
  assert(!esubstMetaVarSFreshX1->pattern_positive(0));
  assert(!esubstMetaVarUnconsX0->pattern_negative(0));
  assert(!esubstMetaVarUnconsX1->pattern_negative(0));
  assert(!esubstMetaVarUnconsX1->pattern_negative(0));

  // SSubst
  auto ssubstMetaVarUnconsX0 =
      Pattern::ssubst(Pattern::metavar_unconstrained(0), 0, X0.clone());
  auto ssubstMetaVarUnconsX1 =
      Pattern::ssubst(Pattern::metavar_unconstrained(0), 0, X1.clone());
  auto ssubstMetaVarSFreshX1 = Pattern::ssubst(
      Pattern::metavar_s_fresh(0, 1, IdList(1), IdList()), 0, X1.clone());

  assert(!ssubstMetaVarUnconsX0->pattern_positive(0));
  assert(ssubstMetaVarUnconsX1->pattern_positive(0));
  assert(ssubstMetaVarSFreshX1->pattern_positive(0));

  assert(!ssubstMetaVarUnconsX0->pattern_negative(0));
  assert(ssubstMetaVarUnconsX1->pattern_negative(0));
  assert(ssubstMetaVarSFreshX1->pattern_negative(0));

  // Combinations
  assert(!neg_X1->pattern_positive(1));
  assert(neg_X1->pattern_positive(2));
  assert(neg_X1->pattern_negative(1));
  assert(neg_X1->pattern_negative(2));

  auto negX1_implies_negX1 = Pattern::implies(neg_X1.clone(), neg_X1.clone());
  assert(!negX1_implies_negX1->pattern_positive(1));
  assert(negX1_implies_negX1->pattern_positive(2));
  assert(!negX1_implies_negX1->pattern_negative(1));
  assert(negX1_implies_negX1->pattern_negative(2));

  auto negX1_implies_X1 = Pattern::implies(neg_X1.clone(), X1.clone());
  assert(negX1_implies_X1->pattern_positive(1));
  assert(!negX1_implies_X1->pattern_negative(1));

#if DEBUG
  X0->print();
  std::cout << std::endl;
  X1->print();
  std::cout << std::endl;
  X2->print();
  std::cout << std::endl;
  c1->print();
  std::cout << std::endl;
  neg_X1->print();
  std::cout << std::endl;
  evar1->print();
  std::cout << std::endl;
  appX1X2->print();
  std::cout << std::endl;
  impliesX1X2->print();
  std::cout << std::endl;
  impliesX1X1->print();
  std::cout << std::endl;
  existsX1X2->print();
  std::cout << std::endl;
  muX1x1->print();
  std::cout << std::endl;
  muX1X1->print();
  std::cout << std::endl;
  muX1X2->print();
  std::cout << std::endl;
  muX1impliesX2X1->print();
  std::cout << std::endl;
  metavarUncons1->print();
  std::cout << std::endl;
  metavarSFresh11__->print();
  std::cout << std::endl;
  metavarSFresh1111->print();
  std::cout << std::endl;
  metavarSFresh111_->print();
  std::cout << std::endl;
  metavarSFresh11_1->print();
  std::cout << std::endl;
  esubstMetaVarUnconsX0->print();
  std::cout << std::endl;
  esubstMetaVarUnconsX1->print();
  std::cout << std::endl;
  esubstMetaVarSFreshX1->print();
  std::cout << std::endl;
  ssubstMetaVarUnconsX0->print();
  std::cout << std::endl;
  ssubstMetaVarUnconsX1->print();
  std::cout << std::endl;
  ssubstMetaVarSFreshX1->print();
  std::cout << std::endl;
  negX1_implies_negX1->print();
  std::cout << std::endl;
  negX1_implies_X1->print();
  std::cout << std::endl;

#endif

}

void test_app_ctx_hole() {

  assert(!Pattern::metavar_unconstrained(0)->pattern_app_ctx_hole(0));
  assert(Pattern::metavar(0, IdList(), IdList(), IdList(), IdList(), IdList(0))
          ->pattern_app_ctx_hole(0));
  assert(Pattern::evar(0)->pattern_app_ctx_hole(0));
  assert(!Pattern::evar(1)->pattern_app_ctx_hole(0));
  assert(!Pattern::svar(0)->pattern_app_ctx_hole(0));
  assert(!Pattern::symbol(0)->pattern_app_ctx_hole(0));
  assert(!Pattern::implies(Pattern::evar(0), Pattern::evar(1))
          ->pattern_app_ctx_hole(0));
  assert(Pattern::app(Pattern::evar(0), Pattern::evar(1))
          ->pattern_app_ctx_hole(0));
  assert(Pattern::app(Pattern::evar(1), Pattern::evar(0))
          ->pattern_app_ctx_hole(0));
  assert(!Pattern::app(Pattern::evar(0), Pattern::evar(0))
          ->pattern_app_ctx_hole(0));
  assert(!Pattern::app(Pattern::evar(1), Pattern::evar(1))
          ->pattern_app_ctx_hole(0));
  assert(!Pattern::exists(0, Pattern::evar(0))->pattern_app_ctx_hole(0));
  assert(!Pattern::exists(0, Pattern::evar(1))->pattern_app_ctx_hole(0));
  assert(!Pattern::exists(1, Pattern::evar(0))->pattern_app_ctx_hole(0));
  assert(!Pattern::exists(1, Pattern::evar(1))->pattern_app_ctx_hole(0));
  assert(!Pattern::mu(0, Pattern::evar(0))->pattern_app_ctx_hole(0));
  assert(!Pattern::mu(0, Pattern::svar(1))->pattern_app_ctx_hole(0));

}

void test_wellformedness_positive() {

  auto svar = Pattern::svar(1);
  auto mux_x = Pattern::mu(1, svar.clone());
  assert(mux_x->pattern_well_formed());

  auto mux_x2 = Pattern::mu(2, Pattern::negate(svar.clone()));
  assert(mux_x2->pattern_well_formed());

  auto mux_x3 = Pattern::mu(2, Pattern::negate(Pattern::symbol(1)));
  assert(mux_x3->pattern_well_formed());

  auto mux_x4 = Pattern::mu(1, Pattern::negate(svar.clone()));
  assert(!mux_x4->pattern_well_formed());

  auto phi = Pattern::metavar_s_fresh(97, 2, IdList(), IdList());
  auto mux_phi = Pattern::mu(1, phi.clone());
  assert(!mux_phi->pattern_well_formed());

  auto phi2 = Pattern::metavar_s_fresh(98, 1, IdList(), IdList());
  auto mux_phi2 = Pattern::mu(1, phi2.clone());
  assert(mux_phi2->pattern_well_formed());

  // It's ok if 2 is negative, the only thing we care about is that 2 is
  // guaranteed to be positive (we can instantiate without this variable)
  auto phi3 = Pattern::metavar_s_fresh(99, 1, IdList(2), IdList(2));
  auto mux_phi3 = Pattern::mu(2, phi3.clone());
  assert(mux_phi3->pattern_well_formed());

  auto phi4 = Pattern::metavar_s_fresh(100, 1, IdList(2), IdList());
  auto mux_phi4 = Pattern::mu(2, phi4.clone());
  assert(mux_phi4->pattern_well_formed());

#if DEBUG
  svar->print();
  std::cout << std::endl;
  mux_x->print();
  std::cout << std::endl;
  mux_x2->print();
  std::cout << std::endl;
  mux_x3->print();
  std::cout << std::endl;
  mux_x4->print();
  std::cout << std::endl;
  phi->print();
  std::cout << std::endl;
  mux_phi->print();
  std::cout << std::endl;
  phi2->print();
  std::cout << std::endl;
  mux_phi2->print();
  std::cout << std::endl;
  phi3->print();
  std::cout << std::endl;
  mux_phi3->print();
  std::cout << std::endl;
  phi4->print();
  std::cout << std::endl;
  mux_phi4->print();
  std::cout << std::endl;
#endif

}

void test_instantiate() {
  typedef LinkedList<Rc<Pattern>> Patterns;
  auto x0 = Pattern::evar(0);
  auto X0 = Pattern::svar(0);
  auto c0 = Pattern::symbol(0);
  auto x0_implies_x0 = Pattern::implies(x0.clone(), x0.clone());
  auto appx0x0 = Pattern::app(x0.clone(), x0.clone());
  auto existsx0x0 = Pattern::exists(0, x0.clone());
  auto muX0x0 = Pattern::mu(0, x0.clone());

  // Concrete patterns are unaffected by instantiate
  IdList vars0 = IdList();
  vars0.push_back(0);
  IdList vars1 = IdList();
  vars1.push_back(1);
  Patterns plugsX0 = Patterns();
  plugsX0.push_back(X0.clone());
  Patterns plugsx0 = Patterns();
  plugsx0.push_back(x0.clone());
  assert(!Pattern::instantiate_internal(x0, vars0, plugsX0));
  assert(!Pattern::instantiate_internal(x0, vars1, plugsX0));
  assert(!Pattern::instantiate_internal(X0, vars0, plugsx0));
  assert(!Pattern::instantiate_internal(X0, vars1, plugsx0));
  assert(!Pattern::instantiate_internal(c0, vars0, plugsx0));
  assert(!Pattern::instantiate_internal(c0, vars1, plugsx0));
  assert(!Pattern::instantiate_internal(x0_implies_x0, vars0, plugsx0));
  assert(!Pattern::instantiate_internal(x0_implies_x0, vars1, plugsx0));
  assert(!Pattern::instantiate_internal(appx0x0, vars0, plugsx0));
  assert(!Pattern::instantiate_internal(appx0x0, vars1, plugsx0));
  assert(!Pattern::instantiate_internal(existsx0x0, vars0, plugsX0));
  assert(!Pattern::instantiate_internal(existsx0x0, vars1, plugsX0));
  assert(!Pattern::instantiate_internal(muX0x0, vars0, plugsx0));
  assert(!Pattern::instantiate_internal(muX0x0, vars1, plugsx0));

  auto phi0 = Pattern::metavar_unconstrained(0);
  auto phi0_implies_phi0 = Pattern::implies(phi0.clone(), phi0.clone());
  auto appphi0phi0 = Pattern::app(phi0.clone(), phi0.clone());
  auto existsx0phi0 = Pattern::exists(0, phi0.clone());
  auto muX0phi0 = Pattern::mu(0, phi0.clone());
  auto existsx0X0 = Pattern::exists(0, X0.clone());

  auto internal0 =
      Pattern::instantiate_internal(phi0_implies_phi0, vars0, plugsx0);
  auto internal1 =
      Pattern::instantiate_internal(phi0_implies_phi0, vars1, plugsx0);
  auto internal2 = Pattern::instantiate_internal(appphi0phi0, vars0, plugsx0);
  auto internal3 = Pattern::instantiate_internal(appphi0phi0, vars1, plugsx0);
  auto internal4 = Pattern::instantiate_internal(existsx0phi0, vars0, plugsx0);
  auto internal5 = Pattern::instantiate_internal(existsx0phi0, vars1, plugsx0);
  auto internal6 = Pattern::instantiate_internal(muX0phi0, vars0, plugsx0);
  auto internal7 = Pattern::instantiate_internal(muX0phi0, vars1, plugsx0);

  assert(internal0.unwrap().operator==(x0_implies_x0));
  assert(!internal1);
  assert(internal2.unwrap().operator==(appx0x0));
  assert(!internal3);
  assert(internal4.unwrap().operator==(existsx0x0));
  assert(!internal5);
  assert(internal6.unwrap().operator==(muX0x0));
  assert(!internal7);

  // Simultaneous instantiations
  auto vars12 = IdList();
  vars12.push_back(1);
  vars12.push_back(2);
  auto vars21 = IdList();
  vars21.push_back(2);
  vars21.push_back(1);
  auto plugsx0X0 = Patterns();
  plugsx0X0.push_back(x0.clone());
  plugsx0X0.push_back(X0.clone());
  auto phi1 = Pattern::metavar_unconstrained(1);
  auto muX0phi1 = Pattern::mu(0, phi1.clone());
  auto muX0X0 = Pattern::mu(0, X0.clone());

  // Empty substs have no effect
  assert(!Pattern::instantiate_internal(existsx0phi0, vars12, plugsx0X0));
  assert(!Pattern::instantiate_internal(existsx0phi0, vars21, plugsx0X0));
  assert(!Pattern::instantiate_internal(muX0phi0, vars12, plugsx0X0));
  assert(!Pattern::instantiate_internal(muX0phi0, vars21, plugsx0X0));

  // Order matters if corresponding value is not moved
  auto vars10 = IdList();
  vars10.push_back(1);
  vars10.push_back(0);
  auto vars01 = IdList();
  vars01.push_back(0);
  vars01.push_back(1);
  auto internal8 =
      Pattern::instantiate_internal(existsx0phi0, vars10, plugsx0X0);
  auto internal9 =
      Pattern::instantiate_internal(existsx0phi0, vars01, plugsx0X0);
  auto internal10 = Pattern::instantiate_internal(muX0phi0, vars10, plugsx0X0);
  auto internal11 = Pattern::instantiate_internal(muX0phi0, vars01, plugsx0X0);

  assert(internal8.unwrap().operator==(existsx0X0));
  assert(internal9.unwrap().operator==(existsx0x0));
  assert(internal10.unwrap().operator==(muX0X0));
  assert(internal11.unwrap().operator==(muX0x0));

  // Order does not matter if corresponding value is moved
  auto plugsX0x0 = Patterns();
  plugsX0x0.push_back(X0.clone());
  plugsX0x0.push_back(x0.clone());
  auto muX0phi0_implies_ph1 = Pattern::implies(muX0phi0.clone(), phi1.clone());
  auto muX0x0_implies_X0 = Pattern::implies(muX0x0.clone(), X0.clone());
  auto internal12 = Pattern::instantiate_internal(muX0phi0_implies_ph1, vars01, plugsx0X0);
  auto internal13 = Pattern::instantiate_internal(muX0phi0_implies_ph1, vars10, plugsX0x0);

  assert(internal12.unwrap().operator==(muX0x0_implies_X0));
  assert(internal13.unwrap().operator==(muX0x0_implies_X0));

  auto muX0phi0_app_ph1 = Pattern::app(muX0phi0.clone(), phi1.clone());
  auto muX0x0_app_X0 = Pattern::app(muX0x0.clone(), X0.clone());
  auto internal14 = Pattern::instantiate_internal(muX0phi0_app_ph1, vars01, plugsx0X0);
  auto internal15 = Pattern::instantiate_internal(muX0phi0_app_ph1, vars10, plugsX0x0);

  assert(internal14.unwrap().operator==(muX0x0_app_X0));
  assert(internal15.unwrap().operator==(muX0x0_app_X0));

  // No side-effects
  auto plugsphi1X0 = Patterns();
  plugsphi1X0.push_back(phi1.clone());
  plugsphi1X0.push_back(X0.clone());
  auto plugsX0phi1 = Patterns();
  plugsX0phi1.push_back(X0.clone());
  plugsX0phi1.push_back(phi1.clone());
  auto muX0ph1_implies_X0 = Pattern::implies(muX0phi1.clone(), X0.clone());
  auto internal16 = Pattern::instantiate_internal(muX0phi0_implies_ph1, vars01, plugsphi1X0);
  auto internal17 = Pattern::instantiate_internal(muX0phi0_implies_ph1, vars10, plugsX0phi1);

  assert(internal16.unwrap().operator==(muX0ph1_implies_X0));
  assert(internal17.unwrap().operator==(muX0ph1_implies_X0));

  auto muX0ph1_app_X0 = Pattern::app(muX0phi1.clone(), X0.clone());
  auto internal18 = Pattern::instantiate_internal(muX0phi0_app_ph1, vars01, plugsphi1X0);
  auto internal19 = Pattern::instantiate_internal(muX0phi0_app_ph1, vars10, plugsX0phi1);

  assert(internal18.unwrap().operator==(muX0ph1_app_X0));
  assert(internal19.unwrap().operator==(muX0ph1_app_X0));

  // First comes first
  auto vars011 = IdList();
  vars011.push_back(0);
  vars011.push_back(1);
  vars011.push_back(1);
  auto vars100 = IdList();
  vars100.push_back(1);
  vars100.push_back(0);
  vars100.push_back(0);
  auto plugsphi1X0x0 = Patterns();
  plugsphi1X0x0.push_back(phi1.clone());
  plugsphi1X0x0.push_back(X0.clone());
  plugsphi1X0x0.push_back(x0.clone());
  auto plugsX0phi1x0 = Patterns();
  plugsX0phi1x0.push_back(X0.clone());
  plugsX0phi1x0.push_back(phi1.clone());
  plugsX0phi1x0.push_back(x0.clone());
  auto internal20 = Pattern::instantiate_internal(muX0phi0_app_ph1, vars011, plugsphi1X0x0);
  auto internal21 = Pattern::instantiate_internal(muX0phi0_app_ph1, vars100, plugsX0phi1x0);

  assert(internal20.unwrap().operator==(muX0ph1_app_X0));
  assert(internal21.unwrap().operator==(muX0ph1_app_X0));

  // Extra values are ignored
  auto vars012 = IdList();
  vars012.push_back(0);
  vars012.push_back(1);
  vars012.push_back(2);
  auto extraplugs = Patterns();
  extraplugs.push_back(phi1.clone());
  extraplugs.push_back(X0.clone());
  extraplugs.push_back(x0.clone());
  extraplugs.push_back(x0.clone());
  extraplugs.push_back(x0.clone());
  extraplugs.push_back(x0.clone());
  extraplugs.push_back(x0.clone());
  extraplugs.push_back(x0.clone());
  auto internal22 = Pattern::instantiate_internal(muX0phi0_app_ph1, vars011, extraplugs);
  auto internal23 = Pattern::instantiate_internal(muX0phi0_app_ph1, vars012, plugsphi1X0);

  assert(internal22.unwrap().operator==(muX0ph1_app_X0));
  assert(internal23.unwrap().operator==(muX0ph1_app_X0));

  // Instantiate with concrete patterns applies pending substitutions
  auto val = Pattern::esubst(phi0.clone(), 0, c0.clone());
  auto internal24 = Pattern::instantiate_internal(val, vars0, plugsx0);

  assert(internal24.unwrap().operator==(c0));

  val = Pattern::ssubst(phi0.clone(), 0, c0.clone());
  auto internal25 = Pattern::instantiate_internal(val, vars0, plugsX0);

  assert(internal25.unwrap().operator==(c0));

  val = Pattern::ssubst(
      Pattern::esubst(phi0.clone(), 0, X0.clone()), 0, c0.clone());
  auto internal26 = Pattern::instantiate_internal(val, vars0, plugsX0);

  assert(internal26.unwrap().operator==(c0));

  // Instantiate with metavar keeps pending substitutions
  auto plugsphi1 = Patterns();
  plugsphi1.push_back(phi1.clone());
  val = Pattern::esubst(phi0.clone(), 0, c0.clone());
  auto internal27 = Pattern::instantiate_internal(val, vars0, plugsphi1);

  assert(internal27.unwrap().operator==(Pattern::esubst(phi1.clone(), 0, c0.clone())));

  val = Pattern::ssubst(phi0.clone(), 0, c0.clone());
  auto internal28 = Pattern::instantiate_internal(val, vars0, plugsphi1);

  assert(internal28.unwrap().operator==(Pattern::ssubst(phi1.clone(), 0, c0.clone())));

  // The plug in a subst. needs to be instantiated as well
  val = Pattern::ssubst(phi0.clone(), 0, phi0.clone());
  auto internal29 = Pattern::instantiate_internal(val, vars0, plugsX0);

  assert(internal29.unwrap().operator==(X0));

  auto plugsX0c0 = Patterns();
  plugsX0c0.push_back(X0.clone());
  plugsX0c0.push_back(c0.clone());
  val = Pattern::ssubst(phi0.clone(), 0, phi1.clone());
  auto internal30 = Pattern::instantiate_internal(val, vars01, plugsX0c0);

  assert(internal30.unwrap().operator==(c0));


#if DEBUG
  x0->print();
  std::cout << std::endl;
  X0->print();
  std::cout << std::endl;
  c0->print();
  std::cout << std::endl;
  x0_implies_x0->print();
  std::cout << std::endl;
  appx0x0->print();
  std::cout << std::endl;
  existsx0x0->print();
  std::cout << std::endl;
  muX0x0->print();
  std::cout << std::endl;
  phi0->print();
  std::cout << std::endl;
  phi0_implies_phi0->print();
  std::cout << std::endl;
  internal0.unwrap()->print();
  std::cout << std::endl;
  internal2.unwrap()->print();
  std::cout << std::endl;
  internal4.unwrap()->print();
  std::cout << std::endl;
  internal6.unwrap()->print();
  std::cout << std::endl;
  phi1->print();
  std::cout << std::endl;
  muX0phi1->print();
  std::cout << std::endl;
  muX0X0->print();
  std::cout << std::endl;
  internal8.unwrap()->print();
  std::cout << std::endl;
  internal9.unwrap()->print();
  std::cout << std::endl;

#endif

}

void execute_vector(std::array<int, MAX_SIZE> &instrs, Pattern::Stack &stack,
                    Pattern::Memory &memory, Pattern::Claims &claims,
                    Pattern::ExecutionPhase phase) {

  Pattern::execute_instructions(instrs, stack, memory, claims, phase);
}

void test_publish() {

  std::array<int, MAX_SIZE> proof;
  proof[0] = 1;
  proof[1] = (int)Instruction::Publish;
  proof[2] = (int)138; // NO_OP

  auto stack = Pattern::Stack();
  auto memory = Pattern::Memory();
  auto claims = Pattern::Claims();
  stack.push_back(Pattern::Term::Pattern_(Pattern::symbol(0)));

  execute_vector(proof, stack, memory, claims, Pattern::ExecutionPhase::Gamma);

  auto expected_stack = Pattern::Stack();
  auto expected_claims = Pattern::Claims();
  auto expected_memory = Pattern::Memory();
  expected_memory.push_back(Pattern::Term::Proved_(Pattern::symbol(0)));

  assert(stack == expected_stack);
  assert(memory == expected_memory);
  assert(claims == expected_claims);

  memory.clear();

  stack = Pattern::Stack();
  memory = Pattern::Memory();
  claims = Pattern::Claims();
  stack.push_back(Pattern::Term::Pattern_(Pattern::symbol(0)));

  execute_vector(proof, stack, memory, claims, Pattern::ExecutionPhase::Claim);

  expected_memory.clear();

  expected_stack = Pattern::Stack();
  expected_memory = Pattern::Memory();
  expected_claims = Pattern::Claims();
  expected_claims.push_back(Pattern::symbol(0));

  assert(stack == expected_stack);
  assert(memory == expected_memory);
  assert(claims == expected_claims);

  claims.clear();

  stack = Pattern::Stack();
  memory = Pattern::Memory();
  claims = Pattern::Claims();
  stack.push_back(Pattern::Term::Proved_(Pattern::symbol(0)));
  claims.push_back(Pattern::symbol(0));

  execute_vector(proof, stack, memory, claims, Pattern::ExecutionPhase::Proof);

  expected_claims.clear();

  expected_stack = Pattern::Stack();
  expected_memory = Pattern::Memory();
  expected_claims = Pattern::Claims();

  assert(stack == expected_stack);
  assert(memory == expected_memory);
  assert(claims == expected_claims);

}

void test_construct_phi_implies_phi() {

  // MetaVar(0,0,0,0,0,0)
  // Save
  // Load 0
  // Implication
  // NO_OP
  std::array<int, MAX_SIZE> proof;
  proof[0] = 12; // Size
  proof[1] = (int)Instruction::MetaVar;
  proof[2] = 0;
  proof[3] = 0;
  proof[4] = 0;
  proof[5] = 0;
  proof[6] = 0;
  proof[7] = 0;
  proof[8] = (int)Instruction::Save;
  proof[9] = (int)Instruction::Load;
  proof[10] = 0;
  proof[11] = (int)Instruction::Implication;
  proof[12] = (int)138; // NO_OP

  auto stack = Pattern::Stack();
  auto memory = Pattern::Memory();
  auto claims = Pattern::Claims();

  execute_vector(proof, stack, memory, claims, Pattern::ExecutionPhase::Proof);

  auto phi0 = Pattern::metavar_unconstrained(0);
  auto expected_stack = Pattern::Stack();
  expected_stack.push_back(
      Pattern::Term::Pattern_(Pattern::implies(phi0.clone(), phi0.clone())));
  assert(stack == expected_stack);

}

void test_phi_implies_phi_impl() {

  std::array<int, MAX_SIZE> proof;
  proof[0] = 36; // Size
  proof[1] = (int)Instruction::MetaVar;
  proof[2] = 0;
  proof[3] = 0;
  proof[4] = 0;
  proof[5] = 0;
  proof[6] = 0;
  proof[7] = 0;
  // Stack: $ph0
  proof[8] = (int)Instruction::Save;
  // @0
  proof[9] = (int)Instruction::Load;
  proof[10] = 0;
  // Stack: $ph0; ph0
  proof[11] = (int)Instruction::Load;
  proof[12] = 0;
  // Stack: $ph0; $ph0; ph0
  proof[13] = (int)Instruction::Implication;
  // Stack: $ph0; ph0 -> ph0
  proof[14] = (int)Instruction::Save;
  // @1
  proof[15] = (int)Instruction::Prop2;
  // Stack: $ph0; $ph0 -> ph0;
  //        [prop2: (ph0 -> (ph1 -> ph2)) -> ((ph0 -> ph1) -> (ph0 -> ph2))]
  proof[16] = (int)Instruction::Instantiate;
  proof[17] = 1;
  proof[18] = 1;
  // Stack: $ph0; [p1: (ph0 -> ((ph0 -> ph0) -> ph2))
  //                -> (ph0 -> (ph0 -> ph0)) -> (ph0 -> ph2)]
  proof[19] = (int)Instruction::Instantiate;
  proof[20] = 1;
  proof[21] = 2;
  // Stack: [p1: (ph0 -> ((ph0 -> ph0) -> ph0))
  //          -> (ph0 -> (ph0 -> ph0)) -> (ph0 -> ph0)]
  proof[22] = (int)Instruction::Load;
  proof[23] = 1;
  // Stack: p1 ; $ph0 -> ph0
  proof[24] = (int)Instruction::Prop1;
  // Stack: p1 ; $ph0 -> ph0; [prop1: ph0 -> (ph1 -> ph0)]
  proof[25] = (int)Instruction::Instantiate;
  proof[26] = 1;
  proof[27] = 1;
  // Stack: p1 ; [p2: (ph0 -> (ph0 -> ph0) -> ph0) ]
  proof[28] = (int)Instruction::ModusPonens;
  // Stack: [p3: (ph0 -> (ph0 -> ph0)) -> (ph0 -> ph0)]
  proof[29] = (int)Instruction::Load;
  proof[30] = 0;
  // Stack: p3 ; ph0;
  proof[31] = (int)Instruction::Prop1;
  // Stack: p3 ; ph0; prop1
  proof[32] = (int)Instruction::Instantiate;
  proof[33] = 1;
  proof[34] = 1;
  // Stack: p3 ; ph0 -> (ph0 ->ph0)
  proof[35] = (int)Instruction::ModusPonens;
  // Stack: ph0 -> ph0
  proof[36] = (int)138; // NO_OP

  auto stack = Pattern::Stack();
  auto memory = Pattern::Memory();
  auto claims = Pattern::Claims();

  execute_vector(proof, stack, memory, claims, Pattern::ExecutionPhase::Proof);
#if DEBUG
  std::cout << "Stack size: " << stack.size() << std::endl;
  for (auto it : stack) {
    it.pattern->print();
    std::cout << std::endl;
  }
#endif

  auto phi0 = Pattern::metavar_unconstrained(0);
  auto expected_stack = Pattern::Stack();
  expected_stack.push_back(
      Pattern::Term::Proved_(Pattern::implies(phi0.clone(), phi0.clone())));

  assert(stack == expected_stack);

}

void test_universal_quantification() {

  std::array<int, MAX_SIZE> proof;
  proof[0] = 2;
  proof[1] = (int)Instruction::Generalization;
  proof[2] = 0;
  proof[3] = (int)138; // NO_OP

  auto stack = Pattern::Stack();
  auto memory = Pattern::Memory();
  auto claims = Pattern::Claims();
  stack.push_back(Pattern::Term::Proved_(Pattern::implies(Pattern::symbol(0), Pattern::symbol(1))));

  execute_vector(proof, stack, memory, claims, Pattern::ExecutionPhase::Proof);

  auto expected_stack = Pattern::Stack();
  auto expected_memory = Pattern::Memory();
  auto expected_claims = Pattern::Claims();

  expected_stack.push_back(Pattern::Term::Proved_(
    Pattern::implies(Pattern::exists(0, Pattern::symbol(0)), Pattern::symbol(1))));
  
  assert(stack == expected_stack);
  assert(memory == expected_memory);
  assert(claims == expected_claims);

}

void test_apply_esubst() {
  // Atomic cases
  auto pattern = Pattern::evar(0);
  auto plug = Pattern::symbol(1);
  auto result = Pattern::apply_esubst(pattern, 0, plug);
  assert(result == Pattern::symbol(1));

  pattern = Pattern::evar(0);
  plug = Pattern::evar(2);
  result = Pattern::apply_esubst(pattern, 0, plug);
  assert(result == Pattern::evar(2));

  result = Pattern::apply_esubst(pattern, 1, plug);
  assert(result == Pattern::evar(0));

  pattern = Pattern::svar(0);
  plug = Pattern::symbol(0);
  result = Pattern::apply_esubst(pattern, 0, plug);
  assert(result == Pattern::svar(0));

  pattern = Pattern::svar(1);
  plug = Pattern::evar(0);
  result = Pattern::apply_esubst(pattern, 0, plug);
  assert(result == Pattern::svar(1));

  pattern = Pattern::symbol(0);
  plug = Pattern::symbol(1);
  result = Pattern::apply_esubst(pattern, 0, plug);
  assert(result == Pattern::symbol(0));

  // Distribute over subpatterns
  pattern = Pattern::implies(Pattern::evar(7), Pattern::symbol(1));
  plug = Pattern::symbol(0);
  result = Pattern::apply_esubst(pattern, 7, plug);
  assert(result == Pattern::implies(Pattern::symbol(0), Pattern::symbol(1)));

  pattern = Pattern::implies(Pattern::evar(7), Pattern::symbol(1));
  plug = Pattern::symbol(0);
  result = Pattern::apply_esubst(pattern, 6, plug);
  assert(result == Pattern::implies(Pattern::evar(7), Pattern::symbol(1)));

  pattern = Pattern::app(Pattern::evar(7), Pattern::symbol(1));
  plug = Pattern::symbol(0);
  result = Pattern::apply_esubst(pattern, 7, plug);
  assert(result == Pattern::app(Pattern::symbol(0), Pattern::symbol(1)));

  pattern = Pattern::app(Pattern::evar(7), Pattern::symbol(1));
  plug = Pattern::symbol(0);
  result = Pattern::apply_esubst(pattern, 6, plug);
  assert(result == Pattern::app(Pattern::evar(7), Pattern::symbol(1)));

  // Distribute over subpatterns unless evar_id = binder
  pattern = Pattern::exists(1, Pattern::evar(1));
  plug = Pattern::symbol(2);
  result = Pattern::apply_esubst(pattern, 0, plug);
  assert(result == Pattern::exists(1, Pattern::evar(1)));

  pattern = Pattern::exists(0, Pattern::evar(1));
  plug = Pattern::symbol(2);
  result = Pattern::apply_esubst(pattern, 1, plug);
  assert(result == Pattern::exists(0, Pattern::symbol(2)));

  pattern = Pattern::mu(1, Pattern::evar(1));
  plug = Pattern::symbol(2);
  result = Pattern::apply_esubst(pattern, 0, plug);
  assert(result == Pattern::mu(1, Pattern::evar(1)));

  pattern = Pattern::mu(1, Pattern::evar(1));
  plug = Pattern::symbol(2);
  result = Pattern::apply_esubst(pattern, 1, plug);
  assert(result == Pattern::mu(1, Pattern::symbol(2)));

  // Subst on metavar should wrap in constructor
  pattern = Pattern::metavar_unconstrained(0);
  plug = Pattern::symbol(1);
  result = Pattern::apply_esubst(pattern, 0, plug);
  assert(result == Pattern::esubst(Pattern::metavar_unconstrained(0), 0, Pattern::symbol(1)));

  // Subst when evar_id is fresh should do nothing
  //(metavar_(0).with_e_fresh((evar(0), evar(1))), 0, symbol(1), metavar_unconstrained(0).with_e_fresh((evar(0), evar(1)))),
  // Subst on substs should stack
  pattern = Pattern::esubst(Pattern::metavar_unconstrained(0), 0, Pattern::symbol(1));
  plug = Pattern::symbol(1);
  result = Pattern::apply_esubst(pattern, 0, plug);
  assert(result == Pattern::esubst(
                     Pattern::esubst(Pattern::metavar_unconstrained(0), 0, Pattern::symbol(1)),
                     0,
                     Pattern::symbol(1)));

  pattern = Pattern::ssubst(Pattern::metavar_unconstrained(0), 0, Pattern::symbol(1));
  plug = Pattern::symbol(1);
  result = Pattern::apply_esubst(pattern, 0, plug);
  assert(result == Pattern::esubst(
                     Pattern::ssubst(Pattern::metavar_unconstrained(0), 0, Pattern::symbol(1)),
                     0,
                     Pattern::symbol(1)));

}

void test_apply_ssubst() {
  // Atomic cases
  auto pattern = Pattern::evar(0);
  auto plug = Pattern::symbol(1);
  auto result = Pattern::apply_ssubst(pattern, 0, plug);
  assert(result == Pattern::evar(0));

  pattern = Pattern::evar(0);
  plug = Pattern::evar(2);
  result = Pattern::apply_ssubst(pattern, 1, plug);
  assert(result == Pattern::evar(0));

  pattern = Pattern::svar(0);
  plug = Pattern::symbol(0);
  result = Pattern::apply_ssubst(pattern, 0, plug);
  assert(result == Pattern::symbol(0));

  pattern = Pattern::svar(1);
  plug = Pattern::evar(0);
  result = Pattern::apply_ssubst(pattern, 0, plug);
  assert(result == Pattern::svar(1));

  pattern = Pattern::symbol(0);
  plug = Pattern::symbol(1);
  result = Pattern::apply_ssubst(pattern, 0, plug);
  assert(result == Pattern::symbol(0));

  // Distribute over subpatterns
  pattern = Pattern::implies(Pattern::svar(7), Pattern::symbol(1));
  plug = Pattern::symbol(0);
  result = Pattern::apply_ssubst(pattern, 7, plug);
  assert(result == Pattern::implies(Pattern::symbol(0), Pattern::symbol(1)));

  pattern = Pattern::implies(Pattern::svar(7), Pattern::symbol(1));
  plug = Pattern::symbol(0);
  result = Pattern::apply_ssubst(pattern, 6, plug);
  assert(result == Pattern::implies(Pattern::svar(7), Pattern::symbol(1)));

  pattern = Pattern::app(Pattern::svar(7), Pattern::symbol(1));
  plug = Pattern::symbol(0);
  result = Pattern::apply_ssubst(pattern, 7, plug);
  assert(result == Pattern::app(Pattern::symbol(0), Pattern::symbol(1)));

  pattern = Pattern::app(Pattern::svar(7), Pattern::symbol(1));
  plug = Pattern::symbol(0);
  result = Pattern::apply_ssubst(pattern, 6, plug);
  assert(result == Pattern::app(Pattern::svar(7), Pattern::symbol(1)));

  // Distribute over subpatterns unless evar_id = binder
  pattern = Pattern::exists(1, Pattern::svar(0));
  plug = Pattern::symbol(2);
  result = Pattern::apply_ssubst(pattern, 0, plug);
  assert(result == Pattern::exists(1, Pattern::symbol(2)));

  pattern = Pattern::exists(1, Pattern::symbol(1));
  plug = Pattern::symbol(2);
  result = Pattern::apply_ssubst(pattern, 1, plug);
  assert(result == Pattern::exists(1, Pattern::symbol(1)));

  pattern = Pattern::mu(1, Pattern::svar(1));
  plug = Pattern::symbol(2);
  result = Pattern::apply_ssubst(pattern, 0, plug);
  assert(result == Pattern::mu(1, Pattern::svar(1)));

  pattern = Pattern::mu(1, Pattern::svar(1));
  plug = Pattern::symbol(2);
  result = Pattern::apply_ssubst(pattern, 1, plug);
  assert(result == Pattern::mu(1, Pattern::svar(1)));

  pattern = Pattern::mu(1, Pattern::svar(2));
  plug = Pattern::symbol(2);
  result = Pattern::apply_ssubst(pattern, 2, plug);
  assert(result == Pattern::mu(1, Pattern::symbol(2)));

  // Subst on metavar should wrap in constructor
  pattern = Pattern::metavar_unconstrained(0);
  plug = Pattern::symbol(1);
  result = Pattern::apply_ssubst(pattern, 0, plug);
  assert(result == Pattern::ssubst(Pattern::metavar_unconstrained(0), 0, Pattern::symbol(1)));

  // Subst when evar_id is fresh should do nothing
  //(metavar_(0).with_e_fresh((evar(0), evar(1))), 0, symbol(1), metavar_unconstrained(0).with_e_fresh((evar(0), evar(1)))),
  // Subst on substs should stack
  pattern = Pattern::esubst(Pattern::metavar_unconstrained(0), 0, Pattern::symbol(1));
  plug = Pattern::symbol(1);
  result = Pattern::apply_ssubst(pattern, 0, plug);
  assert(result == Pattern::ssubst(
                     Pattern::esubst(Pattern::metavar_unconstrained(0), 0, Pattern::symbol(1)),
                     0,
                     Pattern::symbol(1)));

  pattern = Pattern::ssubst(Pattern::metavar_unconstrained(0), 0, Pattern::symbol(1));
  plug = Pattern::symbol(1);
  result = Pattern::apply_ssubst(pattern, 0, plug);
  assert(result == Pattern::ssubst(
                     Pattern::ssubst(Pattern::metavar_unconstrained(0), 0, Pattern::symbol(1)),
                     0,
                     Pattern::symbol(1)));

}

void test_no_remaining_claims() {

  std::array<int, MAX_SIZE> gamma;
  gamma[0] = 0;        // Size
  gamma[1] = (int)138; // NO_OP

  std::array<int, MAX_SIZE> claims;
  claims[0] = 3; // Size
  claims[1] = (int)Instruction::Symbol;
  claims[2] = 0;
  claims[3] = (int)Instruction::Publish;
  claims[4] = (int)138; // NO_OP

  std::array<int, MAX_SIZE> proof;
  proof[0] = 0;        // Size
  proof[1] = (int)138; // NO_OP

  Pattern::verify(gamma, claims, proof);

}

void test_version_ok() {
  std::array<int, MAX_SIZE> gamma;
  gamma[0] = 3;        // Size
  gamma[1] = (int)Instruction::Version;
  gamma[2] = 0;
  gamma[3] = 1;
  gamma[1] = (int)138; // NO_OP

  std::array<int, MAX_SIZE> claims;
  claims[0] = 1;        // Size
  claims[1] = (int)138; // NO_OP

  std::array<int, MAX_SIZE> proof;
  proof[0] = 1;        // Size
  proof[1] = (int)138; // NO_OP

  Pattern::verify(gamma, claims, proof);
}

void test_version_fail() {
  std::array<int, MAX_SIZE> gamma;
  gamma[0] = 3;        // Size
  gamma[1] = (int)Instruction::Version;
  gamma[2] = -1;
  gamma[3] = -1;
  gamma[4] = (int)Instruction::NO_OP;

    std::array<int, MAX_SIZE> claims;
  claims[0] = 1;        // Size
  claims[1] = (int)Instruction::NO_OP;

  std::array<int, MAX_SIZE> proof;
  proof[0] = 1;        // Size
  proof[1] = (int)Instruction::NO_OP;

  // This will raise an excetion what we can't catch due to 
  // `execute_instructions` and `verify` being `noexcept` fuctions as required
  // by zkLLVM
  Pattern::verify(gamma, claims, proof);
}