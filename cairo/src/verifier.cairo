use core::traits::Into;
use core::traits::Destruct;
use core::array::SpanTrait;
use ml_checker_cairo::pattern::PatternTrait;
use core::clone::Clone;
use core::box::BoxTrait;
use core::array::ArrayTrait;
use core::option::OptionTrait;
use ml_checker_cairo::pattern;
use ml_checker_cairo::term::Term;
use ml_checker_cairo::stack::{StackStructure, StackTrait, ClaimTrait};

use core::option::Option::{None, Some};

use pattern::Pattern;
use pattern::Pattern::{EVar, SVar, Symbol, Implies, App, Exists, Mu, MetaVar, ESubst, SSubst};
use pattern::{
    evar, svar, symbol, implies, app, exists, mu, metavar, metavar_unconstrained, metavar_e_fresh,
    metavar_s_fresh, esubst, ssubst
};
use pattern::{
    Id, IdList, ImpliesType, AppType, ExistsType, MuType, ESubstType, SSubstType, MetaVarType
};

// Instructions
// ============
//
// Instructions are used to define the on-the-wire representation of matching
// logic proofs.

#[derive(Debug, Eq, PartialEq)]
enum Instruction {
    // Patterns
    EVar,
    SVar,
    Symbol,
    Implies,
    App,
    Exists,
    Mu,
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
    CleanMetaVar,
    // NoOp to fill Cairo match requirements
    NoOp
}

type InstByte = u8;

