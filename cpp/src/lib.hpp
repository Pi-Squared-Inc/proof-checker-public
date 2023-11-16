#include "../include/shared_ptr.hpp"
#include <array>
#include <cassert>
#include <iostream>
#include <memory>

#define MAX_SIZE 27001 // For Simple transfer 1785 is enough

enum class Instruction : int {
  // Patterns
  EVar = 2,
  SVar,
  Symbol,
  Implication,
  Application,
  Mu,
  Exists,
  // Meta Patterns,
  MetaVar,
  ESubst,
  SSubst,
  // Axiom Schemas,
  Prop1,
  Prop2,
  Prop3,
  Quantifier,
  PropagationOr,
  PropagationExists,
  PreFixpoint,
  Existence,
  Singleton,
  // Inference rules,
  ModusPonens,
  Generalization,
  Frame,
  Substitution,
  KnasterTarski,
  // Meta Incference rules,
  Instantiate,
  // Stack Manipulation,
  Pop,
  // Memory Manipulation,
  Save,
  Load,
  // Journal Manipulation,
  Publish,
  // Metavar with no constraints
  CleanMetaVar, // For some reason setting CleanMetaVar = (int)(9 + 128)
                // isn't working with zkLLVM
  // EOF exclusive for zkLLVM
  NO_OP
};

Instruction from(int value) noexcept {
  switch (value) {
  case 2:
    return Instruction::EVar;
  case 3:
    return Instruction::SVar;
  case 4:
    return Instruction::Symbol;
  case 5:
    return Instruction::Implication;
  case 6:
    return Instruction::Application;
  case 7:
    return Instruction::Mu;
  case 8:
    return Instruction::Exists;
  case 9:
    return Instruction::MetaVar;
  case 10:
    return Instruction::ESubst;
  case 11:
    return Instruction::SSubst;
  case 12:
    return Instruction::Prop1;
  case 13:
    return Instruction::Prop2;
  case 14:
    return Instruction::Prop3;
  case 15:
    return Instruction::Quantifier;
  case 16:
    return Instruction::PropagationOr;
  case 17:
    return Instruction::PropagationExists;
  case 18:
    return Instruction::PreFixpoint;
  case 19:
    return Instruction::Existence;
  case 20:
    return Instruction::Singleton;
  case 21:
    return Instruction::ModusPonens;
  case 22:
    return Instruction::Generalization;
  case 23:
    return Instruction::Frame;
  case 24:
    return Instruction::Substitution;
  case 25:
    return Instruction::KnasterTarski;
  case 26:
    return Instruction::Instantiate;
  case 27:
    return Instruction::Pop;
  case 28:
    return Instruction::Save;
  case 29:
    return Instruction::Load;
  case 30:
    return Instruction::Publish;
  case 137:
    return Instruction::CleanMetaVar;
  case 138:
    return Instruction::NO_OP;
  default:
    exit(1); // Bad instruction!
  }
}

struct Pattern {
  Instruction inst;       // All
  Id id;                  // EVar, SVar, Symbol, Mu, Exists, MetaVar,
                          // ESubst (evar_id), SSubst (svar_id)
  Rc<Pattern> left;       // Implication, Application
  Rc<Pattern> right;      // Implication, Application
  Rc<Pattern> subpattern; // Exists, Mu, ESubst (pattern), SSubst (pattern)
  Rc<Pattern> plug;       // ESubst, SSubst

  IdList e_fresh;       // MetaVar
  IdList s_fresh;       // MetaVar
  IdList positive;      // MetaVar
  IdList negative;      // MetaVar
  IdList app_ctx_holes; // MetaVar

  // Constructor for creating instances of Pattern
  static Pattern *newPattern(Instruction inst, Id id) noexcept {
    auto pattern = static_cast<Pattern *>(malloc(sizeof(Pattern)));
    memset_(pattern, 0, sizeof(Pattern));

    pattern->id = id;
    pattern->inst = inst;

    return pattern;
  }

  // Equality operator
  bool operator==(const Pattern &rhs) const noexcept {
    if (inst != rhs.inst || id != rhs.id) {
      return false;
    }
    if (left != rhs.left) {
      return false;
    }
    if (right != rhs.right) {
      return false;
    }
    if (subpattern != rhs.subpattern) {
      return false;
    }
    if (plug != rhs.plug) {
      return false;
    }
    if (e_fresh != rhs.e_fresh) {
      return false;
    }
    if (s_fresh != rhs.s_fresh) {
      return false;
    }
    if(positive != rhs.positive) {
      return false;
    }
    if (negative != rhs.negative) {
      return false;
    }
    if (app_ctx_holes != rhs.app_ctx_holes) {
      return false;
    }
    return true;
  }

  bool operator!=(const Pattern &rhs) noexcept { return !(*this == rhs); }

