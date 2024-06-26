#![deny(warnings)]
#![no_std]

extern crate alloc;

use alloc::boxed::Box;
use alloc::fmt;
use alloc::vec;
use alloc::vec::Vec;
use core::ops::Deref;

// Keep this version in sync with the one in Cargo.toml!
// We could read from Cargo.toml but want to avoid String.
const VERSION_MAJOR: u8 = 3;
const VERSION_MINOR: u8 = 0;

/// Instructions
/// ============
///
/// Instructions are used to define the on-the-wire representation of matching
/// logic proofs.

#[rustfmt::skip]
#[derive(Debug, Eq, PartialEq)]
pub enum Instruction {
    // Patterns
    Bot = 1, EVar, SVar, Symbol, Implies, App, Exists, Mu,
    // Meta Patterns,
    MetaVar, ESubst, SSubst,
    // Axiom Schemas,
    Prop1, Prop2, Prop3, Quantifier, PropagationOr, PropagationExists,
    PreFixpoint, Existence, Singleton,
    // Inference rules,
    ModusPonens, Generalization, Framing, Substitution, KnasterTarski,
    // Meta Incference rules,
    Instantiate,
    // Stack Manipulation,
    Pop,
    // Memory Manipulation,
    Save, Load,
    // Version Control
    Version,
    // Metavar with no constraints
    CleanMetaVar = (9 + 128),
    // Journal Manipulation,
    Publish = 255,
}

type InstByte = u8;
type InstrIterator<'a> = core::slice::Iter<'a, InstByte>;

impl Instruction {
    fn from(value: InstByte) -> Instruction {
        match value {
            1 => Instruction::Bot,
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
            23 => Instruction::Framing,
            24 => Instruction::Substitution,
            25 => Instruction::KnasterTarski,
            26 => Instruction::Instantiate,
            27 => Instruction::Pop,
            28 => Instruction::Save,
            29 => Instruction::Load,
            31 => Instruction::Version,
            // Sub-instructions
            137 => Instruction::CleanMetaVar,
            255 => Instruction::Publish,
            _ => panic!("Bad Instruction!"),
        }
    }
}

/// Terms
/// =====
///
/// Terms define the in-memory representation of matching logic patterns and proofs.
/// However, since we only implement a proof checker in this program we do not need
/// an explicit representation of the entire hilbert proof tree.
/// We only need to store the conclusion of things that are proved so far.
/// We use the `Proved` variant for this.

type Id = u8;
type IdList = Vec<Id>;

pub struct Ptr<T> {
    ptr: *const T,
}

impl<T> Ptr<T> {
    fn new(x: T) -> Self {
        Ptr {
            ptr: Box::leak(Box::new(x)),
        }
    }

    fn is(&self, other: &Self) -> bool {
        (*self).ptr == (*other).ptr
    }
}

impl<T> Copy for Ptr<T> {}
impl<T> Clone for Ptr<T> {
    fn clone(&self) -> Self {
        Ptr { ptr: self.ptr }
    }
}

impl<T: fmt::Debug> fmt::Debug for Ptr<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl<T: PartialEq> PartialEq for Ptr<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        PartialEq::eq(&**self, &**other)
    }
    #[inline]
    fn ne(&self, other: &Self) -> bool {
        PartialEq::ne(&**self, &**other)
    }
}

impl<T: Eq> Eq for Ptr<T> {}

impl<T> Deref for Ptr<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.ptr }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Pattern {
    Bot(),
    EVar(Id),
    SVar(Id),
    Symbol(Id),
    Implies {
        left: Ptr<Pattern>,
        right: Ptr<Pattern>,
    },
    App {
        left: Ptr<Pattern>,
        right: Ptr<Pattern>,
    },
    Exists {
        var: Id,
        subpattern: Ptr<Pattern>,
    },
    Mu {
        var: Id,
        subpattern: Ptr<Pattern>,
    },
    MetaVar {
        id: Id,
        e_fresh: IdList,
        s_fresh: IdList,
        positive: IdList,
        negative: IdList,
        app_ctx_holes: IdList,
    },
    ESubst {
        pattern: Ptr<Pattern>,
        evar_id: Id,
        plug: Ptr<Pattern>,
    },
    SSubst {
        pattern: Ptr<Pattern>,
        svar_id: Id,
        plug: Ptr<Pattern>,
    },
}

impl Pattern {
    fn e_fresh(&self, evar: Id) -> bool {
        match self {
            Pattern::Bot() => true,
            Pattern::EVar(name) => *name != evar,
            Pattern::SVar(_) => true,
            Pattern::Symbol(_) => true,
            Pattern::MetaVar { e_fresh, .. } => e_fresh.contains(&evar),
            Pattern::Implies { left, right } => left.e_fresh(evar) && right.e_fresh(evar),
            Pattern::App { left, right } => left.e_fresh(evar) && right.e_fresh(evar),
            Pattern::Exists { var, subpattern } => evar == *var || subpattern.e_fresh(evar),
            Pattern::Mu { subpattern, .. } => subpattern.e_fresh(evar),
            Pattern::ESubst {
                pattern,
                evar_id,
                plug,
            } => {
                // Assume: substitution is well-formed => plug occurs in the result

                if evar == *evar_id {
                    // Freshness depends only on plug, as all the free instances
                    // of the requested variable are being substituted
                    return plug.e_fresh(evar);
                }

                // Freshness depends on both input and plug,
                // as evar != evar_id (note that instances of evar_id
                // in pattern do not influence the result)
                pattern.e_fresh(evar) && plug.e_fresh(evar)
            }
            Pattern::SSubst { pattern, plug, .. } => {
                // Assume: substitution is well-formed => plug occurs in the result

                // We can skip checking evar == svar_id, because different types

                // Freshness depends on both input and plug,
                // as svar_id != evar (note that instances of evar_id
                // in pattern do not influence the result)
                pattern.e_fresh(evar) && plug.e_fresh(evar)
            }
        }
    }

    fn s_fresh(&self, svar: Id) -> bool {
        match self {
            Pattern::Bot() => true,
            Pattern::EVar(_) => true,
            Pattern::SVar(name) => *name != svar,
            Pattern::Symbol(_) => true,
            Pattern::MetaVar { s_fresh, .. } => s_fresh.contains(&svar),
            Pattern::Implies { left, right } => left.s_fresh(svar) && right.s_fresh(svar),
            Pattern::App { left, right } => left.s_fresh(svar) && right.s_fresh(svar),
            Pattern::Exists { subpattern, .. } => subpattern.s_fresh(svar),
            Pattern::Mu { var, subpattern } => svar == *var || subpattern.s_fresh(svar),
            Pattern::ESubst { pattern, plug, .. } => {
                // Assume: substitution is well-formed => plug occurs in the result

                // We can skip checking svar == evar_id, because different types

                // Freshness depends on both input and plug,
                // as evar_id != svar (note that instances of evar_id
                // in pattern do not influence the result)
                pattern.s_fresh(svar) && plug.s_fresh(svar)
            }
            Pattern::SSubst {
                pattern,
                svar_id,
                plug,
            } => {
                // Assume: substitution is well-formed => plug occurs in the result
                if svar == *svar_id {
                    // Freshness depends only on plug as all the free instances
                    // of the requested variable are being substituted
                    return plug.s_fresh(svar);
                }

                // Freshness depends on both input and plug,
                // as evar != evar_id (note that instances of evar_id
                // in pattern do not influence the result)
                pattern.s_fresh(svar) && plug.s_fresh(svar)
            }
        }
    }

    fn positive(&self, svar: Id) -> bool {
        match self {
            Pattern::Bot() => true,
            Pattern::EVar(_) => true,
            Pattern::SVar(_) => true,
            Pattern::Symbol(_) => true,
            Pattern::MetaVar { positive, .. } => positive.contains(&svar) || self.s_fresh(svar),
            Pattern::Implies { left, right } => left.negative(svar) && right.positive(svar),
            Pattern::App { left, right } => left.positive(svar) && right.positive(svar),
            Pattern::Exists { subpattern, .. } => subpattern.positive(svar),
            Pattern::Mu { var, subpattern } => svar == *var || subpattern.positive(svar),
            Pattern::ESubst { pattern, plug, .. } =>
            // best-effort for now, see spec
            {
                pattern.positive(svar) && plug.s_fresh(svar)
            }
            Pattern::SSubst {
                pattern,
                svar_id,
                plug,
            } => {
                let plug_positive_svar = plug.s_fresh(svar)
                    || (pattern.positive(*svar_id) && plug.positive(svar))
                    || (pattern.negative(*svar_id) && plug.negative(svar));

                if svar == *svar_id {
                    return plug_positive_svar;
                }

                return pattern.positive(svar) && plug_positive_svar;
            }
        }
    }

    fn negative(&self, svar: Id) -> bool {
        match self {
            Pattern::Bot() => true,
            Pattern::EVar(_) => true,
            Pattern::SVar(name) => *name != svar,
            Pattern::Symbol(_) => true,
            Pattern::MetaVar { negative, .. } => negative.contains(&svar) || self.s_fresh(svar),
            Pattern::Implies { left, right } => left.positive(svar) && right.negative(svar),
            Pattern::App { left, right } => left.negative(svar) && right.negative(svar),
            Pattern::Exists { subpattern, .. } => subpattern.negative(svar),
            Pattern::Mu { var, subpattern } => svar == *var || subpattern.negative(svar),
            Pattern::ESubst { pattern, plug, .. } =>
            // best-effort for now, see spec
            {
                pattern.negative(svar) && plug.s_fresh(svar)
            }
            Pattern::SSubst {
                pattern,
                svar_id,
                plug,
            } => {
                let plug_negative_svar = plug.s_fresh(svar)
                    || (pattern.positive(*svar_id) && plug.negative(svar))
                    || (pattern.negative(*svar_id) && plug.positive(svar));

                if svar == *svar_id {
                    return plug_negative_svar;
                }

                return pattern.negative(svar) && plug_negative_svar;
            }
        }
    }