#[derive(Debug, Eq, PartialEq)]
fn from(value: felt252) -> Instruction {
    match value {
        0 => Instruction::NoOp,
        1 => Instruction::NoOp,
        2 => Instruction::EVar,
        3 => Instruction::SVar,
        4 => Instruction::Symbol,
        5 => Instruction::Implies,
        6 => Instruction::App,
        7 => Instruction::Mu,
        8 => Instruction::Exists,
        9 => Instruction::MetaVar,
        10 => Instruction::ESubst,
        11 => Instruction::SSubst,
        12 => Instruction::Prop1,
        13 => Instruction::Prop2,
        14 => Instruction::Prop3,
        15 => Instruction::Quantifier,
        16 => Instruction::PropagationOr,
        17 => Instruction::PropagationExists,
        18 => Instruction::PreFixpoint,
        19 => Instruction::Existence,
        20 => Instruction::Singleton,
        21 => Instruction::ModusPonens,
        22 => Instruction::Generalization,
        23 => Instruction::Frame,
        24 => Instruction::Substitution,
        25 => Instruction::KnasterTarski,
        26 => Instruction::Instantiate,
        27 => Instruction::Pop,
        28 => Instruction::Save,
        29 => Instruction::Load,
        30 => Instruction::Publish,
        31 => Instruction::NoOp,
        32 => Instruction::NoOp,
        33 => Instruction::NoOp,
        34 => Instruction::NoOp,
        35 => Instruction::NoOp,
        36 => Instruction::NoOp,
        37 => Instruction::NoOp,
        38 => Instruction::NoOp,
        39 => Instruction::NoOp,
        40 => Instruction::NoOp,
        41 => Instruction::NoOp,
        42 => Instruction::NoOp,
        43 => Instruction::NoOp,
        44 => Instruction::NoOp,
        45 => Instruction::NoOp,
        46 => Instruction::NoOp,
        47 => Instruction::NoOp,
        48 => Instruction::NoOp,
        49 => Instruction::NoOp,
        50 => Instruction::NoOp,
        51 => Instruction::NoOp,
        52 => Instruction::NoOp,
        53 => Instruction::NoOp,
        54 => Instruction::NoOp,
        55 => Instruction::NoOp,
        56 => Instruction::NoOp,
        57 => Instruction::NoOp,
        58 => Instruction::NoOp,
        59 => Instruction::NoOp,
        60 => Instruction::NoOp,
        61 => Instruction::NoOp,
        62 => Instruction::NoOp,
        63 => Instruction::NoOp,
        64 => Instruction::NoOp,
        65 => Instruction::NoOp,
        66 => Instruction::NoOp,
        67 => Instruction::NoOp,
        68 => Instruction::NoOp,
        69 => Instruction::NoOp,
        70 => Instruction::NoOp,
        71 => Instruction::NoOp,
        72 => Instruction::NoOp,
        73 => Instruction::NoOp,
        74 => Instruction::NoOp,
        75 => Instruction::NoOp,
        76 => Instruction::NoOp,
        77 => Instruction::NoOp,
        78 => Instruction::NoOp,
        79 => Instruction::NoOp,
        80 => Instruction::NoOp,
        81 => Instruction::NoOp,
        82 => Instruction::NoOp,
        83 => Instruction::NoOp,
        84 => Instruction::NoOp,
        85 => Instruction::NoOp,
        86 => Instruction::NoOp,
        87 => Instruction::NoOp,
        88 => Instruction::NoOp,
        89 => Instruction::NoOp,
        90 => Instruction::NoOp,
        91 => Instruction::NoOp,
        92 => Instruction::NoOp,
        93 => Instruction::NoOp,
        94 => Instruction::NoOp,
        95 => Instruction::NoOp,
        96 => Instruction::NoOp,
        97 => Instruction::NoOp,
        98 => Instruction::NoOp,
        99 => Instruction::NoOp,
        100 => Instruction::NoOp,
        101 => Instruction::NoOp,
        102 => Instruction::NoOp,
        103 => Instruction::NoOp,
        104 => Instruction::NoOp,
        105 => Instruction::NoOp,
        106 => Instruction::NoOp,
        107 => Instruction::NoOp,
        108 => Instruction::NoOp,
        109 => Instruction::NoOp,
        110 => Instruction::NoOp,
        111 => Instruction::NoOp,
        112 => Instruction::NoOp,
        113 => Instruction::NoOp,
        114 => Instruction::NoOp,
        115 => Instruction::NoOp,
        116 => Instruction::NoOp,
        117 => Instruction::NoOp,
        118 => Instruction::NoOp,
        119 => Instruction::NoOp,
        120 => Instruction::NoOp,
        121 => Instruction::NoOp,
        122 => Instruction::NoOp,
        123 => Instruction::NoOp,
        124 => Instruction::NoOp,
        125 => Instruction::NoOp,
        126 => Instruction::NoOp,
        127 => Instruction::NoOp,
        128 => Instruction::NoOp,
        129 => Instruction::NoOp,
        130 => Instruction::NoOp,
        131 => Instruction::NoOp,
        132 => Instruction::NoOp,
        133 => Instruction::NoOp,
        134 => Instruction::NoOp,
        135 => Instruction::NoOp,
        136 => Instruction::NoOp,
        137 => Instruction::CleanMetaVar,
        _ => panic!("Bad Instruction!"),
    }
}

// Notation
#[inline(always)]
fn bot() -> Pattern {
    return mu(0, svar(0));
}

#[inline(always)]
fn not(pat: Pattern) -> Pattern {
    return implies(pat, bot());
}

fn forall(evar: Id, pat: Pattern) -> Pattern {
    return not(exists(evar, not(pat)));
}