  bool pattern_e_fresh(Id evar) noexcept {
    switch (inst) {
    case Instruction::EVar:
      return evar != id;
    case Instruction::SVar:
    case Instruction::Symbol:
      return true;
    case Instruction::MetaVar:
      return e_fresh.contains(evar);
    case Instruction::Implication:
    case Instruction::Application:
      return left->pattern_e_fresh(evar) && right->pattern_e_fresh(evar);
    case Instruction::Exists:
      return (evar == id) || subpattern->pattern_e_fresh(evar);
    case Instruction::Mu:
      return subpattern->pattern_e_fresh(evar);
    case Instruction::ESubst:
      // Assume: substitution is well-formed => plug occurs in the result

      if (evar == id /*evar_id*/) {
        // Freshness depends only on plug, as all the free instances
        // of the requested variable are being substituted
        return plug->pattern_e_fresh(evar);
      }

      // Freshness depends on both input and plug,
      // as evar != evar_id (note that instances of evar_id
      // in pattern do not influence the result)
      return subpattern->pattern_e_fresh(evar) && plug->pattern_e_fresh(evar);

    case Instruction::SSubst:
      // Assume: substitution is well-formed => plug occurs in the result

      // We can skip checking evar == svar_id, because different types

      // Freshness depends on both input and plug,
      // as svar_id != evar (note that instances of evar_id
      // in pattern do not influence the result)
      return subpattern->pattern_e_fresh(evar) && plug->pattern_e_fresh(evar);

    default:
      return false;
    }
  }

  bool pattern_s_fresh(Id svar) noexcept {
    switch (inst) {
    case Instruction::EVar:
      return true;
    case Instruction::SVar:
      return id != svar;
    case Instruction::Symbol:
      return true;
    case Instruction::MetaVar:
      return s_fresh.contains(svar);
    case Instruction::Implication:
    case Instruction::Application:
      return left->pattern_s_fresh(svar) && right->pattern_s_fresh(svar);
    case Instruction::Exists:
      return subpattern->pattern_s_fresh(svar);
    case Instruction::Mu:
      return (svar == id) || subpattern->pattern_s_fresh(svar);
    case Instruction::ESubst:
      // Assume: substitution is well-formed => plug occurs in the result

      // We can skip checking svar == evar_id, because different types

      // Freshness depends on both input and plug,
      // as evar_id != svar (note that instances of evar_id
      // in pattern do not influence the result)
      return subpattern->pattern_s_fresh(svar) && plug->pattern_s_fresh(svar);

    case Instruction::SSubst:
      // Assume: substitution is well-formed => plug occurs in the result
      if (svar == id /*svar_id*/) {
        // Freshness depends only on plug as all the free instances
        // of the requested variable are being substituted
        return plug->pattern_s_fresh(svar);
      }

      return subpattern->pattern_s_fresh(svar) && plug->pattern_s_fresh(svar);

    default:
      return false;
    }
  }

  bool pattern_positive(Id svar) noexcept {
    switch (inst) {
    case Instruction::EVar:
    case Instruction::SVar:
    case Instruction::Symbol:
      return true;
    case Instruction::MetaVar:
      return positive.contains(svar);
    case Instruction::Implication:
      return left->pattern_negative(svar) && right->pattern_positive(svar);
    case Instruction::Application:
      return left->pattern_positive(svar) && right->pattern_positive(svar);
    case Instruction::Exists:
      return subpattern->pattern_positive(svar);
    case Instruction::Mu:
      return svar == id || subpattern->pattern_positive(svar);
    case Instruction::ESubst:
      // best-effort for now, see spec
      return subpattern->pattern_positive(svar) && plug->pattern_s_fresh(svar);
    case Instruction::SSubst: {
      auto plug_positive_svar =
          plug->pattern_s_fresh(svar) ||
          (subpattern->pattern_positive(id) && plug->pattern_positive(svar)) ||
          (subpattern->pattern_negative(id) && plug->pattern_negative(svar));

      if (svar == id) {
        return plug_positive_svar;
      }

      return subpattern->pattern_positive(svar) && plug_positive_svar;
    }
    default:
      return false;
    }
  }

  bool pattern_negative(Id svar) noexcept {
    switch (inst) {
    case Instruction::EVar:
      return true;
    case Instruction::SVar:
      return id != svar;
    case Instruction::Symbol:
      return true;
    case Instruction::MetaVar:
      return negative.contains(svar);
    case Instruction::Implication:
      return left->pattern_positive(svar) && right->pattern_negative(svar);
    case Instruction::Application:
      return left->pattern_negative(svar) && right->pattern_negative(svar);
    case Instruction::Exists:
      return subpattern->pattern_s_fresh(svar);
    case Instruction::Mu:
      return svar == id || subpattern->pattern_negative(svar);
    case Instruction::ESubst:
      // best-effort for now, see spec
      return subpattern->pattern_negative(svar) && plug->pattern_s_fresh(svar);
    case Instruction::SSubst: {
      auto plug_negative_svar =
          plug->pattern_s_fresh(svar) ||
          (subpattern->pattern_positive(id) && plug->pattern_negative(svar)) ||
          (subpattern->pattern_negative(id) && plug->pattern_positive(svar));

      if (svar == id) {
        return plug_negative_svar;
      }

      return subpattern->pattern_negative(svar) && plug_negative_svar;
    }
    default:
      return false;
    }
  }

  // Checks whether pattern is well-formed ASSUMING
  // that the sub-patterns are well-formed
  bool pattern_well_formed() noexcept {
    switch (inst) {
    case Instruction::MetaVar:
      return !app_ctx_holes.containsElementOf(&e_fresh);
    case Instruction::Mu:
      return subpattern->pattern_positive(id);
    case Instruction::ESubst:
      return !subpattern->pattern_e_fresh(id);
    case Instruction::SSubst:
      return !subpattern->pattern_s_fresh(id);
    default:
      return false;
    }
  }