    fn app_ctx_hole(&self, evar: Id) -> bool {
        match self {
            Pattern::Bot() => false,
            Pattern::EVar(name) => *name == evar,
            Pattern::SVar(_) => false,
            Pattern::Symbol(_) => false,
            Pattern::MetaVar { app_ctx_holes, .. } => app_ctx_holes.contains(&evar),
            Pattern::Implies { .. } => false,
            Pattern::App { left, right } => {
                (left.app_ctx_hole(evar) && right.e_fresh(evar))
                    || (left.e_fresh(evar) && right.app_ctx_hole(evar))
            }
            Pattern::Exists { .. } => false,
            Pattern::Mu { .. } => false,
            Pattern::ESubst {
                pattern,
                evar_id,
                plug,
            } => {
                if *evar_id == evar {
                    pattern.app_ctx_hole(evar) && plug.app_ctx_hole(evar)
                } else {
                    (pattern.app_ctx_hole(evar) && plug.e_fresh(evar))
                        || (pattern.app_ctx_hole(*evar_id)
                            && plug.app_ctx_hole(evar)
                            && pattern.e_fresh(evar))
                }
            }
            Pattern::SSubst { .. } => {
                unimplemented!("application context hole checking not supported for SSubst's");
            }
        }
    }

    // Checks whether pattern is well-formed ASSUMING
    // that the sub-patterns are well-formed
    // TODO: Audit this function to see if we need to add any more cases
    fn well_formed(&self) -> bool {
        match self {
            Pattern::MetaVar {
                e_fresh,
                app_ctx_holes,
                ..
            } => return !app_ctx_holes.into_iter().any(|hole| e_fresh.contains(hole)),
            Pattern::Mu { var, subpattern } => subpattern.positive(*var),
            Pattern::ESubst {
                pattern,
                evar_id,
                plug,
            } => {
                !pattern.is_redundant_esubst(*evar_id, *plug)
                    && matches!(
                        **pattern,
                        Pattern::MetaVar { .. } | Pattern::ESubst { .. } | Pattern::SSubst { .. }
                    )
            }
            Pattern::SSubst {
                pattern,
                svar_id,
                plug,
            } => {
                !pattern.is_redundant_ssubst(*svar_id, *plug)
                    && matches!(
                        **pattern,
                        Pattern::MetaVar { .. } | Pattern::ESubst { .. } | Pattern::SSubst { .. }
                    )
            }
            _ => {
                // TODO: If we make sure that we only use well-formed above constructs, then we should not need to check recursively
                unimplemented!(
                    "Well-formedness checking is unimplemented yet for this kind of pattern."
                );
            }
        }
    }

    fn is_redundant_esubst(&self, evar_id: Id, plug: Ptr<Pattern>) -> bool {
        self.e_fresh(evar_id) || *plug == Pattern::EVar(evar_id)
    }