fn instantiate_internal(
    ref p: Pattern, ref vars: IdList, ref plugs: Array<Pattern>,
) -> Option<Pattern> {
    match p.clone() {
        Pattern::EVar(_) => Option::None,
        Pattern::SVar(_) => Option::None,
        Pattern::Symbol(_) => Option::None,
        Pattern::Implies(ImpliesType{left,
        right }) => {
            let mut left = left.unwrap().unbox();
            let mut right = right.unwrap().unbox();

            let mut inst_left = instantiate_internal(ref left, ref vars, ref plugs);
            let mut inst_right = instantiate_internal(ref right, ref vars, ref plugs);

            if inst_left.is_none() && inst_right.is_none() {
                return Option::None;
            } else {
                if inst_left.is_none() {
                    inst_left = Option::Some(left.clone());
                }
                if inst_right.is_none() {
                    inst_right = Option::Some(right.clone());
                }
                return Option::Some(implies(inst_left.unwrap(), inst_right.unwrap()));
            }
        },
        Pattern::App(_) => Option::None,
        Pattern::Exists(_) => Option::None,
        Pattern::Mu(_) => Option::None,
        Pattern::MetaVar(MetaVarType{id,
        e_fresh,
        s_fresh,
        positive,
        negative,
        app_ctx_holes }) => {
            let mut pos: u32 = 0;
            let mut e_fresh = e_fresh.clone();
            let mut s_fresh = s_fresh.clone();
            let mut negative = negative.clone();
            let mut positive = positive.clone();
            let mut _app_ctx_holes = app_ctx_holes; // To supress warning
            let mut ret: Option<Pattern> = Option::None;
            let plugs_as_ref: @Array<Pattern> = @plugs;
            let mut vars_clone = vars.clone();
            loop {
                match vars_clone.pop_front() {
                    Option::Some(var) => {
                        if var == id {
                            loop {
                                match e_fresh.pop_front() {
                                    Option::Some(evar) => {
                                        let plug: Pattern = plugs_as_ref.at(pos).clone();
                                        if !plug.e_fresh(evar) {
                                            panic!(
                                                "Instantiation of MetaVar {} breaks a freshness constraint: EVar {}",
                                                id,
                                                evar
                                            );
                                        }
                                    },
                                    Option::None => {}
                                }
                                match s_fresh.pop_front() {
                                    Option::Some(svar) => {
                                        let plug: Pattern = plugs_as_ref.at(pos).clone();
                                        if !plug.s_fresh(svar) {
                                            panic!(
                                                "Instantiation of MetaVar {} breaks a freshness constraint: SVar {}",
                                                id,
                                                svar
                                            );
                                        }
                                    },
                                    Option::None => {}
                                }
                                match positive.pop_front() {
                                    Option::Some(svar) => {
                                        let plug: Pattern = plugs_as_ref.at(pos).clone();
                                        if !plug.positive(svar) {
                                            panic!(
                                                "Instantiation of MetaVar {} breaks a positivity constraint: SVar {:?}",
                                                id,
                                                svar
                                            );
                                        }
                                    },
                                    Option::None => {}
                                }
                                match negative.pop_front() {
                                    Option::Some(svar) => {
                                        let plug: Pattern = plugs_as_ref.at(pos).clone();
                                        if !plug.negative(svar) {
                                            panic!(
                                                "Instantiation of MetaVar {} breaks a negativity constraint: SVar {:?}",
                                                id,
                                                svar
                                            );
                                        }
                                    },
                                    Option::None => {}
                                }

                                if pos >= plugs_as_ref.len() {
                                    panic!("Substitution does not contain a corresponding value.")
                                }

                                ret = Option::Some(plugs_as_ref.at(pos).clone());
                                break;
                            }
                        }
                        pos += 1;
                    },
                    Option::None => { break; }
                }
            };
            return ret;
        },
        Pattern::ESubst(_) => Option::None,
        Pattern::SSubst(_) => Option::None,
    }
}

fn instantiate_in_place(ref p: Pattern, ref ids: IdList, ref plugs: Array<Pattern>) {
    let option_ret = instantiate_internal(ref p, ref ids, ref plugs);
    match option_ret {
        Option::Some(ret) => { p = ret; },
        Option::None => { panic!("Instantiation failed!"); },
    }
}

/// Proof checker
/// =============

type Stack = StackStructure<Term>;
type Claims = StackStructure<Pattern>;
type Memory = Array<Term>;

/// Stack manipulation
/// ------------------
#[inline(always)]
fn pop_stack(ref stack: Stack) -> Term {
    return stack.pop();
}

fn pop_stack_pattern(ref stack: Stack) -> Pattern {
    let term = pop_stack(ref stack);
    match term {
        Term::Pattern(pat) => { return pat; },
        Term::Proved(_) => panic!("Expected pattern on stack."),
    }
}