  /// Pattern construction utilities
  /// ------------------------------
  static Rc<Pattern> evar(Id id) noexcept {
    return newPattern(Instruction::EVar, id);
  }

  static Rc<Pattern> svar(Id id) noexcept {
    return newPattern(Instruction::SVar, id);
  }

  static Rc<Pattern> symbol(Id id) noexcept {
    return newPattern(Instruction::Symbol, id);
  }

  static Rc<Pattern> metavar_unconstrained(Id id) noexcept {
    auto pattern = newPattern(Instruction::MetaVar, id);
    pattern->e_fresh = IdList();
    pattern->s_fresh = IdList();
    pattern->positive = IdList();
    pattern->negative = IdList();
    pattern->app_ctx_holes = IdList();
    return pattern;
  }

  static Rc<Pattern> metavar_s_fresh(Id id, Id s_fresh, IdList positive,
                                     IdList negative) noexcept {
    auto pattern = newPattern(Instruction::MetaVar, id);
    pattern->e_fresh = IdList();
    pattern->s_fresh = IdList(s_fresh);
    pattern->positive = std::move(positive);
    pattern->negative = std::move(negative);
    pattern->app_ctx_holes = IdList();
    return pattern;
  }

  static Rc<Pattern> metavar(Id id, IdList e_fresh, IdList s_fresh,
                             IdList positive, IdList negative,
                             IdList app_ctx_holes) noexcept {
    auto pattern = newPattern(Instruction::MetaVar, id);
    pattern->e_fresh = std::move(e_fresh);
    pattern->s_fresh = std::move(s_fresh);
    pattern->positive = std::move(positive);
    pattern->negative = std::move(negative);
    pattern->app_ctx_holes = std::move(app_ctx_holes);
    return pattern;
  }

  static Rc<Pattern> exists(Id var, Rc<Pattern> subpattern) noexcept {
    auto pattern = newPattern(Instruction::Exists, var);
    pattern->subpattern = subpattern;
    return pattern;
  }

  static Rc<Pattern> mu(Id var, Rc<Pattern> subpattern) noexcept {
    auto pattern = newPattern(Instruction::Mu, var);
    pattern->subpattern = subpattern;
    return pattern;
  }

  static Rc<Pattern> esubst(Rc<Pattern> pattern, int evar_id,
                            Rc<Pattern> plug) noexcept {
    auto evarPattern = newPattern(Instruction::ESubst, evar_id);
    evarPattern->subpattern = pattern;
    evarPattern->plug = plug;
    return evarPattern;
  }

  static Rc<Pattern> ssubst(Rc<Pattern> pattern, int svar_id,
                            Rc<Pattern> plug) noexcept {
    auto svarPattern = newPattern(Instruction::SSubst, svar_id);
    svarPattern->subpattern = pattern;
    svarPattern->plug = plug;
    return svarPattern;
  }

  static Rc<Pattern> implies(Rc<Pattern> left, Rc<Pattern> right) noexcept {
    auto pattern = newPattern(Instruction::Implication, 0);
    pattern->left = left;
    pattern->right = right;
    return pattern;
  }

  static Rc<Pattern> app(Rc<Pattern> left, Rc<Pattern> right) noexcept {
    auto pattern = newPattern(Instruction::Application, 0);
    pattern->left = left;
    pattern->right = right;
    return pattern;
  }

  // Destructor to manually release memory
  ~Pattern() noexcept = default;

#if DEBUG
  void print() noexcept {
    switch (inst) {
    case Instruction::EVar:
      std::cout << "EVar(" << static_cast<int>(id) << ")";
      break;
    case Instruction::SVar:
      std::cout << "SVar(" << static_cast<int>(id) << ")";
      break;
    case Instruction::Symbol:
      std::cout << "Symbol(" << static_cast<int>(id) << ")";
      break;
    case Instruction::Implication:
      /* std::cout << "Implication(";
       left->print();
       std::cout << ", ";
       right->print();
       std::cout << ")";*/
      std::cout << "(";
      left->print();
      std::cout << " -> ";
      right->print();
      std::cout << ")";
      break;
    case Instruction::Application:
      std::cout << "Application(";
      left->print();
      std::cout << ", ";
      right->print();
      std::cout << ")";
      break;
    case Instruction::Exists:
      std::cout << "Exists(" << static_cast<int>(id) << ", ";
      subpattern->print();
      std::cout << ")";
      break;
    case Instruction::Mu:
      std::cout << "Mu(" << static_cast<int>(id) << ", ";
      subpattern->print();
      std::cout << ")";
      break;
    case Instruction::MetaVar:
      // std::cout << "phi" << static_cast<int>(id);
      std::cout << "MetaVar(" << static_cast<int>(id);
      if (e_fresh.head) {
        std::cout << ", ";
        e_fresh.print();
      }
      if (s_fresh.head) {
        std::cout << ", ";
        s_fresh.print();
      }
      if (positive.head) {
        std::cout << ", ";
        positive.print();
      }
      if (negative.head) {
        std::cout << ", ";
        negative.print();
      }
      if (app_ctx_holes.head) {
        std::cout << ", ";
        app_ctx_holes.print();
      }
      std::cout << ")";
      break;
    case Instruction::ESubst:
      std::cout << "ESubst(";
      subpattern->print();
      std::cout << ", " << static_cast<int>(id) << ", ";
      plug->print();
      std::cout << ")";
      break;
    case Instruction::SSubst:
      std::cout << "SSubst(";
      subpattern->print();
      std::cout << ", " << static_cast<int>(id) << ", ";
      plug->print();
      std::cout << ")";
      break;
    }
  }

