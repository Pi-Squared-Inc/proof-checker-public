use ml_checker_cairo::pattern;
use ml_checker_cairo::term::Term;
use ml_checker_cairo::stack::{StackStructure, StackTrait, ClaimTrait};

use core::option::Option::{None, Some};

use pattern::Pattern;
use pattern::Pattern::{EVar, SVar, Symbol, Implies, App, Exists, Mu, MetaVar, ESubst, SSubst};
use pattern::{
    Id, evar, svar, symbol, implies, app, exists, mu, metavar, metavar_unconstrained,
    metavar_e_fresh, metavar_s_fresh, esubst, ssubst
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
    let _prop1 = implies(phi0.clone(), implies(phi1.clone(), phi0.clone()));

    let _prop2 = implies(
        implies(phi0.clone(), implies(phi1.clone(), phi2.clone())),
        implies(implies(phi0.clone(), phi1), implies(phi0.clone(), phi2))
    );
    let _prop3 = implies(not(not(phi0.clone())), phi0.clone());
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
                    Instruction::Implies => { panic!("Implies not implemented!"); },
                    Instruction::App => { panic!("App not implemented!"); },
                    Instruction::Exists => { panic!("Exists not implemented!"); },
                    Instruction::Mu => { panic!("Mu not implemented!"); },
                    Instruction::MetaVar => { panic!("MetaVar not implemented!"); },
                    Instruction::ESubst => { panic!("ESubst not implemented!"); },
                    Instruction::SSubst => { panic!("SSubst not implemented!"); },
                    Instruction::Prop1 => { panic!("Prop1 not implemented!"); },
                    Instruction::Prop2 => { panic!("Prop2 not implemented!"); },
                    Instruction::Prop3 => { panic!("Prop3 not implemented!"); },
                    Instruction::Quantifier => { panic!("Quantifier not implemented!"); },
                    Instruction::PropagationOr => { panic!("PropagationOr not implemented!"); },
                    Instruction::PropagationExists => {
                        panic!("PropagationExists not implemented!");
                    },
                    Instruction::PreFixpoint => { panic!("PreFixpoint not implemented!"); },
                    Instruction::Existence => { panic!("Existence not implemented!"); },
                    Instruction::Singleton => { panic!("Singleton not implemented!"); },
                    Instruction::ModusPonens => { panic!("ModusPonens not implemented!"); },
                    Instruction::Generalization => { panic!("Generalization not implemented!"); },
                    Instruction::Frame => { panic!("Frame not implemented!"); },
                    Instruction::Substitution => { panic!("Substitution not implemented!"); },
                    Instruction::KnasterTarski => { panic!("KnasterTarski not implemented!"); },
                    Instruction::Instantiate => { panic!("Instantiate not implemented!"); },
                    Instruction::Pop => { panic!("Pop not implemented!"); },
                    Instruction::Save => { panic!("Save not implemented!"); },
                    Instruction::Load => { panic!("Load not implemented!"); },
                    Instruction::Publish => { panic!("Publish not implemented!"); },
                    Instruction::CleanMetaVar => { panic!("NoOp not implemented!"); },
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
}