fn pop_stack_proved(ref stack: Stack) -> Pattern {
    let term = pop_stack(ref stack);
    match term {
        Term::Pattern(_) => panic!("Expected proved on stack."),
        Term::Proved(pat) => { return pat; },
    }
}

/// Main implementation
/// -------------------

#[derive(Drop, Copy)]
enum ExecutionPhase {
    Gamma,
    Claim,
    Proof,
}

fn read_u8_vec(ref buffer: Array<u8>) -> Array<u8> {
    let mut result = array![];
    let mut i = 0;
    let len: u8 = buffer.pop_front().expect('Expected length for array');
    loop {
        if i == len {
            break;
        }
        result.append(buffer.pop_front().unwrap());
        i += 1;
    };
    return result;
}

fn execute_instructions(
    mut buffer: Array<u8>,
    ref stack: Stack,
    ref memory: Memory,
    ref claims: Claims,
    phase: ExecutionPhase,
) {
    // Metavars
    let phi0 = metavar_unconstrained(0);
    let phi1 = metavar_unconstrained(1);
    let phi2 = metavar_unconstrained(2);

    // Axioms
    let prop1 = implies(phi0.clone(), implies(phi1.clone(), phi0.clone()));

    let prop2 = implies(
        implies(phi0.clone(), implies(phi1.clone(), phi2.clone())),
        implies(implies(phi0.clone(), phi1), implies(phi0.clone(), phi2))
    );
    let prop3 = implies(not(not(phi0.clone())), phi0.clone());
    let _quantifier = implies(esubst(phi0.clone(), 0, evar(1)), exists(0, phi0));

    let _existence = exists(0, evar(0));

    // For enums we must implement all cases to make match works
    loop {
        match (buffer.pop_front()) {
            Some(inst) => {
                let inst_felt252 = U8IntoFelt252::into(inst);
                match from(inst_felt252) {
                    Instruction::EVar => { panic!("EVar not implemented!"); },
                    Instruction::SVar => { panic!("SVar not implemented!"); },
                    Instruction::Symbol => { panic!("Symbol not implemented!"); },
                    Instruction::Implies => {
                        let left = pop_stack_pattern(ref stack);
                        let right = pop_stack_pattern(ref stack);
                        stack.push(Term::Pattern(implies(left, right)));
                    },
                    Instruction::App => { panic!("App not implemented!"); },
                    Instruction::Exists => { panic!("Exists not implemented!"); },
                    Instruction::Mu => { panic!("Mu not implemented!"); },
                    Instruction::MetaVar => { panic!("MetaVar not implemented!"); },
                    Instruction::ESubst => { panic!("ESubst not implemented!"); },
                    Instruction::SSubst => { panic!("SSubst not implemented!"); },
                    Instruction::Prop1 => stack.push(Term::Proved(prop1.clone())),
                    Instruction::Prop2 => stack.push(Term::Proved(prop2.clone())),
                    Instruction::Prop3 => stack.push(Term::Proved(prop3.clone())),
                    Instruction::Quantifier => { panic!("Quantifier not implemented!"); },
                    Instruction::PropagationOr => { panic!("PropagationOr not implemented!"); },
                    Instruction::PropagationExists => {
                        panic!("PropagationExists not implemented!");
                    },
                    Instruction::PreFixpoint => { panic!("PreFixpoint not implemented!"); },
                    Instruction::Existence => { panic!("Existence not implemented!"); },
                    Instruction::Singleton => { panic!("Singleton not implemented!"); },
                    Instruction::ModusPonens => {
                        let premise2 = pop_stack_proved(ref stack);
                        let premise1: Pattern = pop_stack_proved(ref stack);
                        match premise1 {
                            Pattern::Implies(ImpliesType{left,
                            right }) => {
                                let left = left.unwrap().unbox();
                                if left != premise2 {
                                    panic!(
                                        "Antecedents do not match for modus ponens.\nleft.psi:\n{:?}\n\n right:\n{:?}\n",
                                        left,
                                        premise2
                                    );
                                }
                                stack.push(Term::Proved(right.unwrap().unbox().clone()))
                            },
                            Pattern::EVar(_) => panic!(
                                "Expected an implication as a first parameter."
                            ),
                            Pattern::SVar(_) => panic!(
                                "Expected an implication as a first parameter."
                            ),
                            Pattern::Symbol(_) => panic!(
                                "Expected an implication as a first parameter."
                            ),
                            Pattern::App(_) => panic!(
                                "Expected an implication as a first parameter."
                            ),
                            Pattern::Exists(_) => panic!(
                                "Expected an implication as a first parameter."
                            ),
                            Pattern::Mu(_) => panic!(
                                "Expected an implication as a first parameter."
                            ),
                            Pattern::MetaVar(_) => panic!(
                                "Expected an implication as a first parameter."
                            ),
                            Pattern::ESubst(_) => panic!(
                                "Expected an implication as a first parameter."
                            ),
                            Pattern::SSubst(_) => panic!(
                                "Expected an implication as a first parameter."
                            ),
                        };
                    },
                    Instruction::Generalization => { panic!("Generalization not implemented!"); },
                    Instruction::Frame => { panic!("Frame not implemented!"); },
                    Instruction::Substitution => { panic!("Substitution not implemented!"); },
                    Instruction::KnasterTarski => { panic!("KnasterTarski not implemented!"); },
                    Instruction::Instantiate => {
                        let n: u8 = buffer.pop_front().expect('Insufficient parms Instantiate');
                        let mut ids: IdList = ArrayTrait::new();
                        let mut plugs: Array<Pattern> = ArrayTrait::new();

                        let metaterm = pop_stack(ref stack);

                        let mut i = 0;
                        loop {
                            if i == n {
                                break;
                            }
                            ids
                                .append(
                                    buffer.pop_front().expect('Expected id for Instantiate').into()
                                );
                            plugs.append(pop_stack_pattern(ref stack));
                            i += 1;
                        };

                        match metaterm {
                            Term::Pattern(mut p) => {
                                instantiate_in_place(ref p, ref ids, ref plugs);
                                stack.push(Term::Pattern(p));
                            },
                            Term::Proved(mut p) => {
                                instantiate_in_place(ref p, ref ids, ref plugs);
                                stack.push(Term::Proved(p));
                            }
                        }
                    },
                    Instruction::Pop => { let _ = pop_stack(ref stack); },
                    Instruction::Save => memory
                        .append(stack.last().expect('Save needs term on the stack').clone()),
                    Instruction::Load => {
                        let index: u32 = buffer
                            .pop_front()
                            .expect('Error on Load instruction')
                            .into();
                        stack
                            .push(
                                memory
                                    .get(index)
                                    .expect('Load needs term on Memory')
                                    .unbox()
                                    .clone()
                            );
                    },
                    Instruction::Publish => match phase {
                        ExecutionPhase::Gamma => memory
                            .append(Term::Proved(pop_stack_pattern(ref stack))),
                        ExecutionPhase::Claim => {
                            let claim = pop_stack_pattern(ref stack);
                            claims.push(claim)
                        },
                        ExecutionPhase::Proof => {
                            let claim = claims.pop();
                            let theorem = pop_stack_proved(ref stack);
                            // Missing PartialEq for Terms and Pattern
                            if claim != theorem {
                                panic!(
                                    "This proof does not prove the requested claim: {:?}, theorem: {:?}",
                                    claim,
                                    theorem
                                );
                            }
                        },
                    },
                    Instruction::CleanMetaVar => {
                        let id: Id = buffer.pop_front().expect('Expected id for MetaVar').into();

                        let metavar_pat = metavar_unconstrained(id);

                        // Clean metavars are always well-formed
                        stack.push(Term::Pattern(metavar_pat));
                    },
                    Instruction::NoOp => { panic!("NoOp not implemented!"); },
                }
            },
            Option::None => { break; }
        }
    }
}