  class Term;
  static void printStack(LinkedList<Term> &stack) noexcept {
    std::cout << "Stack: ";
    for (Term it : stack) {
      if (it.type == Term::Type::Pattern) {
        std::cout << "[ Pattern: ";
        // it.pattern->print();
        std::cout << "]; ";
      } else if (it.type == Term::Type::Proved) {
        std::cout << "[ Proved: ";
        // it.pattern->print();
        std::cout << " ]; ";
      }
      std::cout << std::endl;
    }
    std::cout << std::endl;
  }
#endif

  class Term {
  public:
    enum class Type { Pattern, Proved };
    Type type;
    Rc<Pattern> pattern = Rc<Pattern>();
    Term() noexcept : type(Type::Pattern), pattern(Rc<Pattern>()) {}
    Term(Type type, Rc<Pattern> pattern) noexcept
        : type(type), pattern(pattern) {}
    static Term Pattern_(Rc<Pattern> pattern) noexcept {
      return Term(Type::Pattern, pattern);
    }
    static Term Proved_(Rc<Pattern> pattern) noexcept {
      return Term(Type::Proved, pattern);
    }
    ~Term() noexcept {}

    bool operator==(const Term &rhs) const noexcept {
      if (type != rhs.type) {
        return false;
      }

      return pattern == rhs.pattern;
    }
    bool operator!=(const Term &rhs) const noexcept { return !(*this == rhs); }
  };

  // Notation
  static Rc<Pattern> bot() noexcept { return mu(0, svar(0)); }

  static Rc<Pattern>
  negate(Rc<Pattern> pattern) noexcept { // C++ doesn't accepted not
    return implies(pattern, bot());
  }

  static Rc<Pattern> forall(Id evar, Rc<Pattern> pattern) noexcept {
    return negate(exists(evar, negate(pattern)));
  }

  /// Substitution utilities
  /// ----------------------
  template <class T> class Optional {
  private:
    T value;
    bool hasValue = false;

  public:
    Optional(const T &value) noexcept : value(value), hasValue(true) {}
    Optional() noexcept : hasValue(false) {}
    ~Optional() noexcept {}

    operator bool() const noexcept { return hasValue; }

    // returns nullptr if hasValue is false
    T operator*() noexcept { return value; }
    T unwrap() noexcept { return value; }

    bool has_value() noexcept { return hasValue; }
  };