    fn is_redundant_ssubst(&self, svar_id: Id, plug: Ptr<Pattern>) -> bool {
        self.s_fresh(svar_id) || *plug == Pattern::SVar(svar_id)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Term {
    Pattern(Ptr<Pattern>),
    Proved(Ptr<Pattern>),
}
#[derive(Debug, Eq, PartialEq)]
pub enum Entry {
    Pattern(Ptr<Pattern>),
    Proved(Ptr<Pattern>),
}

/// Pattern construction utilities
/// ------------------------------

fn bot() -> Ptr<Pattern> {
    return Ptr::new(Pattern::Bot());
}

fn evar(id: Id) -> Ptr<Pattern> {
    return Ptr::new(Pattern::EVar(id));
}

fn svar(id: Id) -> Ptr<Pattern> {
    return Ptr::new(Pattern::SVar(id));
}

fn symbol(id: Id) -> Ptr<Pattern> {
    return Ptr::new(Pattern::Symbol(id));
}

fn metavar_unconstrained(var_id: Id) -> Ptr<Pattern> {
    return Ptr::new(Pattern::MetaVar {
        id: var_id,
        e_fresh: vec![],
        s_fresh: vec![],
        positive: vec![],
        negative: vec![],
        app_ctx_holes: vec![],
    });
}

fn metavar_positive(var_id: Id, evar: Id) -> Ptr<Pattern> {
    return Ptr::new(Pattern::MetaVar {
        id: var_id,
        e_fresh: vec![],
        s_fresh: vec![],
        positive: vec![evar],
        negative: vec![],
        app_ctx_holes: vec![],
    });
}

fn metavar_app_ctx_hole(var_id: Id, evar: Id) -> Ptr<Pattern> {
    return Ptr::new(Pattern::MetaVar {
        id: var_id,
        e_fresh: vec![],
        s_fresh: vec![],
        positive: vec![],
        negative: vec![],
        app_ctx_holes: vec![evar],
    });
}

#[cfg(test)]
fn metavar_e_fresh(var_id: Id, fresh: Id, positive: IdList, negative: IdList) -> Ptr<Pattern> {
    return Ptr::new(Pattern::MetaVar {
        id: var_id,
        e_fresh: vec![fresh],
        s_fresh: vec![],
        positive,
        negative,
        app_ctx_holes: vec![],
    });
}

#[cfg(test)]
fn metavar_s_fresh(var_id: Id, fresh: Id, positive: IdList, negative: IdList) -> Ptr<Pattern> {
    return Ptr::new(Pattern::MetaVar {
        id: var_id,
        e_fresh: vec![],
        s_fresh: vec![fresh],
        positive,
        negative,
        app_ctx_holes: vec![],
    });
}

#[inline(always)]
fn exists(var: Id, subpattern: Ptr<Pattern>) -> Ptr<Pattern> {
    return Ptr::new(Pattern::Exists { var, subpattern });
}

// Does not do any well-formedness checks!!!!!
#[inline(always)]
fn mu(var: Id, subpattern: Ptr<Pattern>) -> Ptr<Pattern> {
    return Ptr::new(Pattern::Mu { var, subpattern });
}

#[inline(always)]
fn esubst(pattern: Ptr<Pattern>, evar_id: Id, plug: Ptr<Pattern>) -> Ptr<Pattern> {
    return Ptr::new(Pattern::ESubst {
        pattern,
        evar_id,
        plug,
    });
}

#[inline(always)]
fn ssubst(pattern: Ptr<Pattern>, svar_id: Id, plug: Ptr<Pattern>) -> Ptr<Pattern> {
    return Ptr::new(Pattern::SSubst {
        pattern,
        svar_id,
        plug,
    });
}

#[inline(always)]
fn implies(left: Ptr<Pattern>, right: Ptr<Pattern>) -> Ptr<Pattern> {
    return Ptr::new(Pattern::Implies { left, right });
}

#[inline(always)]
fn app(left: Ptr<Pattern>, right: Ptr<Pattern>) -> Ptr<Pattern> {
    return Ptr::new(Pattern::App { left, right });
}

// Notation
#[inline(always)]
fn not(pat: Ptr<Pattern>) -> Ptr<Pattern> {
    implies(pat, bot())
}

#[allow(dead_code)]
fn forall(evar: Id, pat: Ptr<Pattern>) -> Ptr<Pattern> {
    not(exists(evar, not(pat)))
}

/// Substitution utilities
/// ----------------------

fn apply_esubst(pattern: Ptr<Pattern>, evar_id: Id, plug: Ptr<Pattern>) -> Ptr<Pattern> {
    if pattern.is_redundant_esubst(evar_id, plug) {
        return pattern;
    }
    match *pattern {
        Pattern::EVar(e) => {
            if e == evar_id {
                plug
            } else {
                pattern
            }
        }
        Pattern::Implies { left, right } => implies(
            apply_esubst(left, evar_id, plug),
            apply_esubst(right, evar_id, plug),
        ),
        Pattern::App { left, right } => app(
            apply_esubst(left, evar_id, plug),
            apply_esubst(right, evar_id, plug),
        ),
        Pattern::Exists { var, .. } if var == evar_id => pattern,
        Pattern::Exists { var, subpattern } => {
            assert!(
                plug.e_fresh(var),
                "EVar substitution would capture free element variable {}!",
                var
            );
            exists(var, apply_esubst(subpattern, evar_id, plug))
        }
        Pattern::Mu { var, subpattern } => {
            assert!(
                plug.s_fresh(var),
                "EVar substitution would capture free set variable {}!",
                var
            );
            mu(var, apply_esubst(subpattern, evar_id, plug))
        }
        Pattern::ESubst { .. } => esubst(pattern, evar_id, plug),
        Pattern::SSubst { .. } => esubst(pattern, evar_id, plug),
        Pattern::MetaVar { .. } => esubst(pattern, evar_id, plug),
        _ => pattern,
    }
}

fn apply_ssubst(pattern: Ptr<Pattern>, svar_id: Id, plug: Ptr<Pattern>) -> Ptr<Pattern> {
    if pattern.is_redundant_ssubst(svar_id, plug) {
        return pattern;
    }
    match *pattern {
        Pattern::SVar(s) => {
            if s == svar_id {
                plug
            } else {
                pattern
            }
        }
        Pattern::Implies { left, right } => implies(
            apply_ssubst(left, svar_id, plug),
            apply_ssubst(right, svar_id, plug),
        ),
        Pattern::App { left, right } => app(
            apply_ssubst(left, svar_id, plug),
            apply_ssubst(right, svar_id, plug),
        ),
        Pattern::Exists { var, subpattern } => {
            assert!(
                plug.e_fresh(var),
                "SVar substitution would capture free element variable {}!",
                var
            );
            exists(var, apply_ssubst(subpattern, svar_id, plug))
        }
        Pattern::Mu { var, .. } if var == svar_id => pattern,
        Pattern::Mu { var, subpattern } => {
            assert!(
                plug.s_fresh(var),
                "SVar substitution would capture free set variable {}!",
                var
            );
            mu(var, apply_ssubst(subpattern, svar_id, plug))
        }
        Pattern::ESubst { .. } => ssubst(pattern, svar_id, plug),
        Pattern::SSubst { .. } => ssubst(pattern, svar_id, plug),
        Pattern::MetaVar { .. } => ssubst(pattern, svar_id, plug),
        _ => pattern,
    }
}

fn instantiate(p: Ptr<Pattern>, vars: &[Id], plugs: &[Ptr<Pattern>]) -> Ptr<Pattern> {
    match &*p {
        Pattern::Bot() => p,
        Pattern::EVar(_) => p,
        Pattern::SVar(_) => p,
        Pattern::Symbol(_) => p,
        Pattern::MetaVar {
            id,
            e_fresh,
            s_fresh,
            positive,
            negative,
            app_ctx_holes,
        } => {
            if let Some(pos) = vars.iter().position(|&x| x == *id) {
                if let Some(evar) = e_fresh
                    .into_iter()
                    .find(|&evar| !(*plugs[pos]).e_fresh(*evar))
                {
                    panic!(
                        "Instantiation of MetaVar {} breaks a freshness constraint: EVar {}",
                        id, evar
                    );
                }
                if let Some(svar) = s_fresh
                    .into_iter()
                    .find(|&svar| !(*plugs[pos]).s_fresh(*svar))
                {
                    panic!(
                        "Instantiation of MetaVar {} breaks a freshness constraint: SVar {}",
                        id, svar
                    );
                }
                if let Some(svar) = positive
                    .into_iter()
                    .find(|&svar| !(*plugs[pos]).positive(*svar))
                {
                    panic!(
                        "Instantiation of MetaVar {} breaks a positivity constraint: SVar {}",
                        id, svar
                    );
                }
                if let Some(svar) = negative
                    .into_iter()
                    .find(|&svar| !(*plugs[pos]).negative(*svar))
                {
                    panic!(
                        "Instantiation of MetaVar {} breaks a negativity constraint: SVar {}",
                        id, svar
                    );
                }
                if let Some(evar) = app_ctx_holes
                    .into_iter()
                    .find(|&evar| !plugs[pos].app_ctx_hole(*evar))
                {
                    panic!(
                        "Instantiation of MetaVar {} breaks an application context hole constraint: EVar {}",
                        id, evar
                    );
                }

                if pos >= plugs.len() {
                    panic!("Substitution does not contain a corresponding value.")
                }
                return plugs[pos];
            }
            p
        }
        Pattern::Implies { left, right } => {
            let inst_left = instantiate(*left, vars, plugs);
            let inst_right = instantiate(*right, vars, plugs);
            if inst_left.is(left) && inst_right.is(right) {
                p
            } else {
                implies(inst_left, inst_right)
            }
        }
        Pattern::App { left, right } => {
            let inst_left = instantiate(*left, vars, plugs);
            let inst_right = instantiate(*right, vars, plugs);
            if inst_left.is(left) && inst_right.is(right) {
                p
            } else {
                app(inst_left, inst_right)
            }
        }
        Pattern::Exists { var, subpattern } => {
            let new_sub = instantiate(*subpattern, vars, plugs);
            if new_sub.is(subpattern) {
                p
            } else {
                exists(*var, new_sub)
            }
        }
        Pattern::Mu { var, subpattern } => {
            let new_sub = instantiate(*subpattern, vars, plugs);
            if new_sub.is(subpattern) {
                p
            } else {
                mu(*var, new_sub)
            }
        }
        Pattern::ESubst {
            pattern,
            evar_id,
            plug,
        } => {
            let inst_pattern = instantiate(*pattern, vars, plugs);
            let inst_plug = instantiate(*plug, vars, plugs);
            if inst_pattern.is(pattern) && inst_plug.is(plug) {
                p
            } else {
                apply_esubst(inst_pattern, *evar_id, inst_plug)
            }
        }
        Pattern::SSubst {
            pattern,
            svar_id,
            plug,
        } => {
            let inst_pattern = instantiate(*pattern, vars, plugs);
            let inst_plug = instantiate(*plug, vars, plugs);
            if inst_pattern.is(pattern) && inst_plug.is(plug) {
                p
            } else {
                apply_ssubst(inst_pattern, *svar_id, inst_plug)
            }
        }
    }
}

/// Proof checker
/// =============

type Stack = Vec<Term>;
type Claims = Vec<Ptr<Pattern>>;
type Memory = Vec<Entry>;

/// Stack utilities
/// ---------------

fn pop_stack(stack: &mut Stack) -> Term {
    return stack.pop().expect("Insufficient stack items.");
}

fn pop_stack_pattern(stack: &mut Stack) -> Ptr<Pattern> {
    match pop_stack(stack) {
        Term::Pattern(p) => return p,
        _ => panic!("Expected pattern on stack."),
    }
}

fn pop_stack_proved(stack: &mut Stack) -> Ptr<Pattern> {
    match pop_stack(stack) {
        Term::Proved(p) => return p,
        _ => panic!("Expected proved on stack."),
    }
}

/// Main implementation
/// -------------------

pub enum ExecutionPhase {
    Gamma,
    Claim,
    Proof,
}

fn read_u8_vec<'a>(iterator: &mut InstrIterator) -> Vec<u8> {
    let len = (*iterator.next().expect("Expected length for array")) as usize;

    let mut vec: Vec<u8> = Vec::with_capacity(len);
    for _ in 0..len {
        vec.push(
            *iterator
                .next()
                .expect("Expected another constraint of given type"),
        );
    }
    return vec;
}

fn execute_instructions<'a>(
    buffer: &Vec<InstByte>,
    stack: &mut Stack,
    memory: &mut Memory,
    claims: &mut Claims,
    phase: ExecutionPhase,
) {
    // Get an iterator for the input buffer
    let iterator: &mut InstrIterator = &mut buffer.iter();

    // Metavars
    let phi0 = metavar_unconstrained(0);
    let phi1 = metavar_unconstrained(1);
    let phi2 = metavar_unconstrained(2);

    // Axioms
    let prop1 = implies(phi0, implies(phi1, phi0));
    let prop2 = implies(
        implies(phi0, implies(phi1, phi2)),
        implies(implies(phi0, phi1), implies(phi0, phi2)),
    );
    let prop3 = implies(not(not(phi0)), phi0);
    fn quantifier(evar_x_id: Id, evar_y_id: Id) -> Ptr<Pattern> {
        implies(
            apply_esubst(metavar_unconstrained(0), evar_x_id, evar(evar_y_id)),
            exists(evar_x_id, metavar_unconstrained(0)),
        )
    }

    fn existence(evar_id: Id) -> Ptr<Pattern> {
        exists(evar_id, evar(evar_id))
    }

    fn prefixpoint(binder: Id) -> Ptr<Pattern> {
        let phi = metavar_positive(0, binder);
        let fp = mu(binder, phi);
        return implies(apply_ssubst(phi, binder, fp), fp);
    }

    while let Some(instr_u32) = iterator.next() {
        match Instruction::from(*instr_u32) {
            // TODO: Add an abstraction for pushing these one-argument terms on stack?
            Instruction::Bot => {
                stack.push(Term::Pattern(bot()));
            }
            Instruction::EVar => {
                let id = *iterator
                    .next()
                    .expect("Expected id for the EVar to be put on stack")
                    as Id;

                stack.push(Term::Pattern(evar(id)));
            }
            Instruction::SVar => {
                let id = *iterator
                    .next()
                    .expect("Expected id for the SVar to be put on stack")
                    as Id;

                stack.push(Term::Pattern(svar(id)));
            }
            Instruction::Symbol => {
                let id = *iterator
                    .next()
                    .expect("Expected id for the Symbol to be put on stack")
                    as Id;

                stack.push(Term::Pattern(symbol(id)));
            }
            Instruction::MetaVar => {
                let id = *iterator
                    .next()
                    .expect("Expected id for MetaVar instruction") as Id;
                let e_fresh = read_u8_vec(iterator);
                let s_fresh = read_u8_vec(iterator);
                let positive = read_u8_vec(iterator);
                let negative = read_u8_vec(iterator);
                let app_ctx_holes = read_u8_vec(iterator);

                let metavar_pat = Ptr::new(Pattern::MetaVar {
                    id,
                    e_fresh,
                    s_fresh,
                    positive,
                    negative,
                    app_ctx_holes,
                });

                if !metavar_pat.well_formed() {
                    panic!("Constructed meta-var {:?} is ill-formed.", &metavar_pat);
                }

                stack.push(Term::Pattern(metavar_pat));
            }
            Instruction::CleanMetaVar => {
                let id = *iterator
                    .next()
                    .expect("Expected id for MetaVar instruction") as Id;

                let metavar_pat = Ptr::new(Pattern::MetaVar {
                    id,
                    e_fresh: vec![],
                    s_fresh: vec![],
                    positive: vec![],
                    negative: vec![],
                    app_ctx_holes: vec![],
                });

                // Clean metavars are always well-formed
                stack.push(Term::Pattern(metavar_pat));
            }
            Instruction::Implies => {
                let right = pop_stack_pattern(stack);
                let left = pop_stack_pattern(stack);
                stack.push(Term::Pattern(implies(left, right)))
            }
            Instruction::App => {
                let right = pop_stack_pattern(stack);
                let left = pop_stack_pattern(stack);
                stack.push(Term::Pattern(app(left, right)))
            }
            Instruction::Exists => {
                let id = *iterator
                    .next()
                    .expect("Expected var_id for the exists binder") as Id;
                let subpattern = pop_stack_pattern(stack);
                stack.push(Term::Pattern(exists(id, subpattern)))
            }
            Instruction::Mu => {
                let id = *iterator
                    .next()
                    .expect("Expected var_id for the exists binder") as Id;
                let subpattern = pop_stack_pattern(stack);

                let mu_pat = mu(id, subpattern);
                if !mu_pat.well_formed() {
                    panic!("Constructed mu-pattern {:?} is ill-formed", &mu_pat);
                }

                stack.push(Term::Pattern(mu_pat))
            }
            Instruction::ESubst => {
                let evar_id = *iterator
                    .next()
                    .expect("Insufficient parameters for ESubst instruction")
                    as Id;
                let pattern = pop_stack_pattern(stack);
                let plug = pop_stack_pattern(stack);

                let esubst_pat = esubst(pattern, evar_id, plug);
                assert!(
                    esubst_pat.well_formed(),
                    "Creating an ill-formed esubst {:?}",
                    esubst_pat
                );

                stack.push(Term::Pattern(esubst_pat));
            }

            Instruction::SSubst => {
                let svar_id = *iterator
                    .next()
                    .expect("Insufficient parameters for SSubst instruction.")
                    as Id;
                let pattern = pop_stack_pattern(stack);
                let plug = pop_stack_pattern(stack);

                let ssubst_pat = ssubst(pattern, svar_id, plug);
                assert!(
                    ssubst_pat.well_formed(),
                    "Creating an ill-formed ssubst {:?}",
                    ssubst_pat
                );

                stack.push(Term::Pattern(ssubst_pat));
            }

            Instruction::Prop1 => {
                stack.push(Term::Proved(prop1));
            }
            Instruction::Prop2 => {
                stack.push(Term::Proved(prop2));
            }
            Instruction::Prop3 => {
                stack.push(Term::Proved(prop3));
            }
            Instruction::ModusPonens => {
                let premise2 = pop_stack_proved(stack);
                let premise1 = pop_stack_proved(stack);
                match *premise1 {
                    Pattern::Implies { left, right } => {
                        if *left != *premise2 {
                            panic!("Antecedents do not match for modus ponens.\nleft.psi:\n{:?}\n\n right:\n{:?}\n", left, premise2)
                        }
                        stack.push(Term::Proved(right))
                    }
                    _ => {
                        panic!("Expected an implication as a first parameter.")
                    }
                }
            }
            Instruction::Quantifier => {
                let evar_x_id = *iterator
                    .next()
                    .expect("Insufficient parameters for Quantifier instruction")
                    as Id;
                let evar_y_id = *iterator
                    .next()
                    .expect("Insufficient parameters for Quantifier instruction")
                    as Id;
                stack.push(Term::Proved(quantifier(evar_x_id, evar_y_id)));
            }
            Instruction::PreFixpoint => {
                let binder = *iterator
                    .next()
                    .expect("Insufficient parameters for Existence instruction.")
                    as Id;
                stack.push(Term::Proved(prefixpoint(binder)));
            }
            Instruction::Existence => {
                let evar_id = *iterator
                    .next()
                    .expect("Insufficient parameters for Existence instruction.")
                    as Id;
                stack.push(Term::Proved(existence(evar_id)));
            }
            Instruction::Generalization => match *pop_stack_proved(stack) {
                Pattern::Implies { left, right } => {
                    let evar_id = *iterator
                        .next()
                        .expect("Insufficient parameters for Generalization instruction")
                        as Id;

                    if !right.e_fresh(evar_id) {
                        panic!("The binding variable has to be fresh in the conclusion.");
                    }

                    stack.push(Term::Proved(implies(exists(evar_id, left), right)));
                }
                _ => {
                    panic!("Expected an implication as a first parameter.")
                }
            },
            Instruction::Framing => match *pop_stack_proved(stack) {
                Pattern::Implies { left, right } => {
                    let hole = *iterator
                        .next()
                        .expect("Insufficient parameters for Framing instruction")
                        as Id;
                    let phi = metavar_app_ctx_hole(0, hole);
                    stack.push(Term::Proved(implies(
                        apply_esubst(phi, hole, left),
                        apply_esubst(phi, hole, right),
                    )));
                }
                _ => {
                    panic!("Expected an implication as a first parameter.")
                }
            },
            Instruction::Substitution => {
                let svar_id = *iterator
                    .next()
                    .expect("Insufficient parameters for Substitution instruction.");
                let pattern = pop_stack_proved(stack);
                let plug = pop_stack_pattern(stack);

                stack.push(Term::Proved(apply_ssubst(pattern, svar_id, plug)));
            }
            Instruction::KnasterTarski => match *pop_stack_proved(stack) {
                Pattern::Implies { left, right } => {
                    let phi = pop_stack_pattern(stack);
                    let binder = *iterator
                        .next()
                        .expect("Insufficient parameters for KnasterTarski instruction")
                        as Id;
                    if !phi.positive(binder) {
                        panic!("The binding variable has to be positive in phi.");
                    }
                    if apply_ssubst(phi, binder, right) != left {
                        panic!("Incorrect LHS for KnasterTarski");
                    }
                    stack.push(Term::Proved(implies(mu(binder, phi), right)));
                }
                _ => {
                    panic!("Expected an implication as a first parameter.")
                }
            },
            Instruction::Instantiate => {
                let n = *iterator
                    .next()
                    .expect("Insufficient parameters for Instantiate instruction")
                    as usize;
                let mut ids: IdList = Vec::with_capacity(n);
                let mut plugs: Vec<Ptr<Pattern>> = Vec::with_capacity(n);

                let metaterm = pop_stack(stack);

                iterator.take(n).for_each(|arg| {
                    ids.push(*arg as Id);
                    plugs.push(pop_stack_pattern(stack))
                });

                match metaterm {
                    Term::Pattern(p) => {
                        stack.push(Term::Pattern(instantiate(p, &ids, &plugs)));
                    }
                    Term::Proved(p) => {
                        stack.push(Term::Proved(instantiate(p, &ids, &plugs)));
                    }
                }
            }
            Instruction::Pop => {
                _ = pop_stack(stack);
            }
            Instruction::Save => match stack.last().expect("Save needs an entry on the stack") {
                Term::Pattern(p) => memory.push(Entry::Pattern(p.clone())),
                Term::Proved(p) => memory.push(Entry::Proved(p.clone())),
            },
            Instruction::Load => {
                let index = *iterator
                    .next()
                    .expect("Insufficient parameters for Load instruction");
                match &memory[index as usize] {
                    Entry::Pattern(p) => stack.push(Term::Pattern(p.clone())),
                    Entry::Proved(p) => stack.push(Term::Proved(p.clone())),
                }
            }
            Instruction::Publish => match phase {
                ExecutionPhase::Gamma => {
                    memory.push(Entry::Proved(pop_stack_pattern(stack)));
                    assert!(stack.is_empty())
                }
                ExecutionPhase::Claim => {
                    let claim = pop_stack_pattern(stack);
                    claims.push(claim);
                    assert!(stack.is_empty())
                }
                ExecutionPhase::Proof => {
                    let claim = claims.pop().expect("Insufficient claims.");
                    let theorem = pop_stack_proved(stack);
                    if claim != theorem {
                        panic!(
                            "This proof does not prove the requested claim: {:?}, theorem: {:?}",
                            claim, theorem
                        );
                    }
                }
            },
            Instruction::Version => {
                let major = *iterator.next().expect("Expected a major.");
                let minor = *iterator.next().expect("Expected a minor.");
                assert_eq!(
                    major, VERSION_MAJOR,
                    "Proof checker does not support proofs in this version"
                );
                assert_eq!(
                    minor, VERSION_MINOR,
                    "Proof checker does not support proofs in this version"
                );
            }
            _ => {
                unimplemented!("Instruction: {}", instr_u32)
            }
        }
    }
}