fn verify(
    gamma_buffer: Array<InstByte>, claims_buffer: Array<InstByte>, proof_buffer: Array<InstByte>
) {
    let mut stack: Stack = StackTrait::new();
    let mut memory: Memory = array![];
    let mut claims: Claims = ClaimTrait::new();
    execute_instructions(
        gamma_buffer,
        ref stack, // stack is empty initially.
        ref memory, // memory is empty initially.
        ref claims, // claims is unused in this phase.
        ExecutionPhase::Gamma
    );

    stack.clear();

    execute_instructions(
        claims_buffer,
        ref stack, // stack is empty initially.
        ref memory, // reuse memory.
        ref claims, // claims populated in this phase.
        ExecutionPhase::Claim
    );

    stack.clear();

    execute_instructions(
        proof_buffer,
        ref stack, // stack is empty initially.
        ref memory, // axioms are used as initial memory.
        ref claims, // claims are consumed by publish instruction.
        ExecutionPhase::Proof
    );

    assert(claims.is_empty(), 'Claims should be empty!');
}

// Unit tests module
#[cfg(test)]
mod tests {
    use core::clone::Clone;
    use core::traits::Into;
    use core::box::BoxTrait;
    use core::option::OptionTrait;
    use core::array::ArrayTrait;
    use super::verify;