  static Optional<Rc<Pattern>>
  instantiate_internal(Rc<Pattern> &p, IdList &vars,
                       LinkedList<Rc<Pattern>> &plugs) noexcept {
    switch (p->inst) {
    case Instruction::EVar:
    case Instruction::SVar:
    case Instruction::Symbol:
      return Optional<Rc<Pattern>>();
    case Instruction::MetaVar: {
      Id pos = 0;
      for (auto it : vars) {
        if (it == p->id) {
          for (const auto &evar : p->e_fresh) {
            if (!plugs[pos]->pattern_e_fresh(evar)) {
#ifdef DEBUG
              throw std::runtime_error("Instantiation of MetaVar " +
                                       std::to_string(p->id) +
                                       " breaks a freshness constraint: EVar " +
                                       std::to_string(evar));
#endif
              exit(1);
            }
          }
          for (const auto &svar : p->s_fresh) {
            if (!plugs[pos]->pattern_s_fresh(svar)) {
#ifdef DEBUG
              throw std::runtime_error("Instantiation of MetaVar " +
                                       std::to_string(p->id) +
                                       " breaks a freshness constraint: SVar " +
                                       std::to_string(svar));
#endif
              exit(1);
            }
          }
          for (const auto &svar : p->positive) {
            if (!plugs[pos]->pattern_positive(svar)) {
#ifdef DEBUG
              throw std::runtime_error(
                  "Instantiation of MetaVar " + std::to_string(p->id) +
                  " breaks a positivity constraint: SVar " +
                  std::to_string(svar));
#endif
              exit(1);
            }
          }
          for (const auto &svar : p->negative) {
            if (!plugs[pos]->pattern_negative(svar)) {
#ifdef DEBUG
              throw std::runtime_error(
                  "Instantiation of MetaVar " + std::to_string(p->id) +
                  " breaks a negativity constraint: SVar " +
                  std::to_string(svar));
#endif
              exit(1);
            }
          }
          if (pos >= plugs.size()) {
#ifdef DEBUG
            throw std::runtime_error(
                "Substitution does not contain a corresponding value.");
#endif
            exit(1);
          }

          return Optional<Rc<Pattern>>(plugs[pos]);
        }
        pos++;
      }
      return Optional<Rc<Pattern>>();
    }
    case Instruction::Implication: {
      auto left = Rc<Pattern>(p->left);
      auto right = Rc<Pattern>(p->right);

      auto inst_left = instantiate_internal(left, vars, plugs);
      auto inst_right = instantiate_internal(right, vars, plugs);

      if (!inst_left.has_value() && !inst_right.has_value()) {
        return Optional<Rc<Pattern>>();
      } else {
        if (!inst_left.has_value()) {
          inst_left = Optional<Rc<Pattern>>(left.clone());
        }
        if (!inst_right.has_value()) {
          inst_right = Optional<Rc<Pattern>>(right.clone());
        }
        return Optional<Rc<Pattern>>(
            implies(inst_left.unwrap(), inst_right.unwrap()));
      }
    }
    case Instruction::Application: {
      auto left = Rc<Pattern>(p->left);
      auto right = Rc<Pattern>(p->right);

      auto inst_left = instantiate_internal(left, vars, plugs);
      auto inst_right = instantiate_internal(right, vars, plugs);

      if (!inst_left.has_value() && !inst_right.has_value()) {
        return Optional<Rc<Pattern>>();
      } else {
        if (!inst_left.has_value()) {
          inst_left = Optional<Rc<Pattern>>(left.clone());
        }
        if (!inst_right.has_value()) {
          inst_right = Optional<Rc<Pattern>>(right.clone());
        }
        return Optional<Rc<Pattern>>(
            app(inst_left.unwrap(), inst_right.unwrap()));
      }
    }
    case Instruction::Exists: {
      auto subpattern = Rc<Pattern>(p->subpattern);

      auto inst_sub = instantiate_internal(subpattern, vars, plugs);

      if (!inst_sub.has_value()) {
        return Optional<Rc<Pattern>>();
      } else {
        if (!inst_sub.has_value()) {
          inst_sub = Optional<Rc<Pattern>>(subpattern.clone());
        }
        return Optional<Rc<Pattern>>(exists(p->id, inst_sub.unwrap()));
      }
    }
    case Instruction::Mu: {
      auto subpattern = Rc<Pattern>(p->subpattern);

      auto inst_sub = instantiate_internal(subpattern, vars, plugs);

      if (!inst_sub.has_value()) {
        return Optional<Rc<Pattern>>();
      } else {
        if (!inst_sub.has_value()) {
          inst_sub = Optional<Rc<Pattern>>(subpattern.clone());
        }
        return Optional<Rc<Pattern>>(mu(p->id, (inst_sub.unwrap())));
      }
    }
    case Instruction::ESubst: {
      auto subpattern = Rc<Pattern>(p->subpattern);
      auto plug = Rc<Pattern>(p->plug);

      auto inst_pattern = instantiate_internal(subpattern, vars, plugs);
      auto inst_plug = instantiate_internal(plug, vars, plugs);

      if (!inst_pattern.has_value() && !inst_plug.has_value()) {
        return Optional<Rc<Pattern>>();
      } else {
        if (!inst_pattern.has_value()) {
          inst_pattern = Optional<Rc<Pattern>>(subpattern.clone());
        }
        if (!inst_plug.has_value()) {
          inst_plug = Optional<Rc<Pattern>>(plug.clone());
        }
        return Optional<Rc<Pattern>>(
            esubst(inst_pattern.unwrap(), p->id, inst_plug.unwrap()));
      }
    }
    case Instruction::SSubst: {
      auto subpattern = Rc<Pattern>(p->subpattern);
      auto plug = Rc<Pattern>(p->plug);

      auto inst_pattern = instantiate_internal(subpattern, vars, plugs);
      auto inst_plug = instantiate_internal(plug, vars, plugs);

      if (!inst_pattern.has_value() && !inst_plug.has_value()) {
        return Optional<Rc<Pattern>>();
      } else {
        if (!inst_pattern.has_value()) {
          inst_pattern = Optional<Rc<Pattern>>(subpattern.clone());
        }
        if (!inst_plug.has_value()) {
          inst_plug = Optional<Rc<Pattern>>(plug.clone());
        }
        return Optional<Rc<Pattern>>(
            ssubst(inst_pattern.unwrap(), p->id, inst_plug.unwrap()));
      }
    }
    default:
      return Optional<Rc<Pattern>>();
    }
  }

  static void instantiate_in_place(Rc<Pattern> &p, IdList &vars,
                                   LinkedList<Rc<Pattern>> &plugs) noexcept {
    if (auto ret = instantiate_internal(p, vars, plugs)) {
      p = ret.unwrap();
    }
  }

  /// Proof checker
  /// =============

  typedef LinkedList<Term> Stack;
  typedef LinkedList<Rc<Pattern>> Claims;
  typedef LinkedList<Term> Memory;

  /// Stack utilities
  /// ---------------

  static Term pop_stack(Stack &stack) noexcept {
    auto elem = stack.pop();
    elem.pattern.release();
    return elem;
  }

  static Rc<Pattern> pop_stack_pattern(Stack &stack) noexcept {
    auto term = pop_stack(stack);
    if (term.type != Term::Type::Pattern) {
#if DEBUG
      throw std::runtime_error("Expected pattern on the stack.");
#endif
      exit(1);
    }
    return term.pattern;
  }

  static Rc<Pattern> pop_stack_proved(Stack &stack) noexcept {
    auto term = pop_stack(stack);
    if (term.type != Term::Type::Proved) {
#if DEBUG
      throw std::runtime_error("Expected proved on the stack.");
#endif
      exit(1);
    }
    return term.pattern;
  }

  /// Main implementation
  /// -------------------

  enum class ExecutionPhase { Gamma, Claim, Proof };

  static void
  read_u8_vec(std::array<int, MAX_SIZE>::iterator &iterator, IdList *vec) noexcept {
    auto size = *iterator;
    iterator++;
    for (int i = 0; i < size; i++) {
      vec->push_back(static_cast<int>(*iterator));
      iterator++;
    }
  }