pub fn verify<'a>(
    gamma_buffer: &Vec<InstByte>,
    claims_buffer: &Vec<InstByte>,
    proof_buffer: &Vec<InstByte>,
) {
    let mut claims: Claims = Vec::with_capacity(2);
    let mut memory: Memory = Vec::with_capacity(256);
    let mut stack = Vec::with_capacity(256);

    execute_instructions(
        gamma_buffer,
        &mut stack,  // stack is empty initially.
        &mut memory, // memory is empty initially.
        &mut claims, // claims is unused in this phase.
        ExecutionPhase::Gamma,
    );

    stack.clear();

    execute_instructions(
        claims_buffer,
        &mut stack,  // stack is empty initially.
        &mut memory, // reuse memory
        &mut claims, // claims populated in this phase
        ExecutionPhase::Claim,
    );

    stack.clear();

    execute_instructions(
        proof_buffer,
        &mut stack,  // stack is empty initially.
        &mut memory, // axioms are used as initial memory
        &mut claims, // claims are consumed by publish instruction
        ExecutionPhase::Proof,
    );

    assert!(
        claims.is_empty(),
        "Checking finished but there are claims left unproved:\n{:?}\n",
        claims
    );
}

/// Testing
/// =======
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(&mut vec![
        Instruction::SVar as InstByte, 0,
        Instruction::CleanMetaVar as InstByte, 0,
        Instruction::ESubst as InstByte, 0
        ], esubst(metavar_unconstrained(0), 0, svar(0)))]
    fn test_pattern_construction(
        #[case] instructions: &mut Vec<InstByte>,
        #[case] expected_pattern: Ptr<Pattern>,
    ) {
        let stack = &mut Vec::with_capacity(256);
        execute_instructions(
            instructions,
            stack,
            &mut vec![],
            &mut vec![],
            ExecutionPhase::Gamma,
        );
        assert_eq!(stack.len(), 1);
        assert_eq!(stack[0], Term::Pattern(expected_pattern));
    }

    #[test]
    #[should_panic]
    fn test_pop_instruction() {
        execute_instructions(
            &mut vec![Instruction::Pop as InstByte],
            &mut vec![],
            &mut vec![],
            &mut vec![],
            ExecutionPhase::Gamma,
        );
    }

    #[test]
    fn test_efresh() {
        assert!(bot().e_fresh(0));
        let evar = evar(1);
        let left = Ptr::new(Pattern::Exists {
            var: 1,
            subpattern: evar.clone(),
        });
        assert!(left.e_fresh(1));

        let right = Ptr::new(Pattern::Exists {
            var: 2,
            subpattern: evar,
        });
        assert!(!right.e_fresh(1));

        let implication = implies(left, right);
        assert!(!implication.e_fresh(1));

        let mvar = metavar_s_fresh(1, 2, vec![2], vec![2]);
        let metaapp = Pattern::App {
            left: left,
            right: mvar,
        };
        assert!(!metaapp.e_fresh(2));

        let esubst_ = esubst(right, 1, left);
        assert!(esubst_.e_fresh(1));

        let ssubst_ = ssubst(right, 1, left);
        assert!(!ssubst_.e_fresh(1));
    }

    #[test]
    fn test_sfresh() {
        assert!(bot().s_fresh(0));
        let svar = svar(1);
        let left = Ptr::new(Pattern::Mu {
            var: 1,
            subpattern: svar.clone(),
        });
        assert!(left.s_fresh(1));

        let right = Ptr::new(Pattern::Mu {
            var: 2,
            subpattern: svar,
        });
        assert!(!right.s_fresh(1));

        let implication = implies(left, right);
        assert!(!implication.s_fresh(1));

        let mvar = metavar_s_fresh(1, 2, vec![2], vec![2]);
        let metaapp = Pattern::App {
            left: left,
            right: mvar,
        };
        assert!(!metaapp.s_fresh(1));

        let metaapp2 = Pattern::App {
            left: left,
            right: mvar,
        };
        assert!(metaapp2.s_fresh(2));

        let esubst_ = esubst(right, 1, left);
        assert!(!esubst_.s_fresh(1));

        let ssubst_ = ssubst(right, 1, left);
        assert!(ssubst_.s_fresh(1));
    }

    #[test]
    #[should_panic]
    fn test_instantiate_fresh() {
        let svar_0 = svar(0);
        let phi0_s_fresh_0 = metavar_s_fresh(0, 0, vec![0], vec![0]);
        instantiate(phi0_s_fresh_0, &[0], &[svar_0]);
    }

    #[test]
    fn test_wellformedness_fresh() {
        let phi0_s_fresh_0 = metavar_s_fresh(0, 0, vec![0], vec![0]);
        assert!(phi0_s_fresh_0.well_formed());

        let phi1 = Ptr::new(Pattern::MetaVar {
            id: 1,
            e_fresh: vec![1, 2, 0],
            s_fresh: vec![],
            positive: vec![],
            negative: vec![],
            app_ctx_holes: vec![2],
        });
        assert!(!phi1.well_formed());

        // TODO: Reason why this is not needed
        // let phi1_imp_phi1 = implies(phi1, phi1);
        // assert!(!phi1_imp_phi1.well_formed());
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_wellformedness_esubst_ssubst() {
        let phi0_x1_s1 = esubst(metavar_unconstrained(0), 1, symbol(1));
        assert!(phi0_x1_s1.well_formed());

        let s0_x1_s1 = esubst(symbol(0), 1, symbol(1));
        assert!(!s0_x1_s1.well_formed());

        let phi0_x1_x1 = esubst(metavar_unconstrained(0), 1, evar(1));
        assert!(!phi0_x1_x1.well_formed());

        let phi0_fresh_x1_s1 = esubst(metavar_e_fresh(0, 1, vec![], vec![]), 1, symbol(1));
        assert!(!phi0_fresh_x1_s1.well_formed());

        let phi0_X1_s1 = ssubst(metavar_unconstrained(0), 1, symbol(1));
        assert!(phi0_X1_s1.well_formed());

        let phi0_X1_X1 = ssubst(metavar_unconstrained(0), 1, svar(1));
        assert!(!phi0_X1_X1.well_formed());

        let s0_X1_s1 = ssubst(symbol(0), 1, symbol(1));
        assert!(!s0_X1_s1.well_formed());

        let phi0_fresh_X1_s1 = ssubst(metavar_s_fresh(0, 1, vec![], vec![]), 1, symbol(1));
        assert!(!phi0_fresh_X1_s1.well_formed());
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_positivity() {
        let X0 = svar(0);
        let X1 = svar(1);
        let X2 = svar(2);
        let c1 = symbol(1);
        let neg_X1 = not(X1);

        // Bot
        assert!(bot().positive(0));
        assert!(bot().negative(0));

        // EVar
        let evar1 = evar(1);
        assert!(evar1.positive(1));
        assert!(evar1.negative(1));
        assert!(evar1.positive(2));
        assert!(evar1.negative(2));

        // SVar
        assert!(X1.positive(1));
        assert!(!X1.negative(1));
        assert!(X1.positive(2));
        assert!(X1.negative(2));

        // Symbol
        assert!(c1.positive(1));
        assert!(c1.negative(1));
        assert!(c1.positive(2));
        assert!(c1.negative(2));

        // App
        let appX1X2 = app(X1, X2);
        assert!(appX1X2.positive(1));
        assert!(appX1X2.positive(2));
        assert!(appX1X2.positive(3));
        assert!(!appX1X2.negative(1));
        assert!(!appX1X2.negative(2));
        assert!(appX1X2.negative(3));

        // Implies
        let impliesX1X2 = implies(X1, X2);
        assert!(!impliesX1X2.positive(1));
        assert!(impliesX1X2.positive(2));
        assert!(impliesX1X2.positive(3));
        assert!(impliesX1X2.negative(1));
        assert!(!impliesX1X2.negative(2));
        assert!(impliesX1X2.negative(3));

        let impliesX1X1 = implies(X1, X1);
        assert!(!impliesX1X1.positive(1));
        assert!(!impliesX1X1.negative(1));

        // Exists
        let existsX1X2 = exists(1, X2);
        assert!(existsX1X2.positive(1));
        assert!(existsX1X2.positive(2));
        assert!(existsX1X2.positive(3));
        assert!(existsX1X2.negative(1));
        assert!(!existsX1X2.negative(2));
        assert!(existsX1X2.negative(3));

        let existsX1nX2 = exists(1, not(X2));
        assert!(existsX1nX2.negative(2));

        // Mu
        let muX1x1 = mu(1, evar1);
        assert!(muX1x1.positive(1));
        assert!(muX1x1.positive(2));
        assert!(muX1x1.negative(1));
        assert!(muX1x1.negative(2));

        let muX1X1 = mu(1, X1);
        assert!(muX1X1.positive(1));
        assert!(muX1X1.negative(1));

        let muX1X2 = mu(1, X2);
        assert!(muX1X2.positive(1));
        assert!(muX1X2.positive(2));
        assert!(muX1X2.positive(3));
        assert!(muX1X2.negative(1));
        assert!(!muX1X2.negative(2));
        assert!(mu(1, implies(X2, X1)).negative(2));
        assert!(muX1X2.negative(3));

        // MetaVar
        assert!(!metavar_unconstrained(1).positive(1));
        assert!(!metavar_unconstrained(1).positive(2));
        assert!(!metavar_unconstrained(1).negative(1));
        assert!(!metavar_unconstrained(1).negative(2));

        assert!(metavar_s_fresh(1, 1, vec![], vec![]).positive(1));
        assert!(metavar_s_fresh(1, 1, vec![], vec![]).negative(1));
        assert!(metavar_s_fresh(1, 1, vec![1], vec![1]).positive(1));
        assert!(metavar_s_fresh(1, 1, vec![1], vec![1]).negative(1));
        assert!(metavar_s_fresh(1, 1, vec![1], vec![]).positive(1));
        assert!(metavar_s_fresh(1, 1, vec![1], vec![]).negative(1));
        assert!(metavar_s_fresh(1, 1, vec![], vec![1]).positive(1));
        assert!(metavar_s_fresh(1, 1, vec![], vec![1]).negative(1));

        assert!(!metavar_s_fresh(1, 1, vec![], vec![]).positive(2));
        assert!(!metavar_s_fresh(1, 1, vec![], vec![]).negative(2));

        // ESubst
        assert!(!esubst(metavar_unconstrained(0), 0, X0).positive(0));
        assert!(!esubst(metavar_unconstrained(0), 0, X1).positive(0));
        assert!(!esubst(metavar_s_fresh(0, 1, vec![1], vec![]), 0, X1).positive(0));

        assert!(!esubst(metavar_unconstrained(0), 0, X0).negative(0));
        assert!(!esubst(metavar_unconstrained(0), 0, X1).negative(0));
        assert!(!esubst(metavar_s_fresh(0, 1, vec![1], vec![]), 0, X1).negative(0));

        // SSubst
        assert!(!ssubst(metavar_unconstrained(0), 0, X0).positive(0));
        assert!(ssubst(metavar_unconstrained(0), 0, X1).positive(0));
        assert!(ssubst(metavar_s_fresh(0, 1, vec![1], vec![]), 0, X1).positive(0));

        assert!(!ssubst(metavar_unconstrained(0), 0, X0).negative(0));
        assert!(ssubst(metavar_unconstrained(0), 0, X1).negative(0));
        assert!(ssubst(metavar_s_fresh(0, 1, vec![1], vec![]), 0, X1).negative(0));

        // Combinations
        assert!(!neg_X1.positive(1));
        assert!(neg_X1.positive(2));
        assert!(neg_X1.negative(1));
        assert!(neg_X1.negative(2));

        let negX1_implies_negX1 = implies(neg_X1, neg_X1);
        assert!(!negX1_implies_negX1.positive(1));
        assert!(negX1_implies_negX1.positive(2));
        assert!(!negX1_implies_negX1.negative(1));
        assert!(negX1_implies_negX1.negative(2));

        let negX1_implies_X1 = implies(neg_X1, X1);
        assert!(negX1_implies_X1.positive(1));
        assert!(!negX1_implies_X1.negative(1));
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_app_ctx_hole() {
        assert!(!bot().app_ctx_hole(0));
        assert!(!metavar_unconstrained(0).app_ctx_hole(0));
        assert!((Pattern::MetaVar {
            id: 0,
            e_fresh: vec![],
            s_fresh: vec![],
            positive: vec![],
            negative: vec![],
            app_ctx_holes: vec![0],
        })
        .app_ctx_hole(0));
        assert!(evar(0).app_ctx_hole(0));
        assert!(!evar(1).app_ctx_hole(0));
        assert!(!svar(0).app_ctx_hole(0));
        assert!(!symbol(0).app_ctx_hole(0));
        assert!(!implies(evar(0), evar(1)).app_ctx_hole(0));
        assert!(app(evar(0), evar(1)).app_ctx_hole(0));
        assert!(app(evar(1), evar(0)).app_ctx_hole(0));
        assert!(!app(evar(0), evar(0)).app_ctx_hole(0));
        assert!(!app(evar(1), evar(1)).app_ctx_hole(0));
        assert!(!exists(0, evar(0)).app_ctx_hole(0));
        assert!(!exists(0, evar(1)).app_ctx_hole(0));
        assert!(!exists(1, evar(0)).app_ctx_hole(0));
        assert!(!exists(1, evar(1)).app_ctx_hole(0));
        assert!(!mu(0, evar(0)).app_ctx_hole(0));
        assert!(!mu(0, svar(0)).app_ctx_hole(0));
    }

    #[test]
    fn test_wellformedness_positive() {
        let svar = svar(1);
        let mux_x = mu(1, svar);
        assert!(mux_x.well_formed());

        let mux_x2 = mu(2, not(svar));
        assert!(mux_x2.well_formed());

        let mux_x3 = mu(2, not(symbol(1)));
        assert!(mux_x3.well_formed());

        let mux_x = mu(1, not(svar));
        assert!(!mux_x.well_formed());

        let phi = metavar_s_fresh(97, 2, vec![], vec![]);
        let mux_phi = mu(1, phi);
        assert!(!mux_phi.well_formed());

        let phi2 = metavar_s_fresh(98, 1, vec![], vec![]);
        let mux_phi2 = mu(1, phi2);
        assert!(mux_phi2.well_formed());

        // It's ok if 2 is negative, the only thing we care about is that 2 is guaranteed to be positive
        // (we can instantiate without this variable)
        let phi3 = metavar_s_fresh(99, 1, vec![2], vec![2]);
        let mux_phi3 = mu(2, phi3);
        assert!(mux_phi3.well_formed());

        let phi4 = metavar_s_fresh(100, 1, vec![2], vec![]);
        let mux_phi4 = mu(2, phi4);
        assert!(mux_phi4.well_formed());
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_instantiate() {
        let x0 = evar(0);
        let X0 = svar(0);
        let c0 = symbol(0);
        let x0_implies_x0 = implies(x0, x0);
        let appx0x0 = app(x0, x0);
        let existsx0x0 = exists(0, x0);
        let muX0x0 = mu(0, x0);

        // Concrete patterns are unaffected by instantiate
        assert!(instantiate(bot(), &[0], &[X0]) == bot());
        assert!(instantiate(x0, &[0], &[X0]) == x0);
        assert!(instantiate(x0, &[1], &[X0]) == x0);
        assert!(instantiate(X0, &[0], &[x0]) == X0);
        assert!(instantiate(X0, &[1], &[x0]) == X0);
        assert!(instantiate(c0, &[0], &[x0]) == c0);
        assert!(instantiate(c0, &[1], &[x0]) == c0);
        assert!(instantiate(x0_implies_x0, &[0], &[x0]) == x0_implies_x0);
        assert!(instantiate(x0_implies_x0, &[1], &[x0]) == x0_implies_x0);
        assert!(instantiate(appx0x0, &[0], &[x0]) == appx0x0);
        assert!(instantiate(appx0x0, &[1], &[x0]) == appx0x0);
        assert!(instantiate(existsx0x0, &[0], &[X0]) == existsx0x0);
        assert!(instantiate(existsx0x0, &[1], &[X0]) == existsx0x0);
        assert!(instantiate(muX0x0, &[0], &[x0]) == muX0x0);
        assert!(instantiate(muX0x0, &[1], &[x0]) == muX0x0);

        let phi0 = metavar_unconstrained(0);
        let phi0_implies_phi0 = implies(phi0, phi0);
        let appphi0phi0 = app(phi0, phi0);
        let existsx0phi0 = exists(0, phi0);
        let muX0phi0 = mu(0, phi0);
        let existsx0X0 = exists(0, X0);
        assert!(instantiate(phi0_implies_phi0, &[0], &[x0]) == x0_implies_x0);
        assert!(instantiate(phi0_implies_phi0, &[1], &[x0]) == phi0_implies_phi0);
        assert_eq!(instantiate(appphi0phi0, &[0], &[x0]), appx0x0);
        assert_eq!(instantiate(appphi0phi0, &[1], &[x0]), appphi0phi0);
        assert_eq!(instantiate(existsx0phi0, &[0], &[x0]), existsx0x0);
        assert_eq!(instantiate(existsx0phi0, &[1], &[x0]), existsx0phi0);
        assert_eq!(instantiate(muX0phi0, &[0], &[x0]), muX0x0);
        assert_eq!(instantiate(muX0phi0, &[1], &[x0]), muX0phi0);

        // Simultaneous instantiations
        let phi1 = metavar_unconstrained(1);
        let muX0phi1 = mu(0, phi1);
        let muX0X0 = mu(0, X0);
        // Empty substs have no effect
        assert!(instantiate(existsx0phi0, &[1, 2], &[x0, X0]) == existsx0phi0);
        assert!(instantiate(existsx0phi0, &[2, 1], &[x0, X0]) == existsx0phi0);
        assert!(instantiate(muX0phi0, &[1, 2], &[x0, X0]) == muX0phi0);
        assert!(instantiate(muX0phi0, &[2, 1], &[x0, X0]) == muX0phi0);

        // Order matters if corresponding value is not moved
        assert!(instantiate(existsx0phi0, &[1, 0], &[x0, X0]) == existsx0X0);
        assert!(instantiate(existsx0phi0, &[0, 1], &[x0, X0]) == existsx0x0);
        assert!(instantiate(muX0phi0, &[1, 0], &[x0, X0]) == muX0X0);
        assert!(instantiate(muX0phi0, &[0, 1], &[x0, X0]) == muX0x0);

        // Order does not matter if corresponding value is moved
        let muX0phi0_implies_ph1 = implies(muX0phi0, phi1);
        let muX0x0_implies_X0 = implies(muX0x0, X0);
        assert!(instantiate(muX0phi0_implies_ph1, &[0, 1], &[x0, X0]) == muX0x0_implies_X0);
        assert!(instantiate(muX0phi0_implies_ph1, &[1, 0], &[X0, x0]) == muX0x0_implies_X0);
        let muX0phi0_app_ph1 = app(muX0phi0, phi1);
        let muX0x0_app_X0 = app(muX0x0, X0);
        assert!(instantiate(muX0phi0_app_ph1, &[0, 1], &[x0, X0]) == muX0x0_app_X0);
        assert!(instantiate(muX0phi0_app_ph1, &[1, 0], &[X0, x0]) == muX0x0_app_X0);

        // No side-effects
        let muX0ph1_implies_X0 = implies(muX0phi1, X0);
        assert!(instantiate(muX0phi0_implies_ph1, &[0, 1], &[phi1, X0]) == muX0ph1_implies_X0);
        assert!(instantiate(muX0phi0_implies_ph1, &[1, 0], &[X0, phi1]) == muX0ph1_implies_X0);
        let muX0ph1_app_X0 = app(muX0phi1, X0);
        assert!(instantiate(muX0phi0_app_ph1, &[0, 1], &[phi1, X0]) == muX0ph1_app_X0);
        assert!(instantiate(muX0phi0_app_ph1, &[1, 0], &[X0, phi1]) == muX0ph1_app_X0);

        // First comes first
        assert!(instantiate(muX0phi0_app_ph1, &[0, 1, 1], &[phi1, X0, x0]) == muX0ph1_app_X0);
        assert!(instantiate(muX0phi0_app_ph1, &[1, 0, 0], &[X0, phi1, x0]) == muX0ph1_app_X0);

        // Extra values are ignored
        assert!(
            instantiate(
                muX0phi0_app_ph1,
                &[0, 1, 1],
                &[phi1, X0, x0, x0, x0, x0, x0, x0]
            ) == muX0ph1_app_X0
        );
        assert!(instantiate(muX0phi0_app_ph1, &[0, 1, 2], &[phi1, X0]) == muX0ph1_app_X0);

        // Instantiate with concrete patterns applies pending substitutions
        let val = esubst(phi0, 0, c0);
        assert_eq!(instantiate(val, &[0], &[x0]), c0);
        let val = ssubst(phi0, 0, c0);
        assert_eq!(instantiate(val, &[0], &[X0]), c0);
        let val = ssubst(esubst(phi0, 0, X0), 0, c0);
        assert_eq!(instantiate(val, &[0], &[X0]), c0);

        // Instantiate with metavar keeps pending substitutions
        let val = esubst(phi0, 0, c0);
        assert_eq!(instantiate(val, &[0], &[phi1]), esubst(phi1, 0, c0));
        let val = ssubst(phi0, 0, c0);
        assert_eq!(instantiate(val, &[0], &[phi1]), ssubst(phi1, 0, c0));

        // The plug in a subst. needs to be instantiated as well
        let val = ssubst(phi0, 0, phi0);
        assert_eq!(instantiate(val, &[0], &[X0]), X0);
        let val = ssubst(phi0, 0, phi1);
        assert_eq!(instantiate(val, &[0, 1], &[X0, c0]), c0);
    }

    // TODO: Add more cases
    // TODO: Add tests for app_ctx_holes once implemented
    #[rstest]
    #[should_panic]
    #[case(vec![0], vec![], vec![], vec![], evar(0))]
    #[should_panic]
    #[case(vec![], vec![0], vec![], vec![], svar(0))]
    #[should_panic]
    #[case(vec![], vec![], vec![0], vec![], not(svar(0)))]
    #[should_panic]
    #[case(vec![], vec![], vec![], vec![0], svar(0))]
    #[should_panic]
    #[case(vec![], vec![], vec![0], vec![0], svar(0))]
    // SVar(0) is both positive and negative when not included
    #[case(vec![], vec![], vec![0], vec![0], svar(1))]
    fn test_instantiation_breaking_constraints(
        #[case] e_fresh: IdList,
        #[case] s_fresh: IdList,
        #[case] positive: IdList,
        #[case] negative: IdList,
        #[case] plug: Ptr<Pattern>,
    ) {
        instantiate(
            Ptr::new(Pattern::MetaVar {
                id: 0,
                e_fresh,
                s_fresh,
                positive,
                negative,
                app_ctx_holes: vec![],
            }),
            &[0],
            &[plug],
        );
    }

    #[test]
    #[should_panic]
    fn test_illformed_instantiation() {
        let phi0 = metavar_unconstrained(0);
        instantiate(phi0, &[1, 0], &[phi0]);
    }

    #[cfg(test)]
    fn execute_vector(
        instrs: &Vec<InstByte>,
        stack: &mut Stack,
        memory: &mut Memory,
        claims: &mut Claims,
        phase: ExecutionPhase,
    ) {
        return execute_instructions(instrs, stack, memory, claims, phase);
    }

    #[test]
    fn test_publish() {
        let proof = vec![Instruction::Publish as InstByte];

        let mut stack = vec![Term::Pattern(symbol(0))];
        let mut memory = vec![];
        let mut claims = vec![];
        execute_vector(
            &proof,
            &mut stack,
            &mut memory,
            &mut claims,
            ExecutionPhase::Gamma,
        );
        assert_eq!(stack, vec![]);
        assert_eq!(claims, vec![]);
        assert_eq!(memory, vec![Entry::Proved(symbol(0))]);

        let mut stack = vec![Term::Pattern(symbol(0))];
        let mut memory = vec![];
        let mut claims = vec![];
        execute_vector(
            &proof,
            &mut stack,
            &mut memory,
            &mut claims,
            ExecutionPhase::Claim,
        );
        assert_eq!(stack, vec![]);
        assert_eq!(memory, vec![]);
        assert_eq!(claims, vec![symbol(0)]);

        let mut stack = vec![Term::Proved(symbol(0))];
        let mut memory = vec![];
        let mut claims = vec![symbol(0)];
        execute_vector(
            &proof,
            &mut stack,
            &mut memory,
            &mut claims,
            ExecutionPhase::Proof,
        );
        assert_eq!(stack, vec![]);
        assert_eq!(memory, vec![]);
        assert_eq!(claims, vec![]);
    }

    #[test]
    fn test_construct_phi_implies_phi() {
        #[rustfmt::skip]
        let proof = vec![
            Instruction::MetaVar as InstByte, 0, 0, 0, 0, 0, 0, // Stack: Phi
            Instruction::Save as InstByte,        // @ 0
            Instruction::Load as InstByte, 0,     // Phi ; Phi
            Instruction::Implies as InstByte, // Phi -> Phi
        ];

        let mut stack = vec![];
        execute_vector(
            &proof,
            &mut stack,
            &mut vec![],
            &mut vec![],
            ExecutionPhase::Proof,
        );
        let phi0 = metavar_unconstrained(0);
        assert_eq!(
            stack,
            vec![Term::Pattern(Ptr::new(Pattern::Implies {
                left: phi0.clone(),
                right: phi0.clone()
            }))]
        );
    }

    #[cfg(test)]
    fn serialize_metavar(id: u8, all_cons: &Vec<Vec<u8>>) -> Vec<u8> {
        let mut res = vec![Instruction::MetaVar as InstByte, id];

        for cons in all_cons {
            res.push(cons.len() as u8);
            res.append(&mut (*cons).clone());
        }

        return res;
    }

    #[test]
    fn test_construct_phi_implies_phi_with_constraints() {
        let mut cons = vec![vec![1u8], vec![], vec![], vec![], vec![]];

        for _ in 0..5 {
            let mut proof: Vec<InstByte> = serialize_metavar(1, &cons);
            proof.append(&mut vec![
                Instruction::Save as InstByte, // @ 0
                Instruction::Load as InstByte,
                0, // Phi1 ; Phi1
                Instruction::Implies as InstByte,
            ]); // Phi1 -> Phi1

            let mut stack = vec![];
            execute_vector(
                &proof,
                &mut stack,
                &mut vec![],
                &mut vec![],
                ExecutionPhase::Proof,
            );

            let phi1 = Ptr::new(Pattern::MetaVar {
                id: 1,
                e_fresh: cons[0].clone(),
                s_fresh: cons[1].clone(),
                positive: cons[2].clone(),
                negative: cons[3].clone(),
                app_ctx_holes: cons[4].clone(),
            });

            assert_eq!(
                stack,
                vec![Term::Pattern(Ptr::new(Pattern::Implies {
                    left: phi1.clone(),
                    right: phi1.clone()
                }))]
            );

            // Make the next field the non-empty one
            cons.rotate_right(1);
        }
    }

    #[test]
    fn test_phi_implies_phi_impl() {
        #[rustfmt::skip]
        let proof = vec![
            Instruction::MetaVar as InstByte, 0, 0, 0, 0, 0, 0, // Stack: $ph0
            Instruction::Save as InstByte,                    // @0
            Instruction::Load as InstByte, 0,                 // Stack: $ph0; ph0
            Instruction::Load as InstByte, 0,                 // Stack: $ph0; $ph0; ph0
            Instruction::Implies as InstByte,             // Stack: $ph0; ph0 -> ph0
            Instruction::Save as InstByte,                    // @1
            Instruction::Prop2 as InstByte,                   // Stack: $ph0; $ph0 -> ph0; [prop2: (ph0 -> (ph1 -> ph2)) -> ((ph0 -> ph1) -> (ph0 -> ph2))]
            Instruction::Instantiate as InstByte, 1, 1,       // Stack: $ph0; [p1: (ph0 -> ((ph0 -> ph0) -> ph2)) -> (ph0 -> (ph0 -> ph0)) -> (ph0 -> ph2)]
            Instruction::Instantiate as InstByte, 1, 2,       // Stack: [p1: (ph0 -> ((ph0 -> ph0) -> ph0)) -> (ph0 -> (ph0 -> ph0)) -> (ph0 -> ph0)]
            Instruction::Load as InstByte, 1,                 // Stack: p1 ; $ph0 -> ph0
            Instruction::Prop1 as InstByte,                   // Stack: p1 ; $ph0 -> ph0; [prop1: ph0 -> (ph1 -> ph0)

            Instruction::Instantiate as InstByte, 1, 1,       // Stack: p1 ; [p2: (ph0 -> (ph0 -> ph0) -> ph0) ]

            Instruction::ModusPonens as InstByte,             // Stack: [p3: (ph0 -> (ph0 -> ph0)) -> (ph0 -> ph0)]
            Instruction::Load as InstByte, 0,                 // Stack: p3 ; ph0;
            Instruction::Prop1 as InstByte,                   // Stack: p3 ; ph0; prop1

            Instruction::Instantiate as InstByte, 1, 1,       // Stack: p3 ; ph0 -> (ph0 -> ph0)

            Instruction::ModusPonens as InstByte,             // Stack: ph0 -> ph0
        ];
        let mut stack = vec![];
        execute_vector(
            &proof,
            &mut stack,
            &mut vec![],
            &mut vec![],
            ExecutionPhase::Proof,
        );
        let phi0 = metavar_unconstrained(0);
        assert_eq!(
            stack,
            vec![Term::Proved(Ptr::new(Pattern::Implies {
                left: phi0,
                right: phi0
            }))]
        )
    }

    #[test]
    fn test_universal_quantification() {
        #[rustfmt::skip]
        let proof = vec![
            Instruction::Generalization as InstByte,
            0
        ];
        let mut stack = vec![Term::Proved(implies(symbol(0), symbol(1)))];
        let mut memory = vec![];
        let mut claims = vec![];
        execute_vector(
            &proof,
            &mut stack,
            &mut memory,
            &mut claims,
            ExecutionPhase::Proof,
        );
        assert_eq!(
            stack,
            vec![Term::Proved(implies(exists(0, symbol(0)), symbol(1)))]
        );
        assert_eq!(memory, vec![]);
        assert_eq!(claims, vec![]);

        // TODO: Test case for when 0 is not fresh in rhs
    }

    #[test]
    fn test_apply_esubst() {
        // Define test cases as tuples of input pattern, evar_id, plug, and expected pattern
        let test_cases: Vec<(Ptr<Pattern>, u8, Ptr<Pattern>, Ptr<Pattern>)> = vec![
            // Atomic cases
            (bot(), 0, symbol(1), bot()),
            (evar(0), 0, symbol(1), symbol(1)),
            (evar(0), 0, evar(2), evar(2)),
            (evar(0), 1, evar(2), evar(0)),
            (svar(0), 0, symbol(0), svar(0)),
            (svar(1), 0, evar(0), svar(1)),
            (symbol(0), 0, symbol(1), symbol(0)),
            // Distribute over subpatterns
            (
                implies(evar(7), symbol(1)),
                7,
                symbol(0),
                implies(symbol(0), symbol(1)),
            ),
            (
                implies(evar(7), symbol(1)),
                6,
                symbol(0),
                implies(evar(7), symbol(1)),
            ),
            (
                app(evar(7), symbol(1)),
                7,
                symbol(0),
                app(symbol(0), symbol(1)),
            ),
            (
                app(evar(7), symbol(1)),
                6,
                symbol(0),
                app(evar(7), symbol(1)),
            ),
            // Distribute over subpatterns unless evar_id = binder
            (exists(1, evar(1)), 0, symbol(2), exists(1, evar(1))),
            (exists(0, evar(1)), 1, symbol(2), exists(0, symbol(2))),
            (mu(1, evar(1)), 0, symbol(2), mu(1, evar(1))),
            (mu(1, evar(1)), 1, symbol(2), mu(1, symbol(2))),
            // Subst on metavar should wrap in constructor
            (
                metavar_unconstrained(0),
                0,
                symbol(1),
                esubst(metavar_unconstrained(0), 0, symbol(1)),
            ),
            // Subst when evar_id is fresh should do nothing
            (
                metavar_e_fresh(0, 0, vec![], vec![]),
                0,
                symbol(1),
                metavar_e_fresh(0, 0, vec![], vec![]),
            ),
            // Subst when evar_id = evar(plug) should do nothing
            (
                metavar_unconstrained(0),
                1,
                evar(1),
                metavar_unconstrained(0),
            ),
            // Subst on substs should stack
            (
                esubst(metavar_unconstrained(0), 0, symbol(1)),
                1,
                symbol(1),
                esubst(esubst(metavar_unconstrained(0), 0, symbol(1)), 1, symbol(1)),
            ),
            (
                ssubst(metavar_unconstrained(0), 0, symbol(1)),
                0,
                symbol(1),
                esubst(ssubst(metavar_unconstrained(0), 0, symbol(1)), 0, symbol(1)),
            ),
        ];

        // Iterate over the test cases
        for (pattern, evar_id, plug, expected) in test_cases.iter() {
            // Call the apply_esubst function (replace with your Rust code)
            let result = apply_esubst(*pattern, *evar_id, *plug); // Replace with the actual function or code you want to test

            // Assert that the result matches the expected value
            assert_eq!(result, *expected);
        }
    }

    #[rstest]
    // Test that eVar substitution is capture avoiding
    #[should_panic]
    #[case(exists(0, evar(1)), 1, evar(0))]
    #[should_panic]
    #[case(mu(0, evar(1)), 1, svar(0))]
    fn test_apply_esubst_negative(
        #[case] pattern: Ptr<Pattern>,
        #[case] evar_id: Id,
        #[case] plug: Ptr<Pattern>,
    ) {
        _ = apply_esubst(pattern, evar_id, plug);
    }

    #[test]
    fn test_apply_ssubst() {
        let test_cases: Vec<(Ptr<Pattern>, u8, Ptr<Pattern>, Ptr<Pattern>)> = vec![
            // Atomic cases
            (bot(), 0, symbol(1), bot()),
            (evar(0), 0, symbol(1), evar(0)),
            (evar(0), 1, evar(2), evar(0)),
            (svar(0), 0, symbol(0), symbol(0)),
            (svar(1), 0, evar(0), svar(1)),
            (symbol(0), 0, symbol(1), symbol(0)),
            // Distribute over subpatterns
            (
                implies(svar(7), symbol(1)),
                7,
                symbol(0),
                implies(symbol(0), symbol(1)),
            ),
            (
                implies(svar(7), symbol(1)),
                6,
                symbol(0),
                implies(svar(7), symbol(1)),
            ),
            (
                app(svar(7), symbol(1)),
                7,
                symbol(0),
                app(symbol(0), symbol(1)),
            ),
            (
                app(svar(7), symbol(1)),
                6,
                symbol(0),
                app(svar(7), symbol(1)),
            ),
            // Distribute over subpatterns unless svar_id = binder
            (exists(1, svar(0)), 0, symbol(2), exists(1, symbol(2))),
            (exists(1, symbol(1)), 1, symbol(2), exists(1, symbol(1))),
            (mu(1, svar(1)), 0, symbol(2), mu(1, svar(1))),
            (mu(1, svar(1)), 1, symbol(2), mu(1, svar(1))),
            (mu(1, svar(2)), 2, symbol(2), mu(1, symbol(2))),
            // Subst on metavar should wrap in constructor
            (
                metavar_unconstrained(0),
                0,
                symbol(1),
                ssubst(metavar_unconstrained(0), 0, symbol(1)),
            ),
            // Subst when svar_id is fresh should do nothing
            (
                metavar_s_fresh(0, 0, vec![], vec![]),
                0,
                symbol(1),
                metavar_s_fresh(0, 0, vec![], vec![]),
            ),
            // Subst when svar_id = svar(plug) should do nothing
            (
                metavar_unconstrained(0),
                1,
                svar(1),
                metavar_unconstrained(0),
            ), // Subst on substs should stack
            (
                esubst(metavar_unconstrained(0), 0, symbol(1)),
                0,
                symbol(1),
                ssubst(esubst(metavar_unconstrained(0), 0, symbol(1)), 0, symbol(1)),
            ),
            (
                ssubst(metavar_unconstrained(0), 0, symbol(1)),
                1,
                symbol(1),
                ssubst(ssubst(metavar_unconstrained(0), 0, symbol(1)), 1, symbol(1)),
            ),
        ];

        for (pattern, svar_id, plug, expected) in test_cases {
            assert_eq!(apply_ssubst(pattern, svar_id, plug), expected);
        }
    }

    #[rstest]
    // Test that sVar substitution is capture avoiding
    #[should_panic]
    #[case(exists(0, svar(1)), 1, evar(0))]
    #[should_panic]
    #[case(mu(0, svar(1)), 1, svar(0))]
    fn test_apply_ssubst_negative(
        #[case] pattern: Ptr<Pattern>,
        #[case] svar_id: Id,
        #[case] plug: Ptr<Pattern>,
    ) {
        _ = apply_ssubst(pattern, svar_id, plug);
    }

    #[rstest]
    #[case(metavar_unconstrained(0), 0, evar(0), true)]
    #[case(metavar_unconstrained(0), 0, evar(1), false)]
    #[case(metavar_e_fresh(0, 0, vec![], vec![]), 0, symbol(0), true)]
    #[case(metavar_e_fresh(0, 1, vec![], vec![]), 0, symbol(0), false)]
    fn test_redundant_esubst(
        #[case] pattern: Ptr<Pattern>,
        #[case] evar_id: Id,
        #[case] plug: Ptr<Pattern>,
        #[case] expected: bool,
    ) {
        assert_eq!(pattern.is_redundant_esubst(evar_id, plug), expected);
    }

    #[rstest]
    #[case(metavar_unconstrained(0), 0, svar(0), true)]
    #[case(metavar_unconstrained(0), 0, svar(1), false)]
    #[case(metavar_s_fresh(0, 0, vec![], vec![]), 0, symbol(0), true)]
    #[case(metavar_s_fresh(0, 1, vec![], vec![]), 0, symbol(0), false)]
    fn test_redundant_ssubst(
        #[case] pattern: Ptr<Pattern>,
        #[case] svar_id: Id,
        #[case] plug: Ptr<Pattern>,
        #[case] expected: bool,
    ) {
        assert_eq!(pattern.is_redundant_ssubst(svar_id, plug), expected);
    }

    #[test]
    #[should_panic]
    fn test_no_remaining_claims() {
        let gamma = vec![];
        let claims = vec![
            Instruction::Symbol as InstByte,
            0u8,
            Instruction::Publish as InstByte,
        ];
        let proof = vec![];

        verify(&gamma, &claims, &proof);
    }
}