    #[test]
    #[available_gas(1000000000000000)]
    fn it_works() {
        let mut gamma = ArrayTrait::<u8>::new();
        let mut claims = ArrayTrait::<u8>::new();
        let mut proofs = ArrayTrait::<u8>::new();

        verify(gamma, claims, proofs);
    }

    use super::StackTrait;
    use super::Term;
    use super::bot;
    use super::Stack;
    #[test]
    #[available_gas(1000000000000000)]
    fn test_stack_push() {
        let mut stack: Stack = StackTrait::new();
        let term = Term::Pattern(bot());
        stack.push(term);
        assert(stack.len() == 1, 'Hmm.. stack_size should be 1!');
    }

    use super::pop_stack;
    #[test]
    #[available_gas(1000000000000000)]
    fn test_stack_pop() {
        let mut stack: Stack = StackTrait::new();
        let term = Term::Pattern(bot());
        stack.push(term.clone());
        let pop_term = pop_stack(ref stack);
        assert(stack.is_empty(), 'Hmm.. stack_size should be 0!');
        assert(pop_term == term, 'Hmm.. pop_term should be term!');
    }

    use super::pop_stack_pattern;
    #[test]
    #[available_gas(1000000000000000)]
    fn test_stack_pop_pattern() {
        let mut stack: Stack = StackTrait::new();
        let pat = bot();
        stack.push(Term::Pattern(pat.clone()));
        let pop_term = pop_stack_pattern(ref stack);
        assert(stack.is_empty(), 'Hmm.. stack_size should be 0!');
        // This test ins't possible yet because of the lack of equality
        assert(pop_term == pat, 'Hmm.. pop_term should be term!');
    }

    use super::pop_stack_proved;
    #[test]
    #[available_gas(1000000000000000)]
    fn test_stack_pop_proved() {
        let mut stack: Stack = StackTrait::new();
        let pat = bot();
        stack.push(Term::Proved(pat.clone()));
        let pop_term = pop_stack_proved(ref stack);
        assert(stack.is_empty(), 'Hmm.. stack_size should be 0!');
        // This test ins't possible yet because of the lack of equality
        assert(pop_term == pat, 'Hmm.. pop_term should be term!');
    }

    use super::read_u8_vec;
    #[test]
    #[available_gas(1000000000000000)]
    fn test_read_u8_vec() {
        let mut buffer = array![];
        buffer.append(3);
        buffer.append(1);
        buffer.append(2);
        buffer.append(3);
        let result = read_u8_vec(ref buffer);
        assert(result.len() == 3, 'Hmm this should have length 3!');
    }