  static void execute_instructions(std::array<int, MAX_SIZE> &buffer,
                                   Stack &stack, Memory &memory, Claims &claims,
                                   ExecutionPhase phase) noexcept {

    // Get an iterator for the input buffer
    auto iterator = buffer.begin();
    iterator++; // Skip the first byte, which is the size of the buffer

    // Metavars
    // Phi0 = MetaVar(0)
    // Phi1 = MetaVar(1)
    // Phi2 = MetaVar(2)
    auto phi0 = metavar_unconstrained(0);
    auto phi1 = metavar_unconstrained(1);
    auto phi2 = metavar_unconstrained(2);

    // Axioms
    // Prop1: phi0 => (phi1 => phi0)
    // Prop2: (phi0 => (phi1 => phi2)) => ((phi0 => phi1) => (phi0 => phi2))
    // Prop3: ((~(~phi0)) => phi0)
    auto prop1 = implies(phi0.clone(), implies(phi1.clone(), phi0.clone()));
    auto prop2 =
        implies(implies(phi0.clone(), implies(phi1.clone(), phi2.clone())),
                implies(implies(phi0.clone(), phi1.clone()),
                        implies(phi0.clone(), phi2.clone())));
    auto prop3 = implies(negate(negate(phi0.clone())), phi0.clone());

    // Quantifier: forall x. phi0
    auto quantifier =
        implies(esubst(phi0.clone(), 0, evar(1)), exists(0, phi0.clone()));

    // Existence: exists x. phi0
    auto existence = exists(0, phi0.clone());

    // Iteration through the input buffer
    while (iterator != buffer.end()) {
      Instruction instr_u32 = from(static_cast<int>(*iterator));
      iterator++;

      switch (instr_u32) {
        // TODO: Add an abstraction for pushing these one-argument terms on
        // stack?
      case Instruction::EVar: {
        auto id = iterator;
        iterator++;
        if (id == buffer.end()) {
#if DEBUG
          throw std::runtime_error(
              "Expected id for the EVar to be put on stack");
#endif
          exit(1);
        }
        stack.push(Term::Pattern_(evar(static_cast<Id>(*id))));
        break;
      }
      case Instruction::SVar: {
        auto id = iterator;
        iterator++;
        if (id == buffer.end()) {
#if DEBUG
          throw std::runtime_error(
              "Expected id for the SVar to be put on stack");
#endif
          exit(1);
        }
        stack.push(Term::Pattern_(svar(static_cast<Id>(*id))));
        break;
      }
      case Instruction::Symbol: {
        auto id = iterator;
        iterator++;
        if (id == buffer.end()) {
#if DEBUG
          throw std::runtime_error(
              "Expected id for the Symbol to be put on stack");
#endif
          exit(1);
        }
        stack.push(Term::Pattern_(symbol(static_cast<Id>(*id))));
        break;
      }
      case Instruction::MetaVar: {
        auto getId = iterator;
        iterator++;
        if (getId == buffer.end()) {
#if DEBUG
          throw std::runtime_error("Expected id for MetaVar instruction");
#endif
          exit(1);
        }
        auto id = static_cast<Id>(*getId);

        auto e_fresh = IdList();
        read_u8_vec(iterator, &e_fresh);
        auto s_fresh = IdList();
        read_u8_vec(iterator, &s_fresh);
        auto positive = IdList();
        read_u8_vec(iterator, &positive);
        auto negative = IdList();
        read_u8_vec(iterator, &negative);
        auto app_ctx_holes = IdList();
        read_u8_vec(iterator, &app_ctx_holes);

        auto metavar_pat =
            metavar(id, std::move(e_fresh), std::move(s_fresh), std::move(positive),
                std::move(negative), std::move(app_ctx_holes));

        if (!metavar_pat->pattern_well_formed()) {
#if DEBUG
          throw std::runtime_error("Constructed meta-var " +
                                   std::to_string(id) + " is ill-formed.");
#endif
          exit(1);
        }
        stack.push(Term::Pattern_(metavar_pat));
        break;
      }
      case Instruction::Implication: {
        auto right = pop_stack_pattern(stack);
        auto left = pop_stack_pattern(stack);
        stack.push(Term::Pattern_(implies(left, right)));
        break;
      }
      case Instruction::Application: {
        auto right = pop_stack_pattern(stack);
        auto left = pop_stack_pattern(stack);
        stack.push(Term::Pattern_(app(left, right)));
        break;
      }

      case Instruction::Exists: {
        auto id = iterator;
        iterator++;
        if (id == buffer.end()) {
#if DEBUG
          throw std::runtime_error("Expected var_id for the exists binder");
#endif
          exit(1);
        }

        auto subpattern = pop_stack_pattern(stack);
        stack.push(Term::Pattern_(exists(static_cast<Id>(*id), subpattern)));
        break;
      }

      case Instruction::Mu: {
        auto id = iterator;
        iterator++;
        if (id == buffer.end()) {
#if DEBUG
          throw std::runtime_error("Expected var_id for the mu binder");
#endif
          exit(1);
        }

        auto subpattern = pop_stack_pattern(stack);
        auto mu_pat = mu(static_cast<Id>(*id), subpattern);
        if (!mu_pat->pattern_well_formed()) {
#if DEBUG
          throw std::runtime_error("Constructed mu-pattern " +
                                   std::to_string((Id)*id) + " is ill-formed.");
#endif
          exit(1);
        }
        stack.push(Term::Pattern_(mu_pat));
        break;
      }

      case Instruction::ESubst: {
        auto evar_id = iterator;
        iterator++;
        if (evar_id == buffer.end()) {
#if DEBUG
          throw std::runtime_error(
              "Insufficient parameters for ESubst instruction");
#endif
          exit(1);
        }

        auto pattern = pop_stack_pattern(stack);
        auto plug = pop_stack_pattern(stack);
        Instruction pattern_inst = pattern->inst;
        if (!(pattern_inst == Instruction::MetaVar ||
              pattern_inst == Instruction::ESubst ||
              pattern_inst == Instruction::SSubst)) {
#if DEBUG
          throw std::runtime_error("Cannot apply ESubst on concrete term!");
#endif
          exit(1);
        }

        auto esubst_pat =
            esubst(pattern.clone(), static_cast<Id>(*evar_id), plug);
        if (esubst_pat->pattern_well_formed()) {
          // The substitution is redundant, we don't apply it.
          stack.push(Term::Pattern_(pattern));
        } else {
          stack.push(Term::Pattern_(esubst_pat));
        }
        break;
      }

      case Instruction::SSubst: {
        auto svar_id = iterator;
        iterator++;
        if (svar_id == buffer.end()) {
#if DEBUG
          throw std::runtime_error(
              "Insufficient parameters for SSubst instruction");
#endif
          exit(1);
        }

        auto pattern = pop_stack_pattern(stack);
        auto plug = pop_stack_pattern(stack);
        Instruction pattern_inst = pattern->inst;
        if (!(pattern_inst == Instruction::MetaVar ||
              pattern_inst == Instruction::ESubst ||
              pattern_inst == Instruction::SSubst)) {
#if DEBUG
          throw std::runtime_error("Cannot apply SSubst on concrete term!");
#endif
          exit(1);
        }

        auto ssubst_pat =
            ssubst(pattern.clone(), static_cast<Id>(*svar_id), plug);
        if (!ssubst_pat->pattern_well_formed()) {
          // The substitution is redundant, we don't apply it.
          stack.push(Term::Pattern_(pattern));
        } else {
          stack.push(Term::Pattern_(ssubst_pat));
        }
        break;
      }
      case Instruction::Prop1: {
        stack.push(Term::Proved_(prop1.clone()));
        break;
      }
      case Instruction::Prop2: {
        stack.push(Term::Proved_(prop2.clone()));
        break;
      }
      case Instruction::Prop3: {
        stack.push(Term::Proved_(prop3.clone()));
        break;
      }
      case Instruction::ModusPonens: {
        auto premise2 = pop_stack_proved(stack);
        auto premise1 = pop_stack_proved(stack);

        if (premise1->inst != Instruction::Implication) {
#if DEBUG
          throw std::runtime_error("Modus Ponens: expected implication on the "
                                   "stack, got: " +
                                   std::to_string((int)premise1->inst));
#endif
          exit(1);
        }

        if (*premise1->left != *premise2) {
#if DEBUG
          throw std::runtime_error(
              "Antecedents do not match for modus ponens.\n" +
              std::to_string((int)premise1->left->inst) + "\n" +
              std::to_string((int)premise2->inst));

#endif
          exit(1);
        }
        stack.push(Term::Proved_(premise1->right.clone()));
        break;
      }

      case Instruction::Quantifier:
        stack.push(Term::Proved_(quantifier.clone()));
        break;

      case Instruction::Generalization: {
        auto proved_pat = pop_stack_proved(stack);

        if (proved_pat->inst == Instruction::Implication) {
          auto evar_id = iterator;
          iterator++;
          if (evar_id == buffer.end()) {
#if DEBUG
            throw std::runtime_error(
                "Insufficient parameters for Generalization instruction");
#endif
            exit(1);
          }

          if (!proved_pat->right->pattern_e_fresh(*evar_id)) {
#if DEBUG
            throw std::runtime_error(
                "The binding variable has to be fresh in the conclusion.");
#endif
            exit(1);
          }

          stack.push(Term::Proved_(implies(
              exists(static_cast<Id>(*evar_id), proved_pat->left.clone()),
              proved_pat->right.clone())));
        } else {
#if DEBUG
          throw std::runtime_error(
              "Expected an implication as a first parameter.");
#endif
          exit(1);
        }
        break;
      }

      case Instruction::Existence:
        stack.push(Term::Proved_(existence.clone()));
        break;

      case Instruction::Substitution: {
        auto svar_id = iterator;
        iterator++;
        if (svar_id == buffer.end()) {
#if DEBUG
          throw std::runtime_error(
              "Insufficient parameters for Substitution instruction");
#endif
          exit(1);
        }

        auto plug = pop_stack_pattern(stack);
        auto pattern = pop_stack_proved(stack);
        Instruction pattern_inst = pattern->inst;
        if (!(pattern_inst == Instruction::MetaVar ||
              pattern_inst == Instruction::ESubst ||
              pattern_inst == Instruction::SSubst)) {
#if DEBUG
          throw std::runtime_error("Cannot apply SSubst on concrete term!");
#endif
          exit(1);
        }

        auto ssubst_pat =
            ssubst(pattern.clone(), static_cast<Id>(*svar_id), plug);
        if (!ssubst_pat->pattern_well_formed()) {
          // The substitution is redundant, we don't apply it.
          stack.push(Term::Proved_(pattern));
        } else {
          stack.push(Term::Proved_(ssubst_pat));
        }
        break;
      }

      case Instruction::Instantiate: {
        auto n = iterator;
        iterator++;
        if (n == buffer.end()) {
#if DEBUG
          throw std::runtime_error(
              "Insufficient parameters for Instantiate instruction");
#endif
          exit(1);
        }
        auto ids = IdList();
        auto plugs = LinkedList<Rc<Pattern>>();

        Term metaterm = pop_stack(stack);
        for (int i = 0; i < static_cast<int>(*n); i++) {
          ids.push(static_cast<Id>(*iterator));
          iterator++;
          plugs.push(pop_stack_pattern(stack));
        }
        instantiate_in_place(metaterm.pattern, ids, plugs);
        if (metaterm.type == Term::Type::Pattern) {
          stack.push(Term::Pattern_(metaterm.pattern));
        } else if (metaterm.type == Term::Type::Proved) {
          stack.push(Term::Proved_(metaterm.pattern));
        } else {
#if DEBUG
          throw std::runtime_error("Instantiate needs a term on the stack");
#endif
          exit(1);
        }
        break;
      }
      case Instruction::Pop: {
        stack.pop().pattern.release();
        break;
      }
      case Instruction::Save: {
        Term term = stack.front();
        if (term.type == Term::Type::Pattern) {
          memory.push_back(Term::Pattern_(term.pattern.clone()));
        } else if (term.type == Term::Type::Proved) {
          memory.push_back(Term::Proved_(term.pattern.clone()));
        } else {
#if DEBUG
          throw std::runtime_error("Save needs an Term on the stack");
#endif
          exit(1);
        }
        break;
      }
      case Instruction::Load: {
        auto index = iterator;
        iterator++;
        if (index == buffer.end()) {
#if DEBUG
          throw std::runtime_error(
              "Insufficient parameters for Load instruction");
#endif
          exit(1);
        }
        Term term = memory.get(static_cast<int>(*index));
        if (term.type == Term::Type::Pattern) {
          stack.push(Term::Pattern_(term.pattern.clone()));
        } else if (term.type == Term::Type::Proved) {
          stack.push(Term::Proved_(term.pattern.clone()));
        } else {
#if DEBUG
          throw std::runtime_error("Load needs an Term in memory");
#endif
          exit(1);
        }
        break;
      }
      case Instruction::Publish: {
        switch (phase) {
        case ExecutionPhase::Gamma:
          memory.push_back(Term::Proved_(pop_stack_pattern(stack)));
          break;
        case ExecutionPhase::Claim:
          claims.push_back(pop_stack_pattern(stack));
          break;
        case ExecutionPhase::Proof: {
          auto claim = claims.pop();
          if (claim == nullptr) {
#if DEBUG
            throw std::runtime_error("Insufficient claims.");
#endif
            exit(1);
          }
          auto theorem = pop_stack_proved(stack);
          if (claim != theorem) {
#if DEBUG
            throw std::runtime_error(
                "This proof does not prove the requested claim: " +
                std::to_string((int)claim->inst) +
                ", theorem: " + std::to_string((int)theorem->inst));
#endif
            exit(1);
          }
          claim.release();
          break;
        }
        }
        break;
      }
      case Instruction::CleanMetaVar: {
        auto id = iterator;
        iterator++;
        if (id == buffer.end()) {
#if DEBUG
          throw std::runtime_error("Expected id for MetaVar instruction");
#endif
          exit(8);
        }
        auto metavar_pat = Pattern::metavar_unconstrained(static_cast<Id>(*id));

        // Clean metavars are always well-formed
        stack.push(Term::Pattern_(metavar_pat));
        break;
      }
      case Instruction::NO_OP: {
        iterator = buffer.end();
        break;
      }
      default: {
#if DEBUG
        throw std::runtime_error("Unknown instruction: " +
                                 std::to_string((int)instr_u32));
#endif
        exit(1);
      }
      }
#if DEBUG
      // printStack(stack);
#endif
    }
  }

  static int verify(std::array<int, MAX_SIZE> &gamma_buffer,
                    std::array<int, MAX_SIZE> &claims_buffer,
                    std::array<int, MAX_SIZE> &proof_buffer) noexcept {
    auto claims = Claims();
    auto memory = Memory();
    auto stack = Stack();

    execute_instructions(gamma_buffer,
                         stack,  // stack is empty initially.
                         memory, // memory is empty initially.
                         claims, // claims is unused in this phase.
                         ExecutionPhase::Gamma);

    stack.clear();

    execute_instructions(claims_buffer,
                         stack,  // stack is empty initially.
                         memory, // reuse memory
                         claims, // claims populated in this phase
                         ExecutionPhase::Claim);

    stack.clear();

    execute_instructions(proof_buffer,
                         stack,  // stack is empty initially.
                         memory, // axioms are used as initial memory
                         claims, // claims are consumed by publish instruction
                         ExecutionPhase::Proof);
    if (!claims.empty()) {
#if DEBUG
      std::cout << "Checking finished but there are claims left unproved:"
                << std::endl;
#endif
      return 1;
    } else {
#if DEBUG
      std::cout << "Checking finished and all claims are proved." << std::endl;
#endif
      return 0;
    }

    return 0;
  }
};