    use super::implies;
    use super::metavar_unconstrained;
    use super::InstByte;
    use super::Instruction;
    use super::execute_instructions;
    use super::ExecutionPhase;
    use super::ClaimTrait;
    use super::Memory;
    use super::Claims;
    #[test]
    #[available_gas(1000000000000000)]
    fn test_construct_phi_implies_phi() {
        let proof: Array<InstByte> = array![
            137,
            0, // CleanMetaVar
            137,
            0, // CleanMetaVar
            5, // Implies 
            28, // Save 
            30 // Publish
        ];

        let mut stack: Stack = StackTrait::new();
        let mut memory: Array<Term> = array![];
        let mut claims: Claims = ClaimTrait::new();

        let phi = metavar_unconstrained(0);
        let phi_implies_phi = implies(phi.clone(), phi.clone());

        execute_instructions(proof, ref stack, ref memory, ref claims, ExecutionPhase::Gamma);
        let mut pattern_phi_implies_phi = memory.pop_front().expect('Expected memory element');
        assert(
            pattern_phi_implies_phi == Term::Pattern(phi_implies_phi.clone()),
            'Expect pattern::phi_implies_phi'
        );
        let proved_phi_implies_phi = memory.pop_front().expect('Expected memory element');
        assert(
            proved_phi_implies_phi == Term::Proved(phi_implies_phi),
            'Expect proved::phi_implies_phi'
        );
    }

    #[test]
    #[available_gas(1000000000000000)]
    fn test_save_and_load() {
        let proof: Array<InstByte> = array![137, 0, // CleanMetaVar
         28, // Save 
        ];

        let mut stack: Stack = StackTrait::new();
        let mut memory: Memory = array![];
        let mut claims: Claims = ClaimTrait::new();

        let phi = metavar_unconstrained(0);

        execute_instructions(proof, ref stack, ref memory, ref claims, ExecutionPhase::Gamma);
        let mut pattern_phi: Term = memory.get(0).expect('Expected memory element').unbox().clone();
        assert(pattern_phi == Term::Pattern(phi.clone()), 'Expect pattern::phi');
        execute_instructions(
            array![29, 0], ref stack, ref memory, ref claims, ExecutionPhase::Gamma
        );
        pattern_phi = stack.pop();
        assert(pattern_phi == Term::Pattern(phi.clone()), 'Expect pattern::phi');
    }

    use super::evar;
    use super::instantiate_internal;
    use super::Pattern;
    #[test]
    #[available_gas(1000000000000000)]
    fn test_instantiate() {
        let x0 = evar(0);
        let x0_implies_x0 = implies(x0.clone(), x0.clone());

        let phi0 = metavar_unconstrained(0);
        let mut phi0_implies_phi0: Pattern = implies(phi0.clone(), phi0.clone());

        let mut vars0 = array![0];
        let mut plugs0 = array![x0.clone()];

        let expected = Option::Some(x0_implies_x0.clone());
        let result: Option<Pattern> = instantiate_internal(
            ref phi0_implies_phi0, ref vars0, ref plugs0
        );
        assert(expected == result, 'Expect x0_implies_x0');

        let mut vars1 = array![1];
        let expected: Option<Pattern> = Option::None;
        let result = instantiate_internal(ref phi0_implies_phi0, ref vars1, ref plugs0);
        assert(expected == result, 'Expect None');
    }

    #[test]
    #[available_gas(1000000000000000)]
    fn test_impreflex_compress_goal() {
        let gamma_buffer: Array<InstByte> = array![];
        let claim_buffer: Array<InstByte> = array![137, 0, 137, 0, 5, 28, 30];
        let proof_buffer: Array<InstByte> = array![
            137,
            0,
            137,
            0,
            137,
            0,
            5,
            28,
            5,
            28,
            29,
            0,
            137,
            0,
            29,
            0,
            137,
            0,
            5,
            5,
            29,
            2,
            29,
            0,
            5,
            137,
            0,
            29,
            0,
            137,
            0,
            13,
            26,
            3,
            2,
            1,
            0,
            137,
            0,
            29,
            0,
            12,
            26,
            2,
            1,
            0,
            21,
            28,
            27,
            27,
            27,
            29,
            3,
            137,
            0,
            137,
            0,
            12,
            26,
            2,
            1,
            0,
            21,
            28,
            27,
            27,
            27,
            29,
            4,
            30
        ];

        verify(gamma_buffer, claim_buffer, proof_buffer);
    }
}
