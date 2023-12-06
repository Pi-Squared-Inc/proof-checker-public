#![deny(warnings)]
#![no_std]

extern crate alloc;

use alloc::rc::Rc;
use alloc::vec;
use alloc::vec::Vec;

/// Instructions
/// ============
///
/// Instructions are used to define the on-the-wire representation of matching
/// logic proofs.

#[rustfmt::skip]
#[derive(Debug, Eq, PartialEq)]
pub enum Instruction {
    // Patterns
    EVar = 2, SVar, Symbol, Implies, App, Exists, Mu,
    // Meta Patterns,
    MetaVar, ESubst, SSubst,
    // Axiom Schemas,
    Prop1, Prop2, Prop3, Quantifier, PropagationOr, PropagationExists,
    PreFixpoint, Existence, Singleton,
    // Inference rules,
    ModusPonens, Generalization, Frame, Substitution, KnasterTarski,
    // Meta Incference rules,
    Instantiate,
    // Stack Manipulation,
    Pop,
    // Memory Manipulation,
    Save, Load,
    // Journal Manipulation,
    Publish,
    // Metavar with no constraints
    CleanMetaVar = (9 + 128)
}

type InstByte = u8;
type InstrIterator<'a> = core::slice::Iter<'a, InstByte>;

impl Instruction {
    fn from(value: InstByte) -> Instruction {
        match value {
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
            // Sub-instructions
            137 => Instruction::CleanMetaVar,
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

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Pattern {
    EVar(Id),
    SVar(Id),
    Symbol(Id),
    Implies {
        left: Rc<Pattern>,
        right: Rc<Pattern>,
    },
    App {
        left: Rc<Pattern>,
        right: Rc<Pattern>,
    },
    Exists {
        var: Id,
        subpattern: Rc<Pattern>,
    },
    Mu {
        var: Id,
        subpattern: Rc<Pattern>,
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
        pattern: Rc<Pattern>,
        evar_id: Id,
        plug: Rc<Pattern>,
    },
    SSubst {
        pattern: Rc<Pattern>,
        svar_id: Id,
        plug: Rc<Pattern>,
    },
}

impl Pattern {
    fn e_fresh(&self, evar: Id) -> bool {
        match self {
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
            Pattern::EVar(_) => true,
            Pattern::SVar(_) => true,
            Pattern::Symbol(_) => true,
            Pattern::MetaVar { positive, .. } => positive.contains(&svar),
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
            Pattern::EVar(_) => true,
            Pattern::SVar(name) => *name != svar,
            Pattern::Symbol(_) => true,
            Pattern::MetaVar { negative, .. } => negative.contains(&svar),
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
            _ => {
                // TODO: If we make sure that we only use well-formed above constructs, then we should not need to check recursively
                unimplemented!(
                    "Well-formedness checking is unimplemented yet for this kind of pattern."
                );
            }
        }
    }

    fn is_redundant_subst(&self) -> bool {
        match self {
            Pattern::ESubst {
                pattern, evar_id, ..
            } => pattern.e_fresh(*evar_id),
            Pattern::SSubst {
                pattern, svar_id, ..
            } => pattern.s_fresh(*svar_id),
            _ => false,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Term {
    Pattern(Rc<Pattern>),
    Proved(Rc<Pattern>),
}
#[derive(Debug, Eq, PartialEq)]
pub enum Entry {
    Pattern(Rc<Pattern>),
    Proved(Rc<Pattern>),
}

/// Pattern construction utilities
/// ------------------------------

fn evar(id: Id) -> Rc<Pattern> {
    return Rc::new(Pattern::EVar(id));
}

fn svar(id: Id) -> Rc<Pattern> {
    return Rc::new(Pattern::SVar(id));
}

fn symbol(id: Id) -> Rc<Pattern> {
    return Rc::new(Pattern::Symbol(id));
}

fn metavar_unconstrained(var_id: Id) -> Rc<Pattern> {
    return Rc::new(Pattern::MetaVar {
        id: var_id,
        e_fresh: vec![],
        s_fresh: vec![],
        positive: vec![],
        negative: vec![],
        app_ctx_holes: vec![],
    });
}

#[cfg(test)]
fn metavar_s_fresh(var_id: Id, fresh: Id, positive: IdList, negative: IdList) -> Rc<Pattern> {
    return Rc::new(Pattern::MetaVar {
        id: var_id,
        e_fresh: vec![],
        s_fresh: vec![fresh],
        positive,
        negative,
        app_ctx_holes: vec![],
    });
}

#[inline(always)]
fn exists(var: Id, subpattern: Rc<Pattern>) -> Rc<Pattern> {
    return Rc::new(Pattern::Exists { var, subpattern });
}

// Does not do any well-formedness checks!!!!!
#[inline(always)]
fn mu(var: Id, subpattern: Rc<Pattern>) -> Rc<Pattern> {
    return Rc::new(Pattern::Mu { var, subpattern });
}

#[inline(always)]
fn esubst(pattern: Rc<Pattern>, evar_id: Id, plug: Rc<Pattern>) -> Rc<Pattern> {
    return Rc::new(Pattern::ESubst {
        pattern,
        evar_id,
        plug,
    });
}

#[inline(always)]
fn ssubst(pattern: Rc<Pattern>, svar_id: Id, plug: Rc<Pattern>) -> Rc<Pattern> {
    return Rc::new(Pattern::SSubst {
        pattern,
        svar_id,
        plug,
    });
}

#[inline(always)]
fn implies(left: Rc<Pattern>, right: Rc<Pattern>) -> Rc<Pattern> {
    return Rc::new(Pattern::Implies { left, right });
}

#[inline(always)]
fn app(left: Rc<Pattern>, right: Rc<Pattern>) -> Rc<Pattern> {
    return Rc::new(Pattern::App { left, right });
}

// Notation
#[inline(always)]
fn bot() -> Rc<Pattern> {
    mu(0, svar(0))
}

#[inline(always)]
fn not(pat: Rc<Pattern>) -> Rc<Pattern> {
    implies(pat, bot())
}

#[allow(dead_code)]
fn forall(evar: Id, pat: Rc<Pattern>) -> Rc<Pattern> {
    not(exists(evar, not(pat)))
}

/// Substitution utilities
/// ----------------------

fn apply_esubst(pattern: &Rc<Pattern>, evar_id: Id, plug: &Rc<Pattern>) -> Rc<Pattern> {
    // Wraps pattern in appropriate ESubst
    let wrap_subst = || esubst(Rc::clone(pattern), evar_id, Rc::clone(plug));

    match pattern.as_ref() {
        Pattern::EVar(e) => {
            if *e == evar_id {
                Rc::clone(plug)
            } else {
                Rc::clone(pattern)
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
        Pattern::Exists { var, .. } if *var == evar_id => Rc::clone(pattern),
        Pattern::Exists { var, subpattern } => {
            assert!(
                plug.e_fresh(*var),
                "EVar substitution would capture free variable {}!",
                var
            );
            exists(*var, apply_esubst(subpattern, evar_id, plug))
        }
        Pattern::Mu { var, subpattern } => mu(*var, apply_esubst(subpattern, evar_id, plug)),
        Pattern::ESubst { .. } => wrap_subst(),
        Pattern::SSubst { .. } => wrap_subst(),
        Pattern::MetaVar { .. } => wrap_subst(),
        _ => Rc::clone(pattern),
    }
}

fn apply_ssubst(pattern: &Rc<Pattern>, svar_id: Id, plug: &Rc<Pattern>) -> Rc<Pattern> {
    // Wraps pattern in appropriate ESubst
    let wrap_subst = || ssubst(Rc::clone(pattern), svar_id, Rc::clone(plug));

    match pattern.as_ref() {
        Pattern::SVar(s) => {
            if *s == svar_id {
                Rc::clone(plug)
            } else {
                Rc::clone(pattern)
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
            exists(*var, apply_ssubst(subpattern, svar_id, plug))
        }
        Pattern::Mu { var, .. } if *var == svar_id => Rc::clone(pattern),
        Pattern::Mu { var, subpattern } => {
            assert!(
                plug.s_fresh(*var),
                "SVar substitution would capture free variable {}!",
                var
            );
            mu(*var, apply_ssubst(subpattern, svar_id, plug))
        }
        Pattern::ESubst { .. } => wrap_subst(),
        Pattern::SSubst { .. } => wrap_subst(),
        Pattern::MetaVar { .. } => wrap_subst(),
        _ => Rc::clone(pattern),
    }
}

fn instantiate_internal(
    p: &Rc<Pattern>,
    vars: &[Id],
    plugs: &[Rc<Pattern>],
) -> Option<Rc<Pattern>> {
    match p.as_ref() {
        Pattern::EVar(_) => None,
        Pattern::SVar(_) => None,
        Pattern::Symbol(_) => None,
        Pattern::MetaVar {
            id,
            e_fresh,
            s_fresh,
            positive,
            negative,
            ..
        } => {
            if let Some(pos) = vars.iter().position(|&x| x == *id) {
                if let Some(evar) = e_fresh.into_iter().find(|&evar| !plugs[pos].e_fresh(*evar)) {
                    panic!(
                        "Instantiation of MetaVar {} breaks a freshness constraint: EVar {}",
                        id, evar
                    );
                }
                if let Some(svar) = s_fresh.into_iter().find(|&svar| !plugs[pos].s_fresh(*svar)) {
                    panic!(
                        "Instantiation of MetaVar {} breaks a freshness constraint: SVar {}",
                        id, svar
                    );
                }
                if let Some(svar) = positive
                    .into_iter()
                    .find(|&svar| !plugs[pos].positive(*svar))
                {
                    panic!(
                        "Instantiation of MetaVar {} breaks a positivity constraint: SVar {}",
                        id, svar
                    );
                }
                if let Some(svar) = negative
                    .into_iter()
                    .find(|&svar| !plugs[pos].negative(*svar))
                {
                    panic!(
                        "Instantiation of MetaVar {} breaks a negativity constraint: SVar {}",
                        id, svar
                    );
                }

                if pos >= plugs.len() {
                    panic!("Substitution does not contain a corresponding value.")
                }
                return Some(Rc::clone(&plugs[pos]));
            }
            None
        }
        Pattern::Implies { left, right } => {
            let mut inst_left = instantiate_internal(&left, vars, plugs);
            let mut inst_right = instantiate_internal(&right, vars, plugs);
            if inst_left.is_none() && inst_right.is_none() {
                None
            } else {
                if inst_left.is_none() {
                    inst_left = Some(Rc::clone(left));
                }
                if inst_right.is_none() {
                    inst_right = Some(Rc::clone(right));
                }
                Some(implies(inst_left.unwrap(), inst_right.unwrap()))
            }
        }
        Pattern::App { left, right } => {
            let mut inst_left = instantiate_internal(&left, vars, plugs);
            let mut inst_right = instantiate_internal(&right, vars, plugs);
            if inst_left.is_none() && inst_right.is_none() {
                None
            } else {
                if inst_left.is_none() {
                    inst_left = Some(Rc::clone(left));
                }
                if inst_right.is_none() {
                    inst_right = Some(Rc::clone(right));
                }
                Some(app(inst_left.unwrap(), inst_right.unwrap()))
            }
        }
        Pattern::Exists { var, subpattern } => {
            let new_sub = instantiate_internal(&subpattern, vars, plugs);
            Some(exists(*var, new_sub?))
        }
        Pattern::Mu { var, subpattern } => {
            let new_sub = instantiate_internal(&subpattern, vars, plugs);
            Some(mu(*var, new_sub?))
        }
        Pattern::ESubst {
            pattern,
            evar_id,
            plug,
        } => {
            let mut inst_pattern = instantiate_internal(pattern, vars, plugs);
            let mut inst_plug = instantiate_internal(plug, vars, plugs);
            if inst_pattern.is_none() && inst_plug.is_none() {
                None
            } else {
                if inst_pattern.is_none() {
                    inst_pattern = Some(Rc::clone(pattern));
                }
                if inst_plug.is_none() {
                    inst_plug = Some(Rc::clone(plug));
                }
                Some(apply_esubst(
                    &inst_pattern.unwrap(),
                    *evar_id,
                    &inst_plug.unwrap(),
                ))
            }
        }
        Pattern::SSubst {
            pattern,
            svar_id,
            plug,
        } => {
            let mut inst_pattern = instantiate_internal(pattern, vars, plugs);
            let mut inst_plug = instantiate_internal(plug, vars, plugs);
            if inst_pattern.is_none() && inst_plug.is_none() {
                None
            } else {
                if inst_pattern.is_none() {
                    inst_pattern = Some(Rc::clone(pattern));
                }
                if inst_plug.is_none() {
                    inst_plug = Some(Rc::clone(plug));
                }
                Some(apply_ssubst(
                    &inst_pattern.unwrap(),
                    *svar_id,
                    &inst_plug.unwrap(),
                ))
            }
        }
    }
}

fn instantiate_in_place(p: &mut Rc<Pattern>, vars: &[Id], plugs: &[Rc<Pattern>]) {
    if let Some(ret) = instantiate_internal(p, vars, plugs) {
        *p = ret
    }
}

/// Proof checker
/// =============

type Stack = Vec<Term>;
type Claims = Vec<Rc<Pattern>>;
type Memory = Vec<Entry>;

/// Stack utilities
/// ---------------

fn pop_stack(stack: &mut Stack) -> Term {
    return stack.pop().expect("Insufficient stack items.");
}

fn pop_stack_pattern(stack: &mut Stack) -> Rc<Pattern> {
    match pop_stack(stack) {
        Term::Pattern(p) => return p,
        _ => panic!("Expected pattern on stack."),
    }
}

fn pop_stack_proved(stack: &mut Stack) -> Rc<Pattern> {
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
    let prop1 = implies(
        Rc::clone(&phi0),
        implies(Rc::clone(&phi1), Rc::clone(&phi0)),
    );
    let prop2 = implies(
        implies(
            Rc::clone(&phi0),
            implies(Rc::clone(&phi1), Rc::clone(&phi2)),
        ),
        implies(
            implies(Rc::clone(&phi0), phi1),
            implies(Rc::clone(&phi0), phi2),
        ),
    );
    let prop3 = implies(not(not(Rc::clone(&phi0))), Rc::clone(&phi0));
    let quantifier = implies(esubst(Rc::clone(&phi0), 0, evar(1)), exists(0, phi0));

    let existence = exists(0, evar(0));

    while let Some(instr_u32) = iterator.next() {
        match Instruction::from(*instr_u32) {
            // TODO: Add an abstraction for pushing these one-argument terms on stack?
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

                let metavar_pat = Rc::new(Pattern::MetaVar {
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

                let metavar_pat = Rc::new(Pattern::MetaVar {
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

                match pattern.as_ref() {
                    Pattern::MetaVar { .. } | Pattern::ESubst { .. } | Pattern::SSubst { .. } => (),
                    _ => panic!("Cannot apply ESubst on concrete term!"),
                }

                let esubst_pat = esubst(Rc::clone(&pattern), evar_id, plug);
                stack.push(Term::Pattern(if esubst_pat.is_redundant_subst() {
                    pattern
                } else {
                    esubst_pat
                }));
            }

            Instruction::SSubst => {
                let svar_id = *iterator
                    .next()
                    .expect("Insufficient parameters for SSubst instruction.")
                    as Id;
                let pattern = pop_stack_pattern(stack);
                let plug = pop_stack_pattern(stack);

                match pattern.as_ref() {
                    Pattern::MetaVar { .. } | Pattern::ESubst { .. } | Pattern::SSubst { .. } => (),
                    _ => panic!("Cannot apply SSubst on concrete term!"),
                }

                let ssubst_pat = ssubst(Rc::clone(&pattern), svar_id, plug);
                stack.push(Term::Pattern(if ssubst_pat.is_redundant_subst() {
                    pattern
                } else {
                    ssubst_pat
                }));
            }

            Instruction::Prop1 => {
                stack.push(Term::Proved(Rc::clone(&prop1)));
            }
            Instruction::Prop2 => {
                stack.push(Term::Proved(Rc::clone(&prop2)));
            }
            Instruction::Prop3 => {
                stack.push(Term::Proved(Rc::clone(&prop3)));
            }
            Instruction::ModusPonens => {
                let premise2 = pop_stack_proved(stack);
                let premise1 = pop_stack_proved(stack);
                match premise1.as_ref() {
                    Pattern::Implies { left, right } => {
                        if *left.as_ref() != *premise2.as_ref() {
                            panic!("Antecedents do not match for modus ponens.\nleft.psi:\n{:?}\n\n right:\n{:?}\n", left.as_ref(), premise2.as_ref())
                        }
                        stack.push(Term::Proved(Rc::clone(&right)))
                    }
                    _ => {
                        panic!("Expected an implication as a first parameter.")
                    }
                }
            }
            Instruction::Quantifier => {
                stack.push(Term::Proved(Rc::clone(&quantifier)));
            }
            Instruction::Generalization => match pop_stack_proved(stack).as_ref() {
                Pattern::Implies { left, right } => {
                    let evar_id = *iterator
                        .next()
                        .expect("Insufficient parameters for Generalization instruction")
                        as Id;

                    if !right.e_fresh(evar_id) {
                        panic!("The binding variable has to be fresh in the conclusion.");
                    }

                    stack.push(Term::Proved(implies(
                        exists(evar_id, Rc::clone(left)),
                        Rc::clone(right),
                    )));
                }
                _ => {
                    panic!("Expected an implication as a first parameter.")
                }
            },
            Instruction::Existence => {
                stack.push(Term::Proved(Rc::clone(&existence)));
            }
            Instruction::Substitution => {
                let svar_id = *iterator
                    .next()
                    .expect("Insufficient parameters for Substitution instruction.");
                let pattern = pop_stack_proved(stack);
                let plug = pop_stack_pattern(stack);

                stack.push(Term::Proved(apply_ssubst(&pattern, svar_id, &plug)));
            }
            Instruction::Instantiate => {
                let n = *iterator
                    .next()
                    .expect("Insufficient parameters for Instantiate instruction")
                    as usize;
                let mut ids: IdList = Vec::with_capacity(n);
                let mut plugs: Vec<Rc<Pattern>> = Vec::with_capacity(n);

                let metaterm = pop_stack(stack);

                iterator.take(n).for_each(|arg| {
                    ids.push(*arg as Id);
                    plugs.push(pop_stack_pattern(stack))
                });

                match metaterm {
                    Term::Pattern(mut p) => {
                        instantiate_in_place(&mut p, &ids, &plugs);
                        stack.push(Term::Pattern(p));
                    }
                    Term::Proved(mut p) => {
                        instantiate_in_place(&mut p, &ids, &plugs);
                        stack.push(Term::Proved(p));
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
                ExecutionPhase::Gamma => memory.push(Entry::Proved(pop_stack_pattern(stack))),
                ExecutionPhase::Claim => {
                    let claim = pop_stack_pattern(stack);
                    claims.push(claim)
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
        #[case] expected_pattern: Rc<Pattern>,
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
        let evar = evar(1);
        let left = Rc::new(Pattern::Exists {
            var: 1,
            subpattern: evar.clone(),
        });
        assert!(left.e_fresh(1));

        let right = Rc::new(Pattern::Exists {
            var: 2,
            subpattern: evar,
        });
        assert!(!right.e_fresh(1));

        let implication = implies(Rc::clone(&left), Rc::clone(&right));
        assert!(!implication.e_fresh(1));

        let mvar = metavar_s_fresh(1, 2, vec![2], vec![2]);
        let metaapp = Pattern::App {
            left: Rc::clone(&left),
            right: mvar,
        };
        assert!(!metaapp.e_fresh(2));

        let esubst_ = esubst(Rc::clone(&right), 1, Rc::clone(&left));
        assert!(esubst_.e_fresh(1));

        let ssubst_ = ssubst(Rc::clone(&right), 1, left);
        assert!(!ssubst_.e_fresh(1));
    }

    #[test]
    fn test_sfresh() {
        let svar = svar(1);
        let left = Rc::new(Pattern::Mu {
            var: 1,
            subpattern: svar.clone(),
        });
        assert!(left.s_fresh(1));

        let right = Rc::new(Pattern::Mu {
            var: 2,
            subpattern: svar,
        });
        assert!(!right.s_fresh(1));

        let implication = implies(Rc::clone(&left), Rc::clone(&right));
        assert!(!implication.s_fresh(1));

        let mvar = metavar_s_fresh(1, 2, vec![2], vec![2]);
        let metaapp = Pattern::App {
            left: Rc::clone(&left),
            right: Rc::clone(&mvar),
        };
        assert!(!metaapp.s_fresh(1));

        let metaapp2 = Pattern::App {
            left: Rc::clone(&left),
            right: mvar,
        };
        assert!(metaapp2.s_fresh(2));

        let esubst_ = esubst(Rc::clone(&right), 1, Rc::clone(&left));
        assert!(!esubst_.s_fresh(1));

        let ssubst_ = ssubst(Rc::clone(&right), 1, left);
        assert!(ssubst_.s_fresh(1));
    }

    #[test]
    #[should_panic]
    fn test_instantiate_fresh() {
        let svar_0 = svar(0);
        let phi0_s_fresh_0 = metavar_s_fresh(0, 0, vec![0], vec![0]);
        instantiate_internal(&phi0_s_fresh_0, &[0], &[svar_0]);
    }

    #[test]
    fn test_wellformedness_fresh() {
        let phi0_s_fresh_0 = metavar_s_fresh(0, 0, vec![0], vec![0]);
        assert!(phi0_s_fresh_0.well_formed());

        let phi1 = Rc::new(Pattern::MetaVar {
            id: 1,
            e_fresh: vec![1, 2, 0],
            s_fresh: vec![],
            positive: vec![],
            negative: vec![],
            app_ctx_holes: vec![2],
        });
        assert!(!phi1.well_formed());

        // TODO: Reason why this is not needed
        // let phi1_imp_phi1 = implies(Rc::clone(&phi1), Rc::clone(&phi1));
        // assert!(!phi1_imp_phi1.well_formed());
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_positivity() {
        let X0 = svar(0);
        let X1 = svar(1);
        let X2 = svar(2);
        let c1 = symbol(1);
        let neg_X1 = not(Rc::clone(&X1));

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
        let appX1X2 = app(Rc::clone(&X1), Rc::clone(&X2));
        assert!(appX1X2.positive(1));
        assert!(appX1X2.positive(2));
        assert!(appX1X2.positive(3));
        assert!(!appX1X2.negative(1));
        assert!(!appX1X2.negative(2));
        assert!(appX1X2.negative(3));

        // Implies
        let impliesX1X2 = implies(Rc::clone(&X1), Rc::clone(&X2));
        assert!(!impliesX1X2.positive(1));
        assert!(impliesX1X2.positive(2));
        assert!(impliesX1X2.positive(3));
        assert!(impliesX1X2.negative(1));
        assert!(!impliesX1X2.negative(2));
        assert!(impliesX1X2.negative(3));

        let impliesX1X1 = implies(Rc::clone(&X1), Rc::clone(&X1));
        assert!(!impliesX1X1.positive(1));
        assert!(!impliesX1X1.negative(1));

        // Exists
        let existsX1X2 = exists(1, Rc::clone(&X2));
        assert!(existsX1X2.positive(1));
        assert!(existsX1X2.positive(2));
        assert!(existsX1X2.positive(3));
        assert!(existsX1X2.negative(1));
        assert!(!existsX1X2.negative(2));
        assert!(existsX1X2.negative(3));

        let existsX1nX2 = exists(1, not(Rc::clone(&X2)));
        assert!(existsX1nX2.negative(2));

        // Mu
        let muX1x1 = mu(1, Rc::clone(&evar1));
        assert!(muX1x1.positive(1));
        assert!(muX1x1.positive(2));
        assert!(muX1x1.negative(1));
        assert!(muX1x1.negative(2));

        let muX1X1 = mu(1, Rc::clone(&X1));
        assert!(muX1X1.positive(1));
        assert!(muX1X1.negative(1));

        let muX1X2 = mu(1, Rc::clone(&X2));
        assert!(muX1X2.positive(1));
        assert!(muX1X2.positive(2));
        assert!(muX1X2.positive(3));
        assert!(muX1X2.negative(1));
        assert!(!muX1X2.negative(2));
        assert!(mu(1, implies(Rc::clone(&X2), Rc::clone(&X1))).negative(2));
        assert!(muX1X2.negative(3));

        // MetaVar
        assert!(!metavar_unconstrained(1).positive(1));
        assert!(!metavar_unconstrained(1).positive(2));
        assert!(!metavar_unconstrained(1).negative(1));
        assert!(!metavar_unconstrained(1).negative(2));

        // Do not imply positivity from freshness
        assert!(!metavar_s_fresh(1, 1, vec![], vec![]).positive(1));
        assert!(!metavar_s_fresh(1, 1, vec![], vec![]).negative(1));
        assert!(metavar_s_fresh(1, 1, vec![1], vec![1]).positive(1));
        assert!(metavar_s_fresh(1, 1, vec![1], vec![1]).negative(1));
        assert!(metavar_s_fresh(1, 1, vec![1], vec![]).positive(1));
        assert!(metavar_s_fresh(1, 1, vec![], vec![1]).negative(1));

        assert!(!metavar_s_fresh(1, 1, vec![], vec![]).positive(2));
        assert!(!metavar_s_fresh(1, 1, vec![], vec![]).negative(2));

        // ESubst
        assert!(!esubst(metavar_unconstrained(0), 0, Rc::clone(&X0)).positive(0));
        assert!(!esubst(metavar_unconstrained(0), 0, Rc::clone(&X1)).positive(0));
        assert!(!esubst(metavar_s_fresh(0, 1, vec![1], vec![]), 0, Rc::clone(&X1)).positive(0));

        assert!(!esubst(metavar_unconstrained(0), 0, Rc::clone(&X0)).negative(0));
        assert!(!esubst(metavar_unconstrained(0), 0, Rc::clone(&X1)).negative(0));
        assert!(!esubst(metavar_s_fresh(0, 1, vec![1], vec![]), 0, Rc::clone(&X1)).negative(0));

        // SSubst
        assert!(!ssubst(metavar_unconstrained(0), 0, Rc::clone(&X0)).positive(0));
        assert!(ssubst(metavar_unconstrained(0), 0, Rc::clone(&X1)).positive(0));
        assert!(ssubst(metavar_s_fresh(0, 1, vec![1], vec![]), 0, Rc::clone(&X1)).positive(0));

        assert!(!ssubst(metavar_unconstrained(0), 0, Rc::clone(&X0)).negative(0));
        assert!(ssubst(metavar_unconstrained(0), 0, Rc::clone(&X1)).negative(0));
        assert!(ssubst(metavar_s_fresh(0, 1, vec![1], vec![]), 0, Rc::clone(&X1)).negative(0));

        // Combinations
        assert!(!neg_X1.positive(1));
        assert!(neg_X1.positive(2));
        assert!(neg_X1.negative(1));
        assert!(neg_X1.negative(2));

        let negX1_implies_negX1 = implies(Rc::clone(&neg_X1), Rc::clone(&neg_X1));
        assert!(!negX1_implies_negX1.positive(1));
        assert!(negX1_implies_negX1.positive(2));
        assert!(!negX1_implies_negX1.negative(1));
        assert!(negX1_implies_negX1.negative(2));

        let negX1_implies_X1 = implies(Rc::clone(&neg_X1), Rc::clone(&X1));
        assert!(negX1_implies_X1.positive(1));
        assert!(!negX1_implies_X1.negative(1));
    }

    #[test]
    fn test_wellformedness_positive() {
        let svar = svar(1);
        let mux_x = mu(1, Rc::clone(&svar));
        assert!(mux_x.well_formed());

        let mux_x2 = mu(2, not(Rc::clone(&svar)));
        assert!(mux_x2.well_formed());

        let mux_x3 = mu(2, not(symbol(1)));
        assert!(mux_x3.well_formed());

        let mux_x = mu(1, not(Rc::clone(&svar)));
        assert!(!mux_x.well_formed());

        let phi = metavar_s_fresh(97, 2, vec![], vec![]);
        let mux_phi = mu(1, phi);
        assert!(!mux_phi.well_formed());

        // Even though freshness implies positivity, we do not want to do any
        // additional reasoning and let everything on the prover
        let phi2 = metavar_s_fresh(98, 1, vec![], vec![]);
        let mux_phi2 = mu(1, phi2);
        assert!(!mux_phi2.well_formed());

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
        let x0_implies_x0 = implies(Rc::clone(&x0), Rc::clone(&x0));
        let appx0x0 = app(Rc::clone(&x0), Rc::clone(&x0));
        let existsx0x0 = exists(0, Rc::clone(&x0));
        let muX0x0 = mu(0, Rc::clone(&x0));

        // Concrete patterns are unaffected by instantiate
        assert!(instantiate_internal(&x0, &[0], &[Rc::clone(&X0)]) == None);
        assert!(instantiate_internal(&x0, &[1], &[Rc::clone(&X0)]) == None);
        assert!(instantiate_internal(&X0, &[0], &[Rc::clone(&x0)]) == None);
        assert!(instantiate_internal(&X0, &[1], &[Rc::clone(&x0)]) == None);
        assert!(instantiate_internal(&c0, &[0], &[Rc::clone(&x0)]) == None);
        assert!(instantiate_internal(&c0, &[1], &[Rc::clone(&x0)]) == None);
        assert!(instantiate_internal(&x0_implies_x0, &[0], &[Rc::clone(&x0)]) == None);
        assert!(instantiate_internal(&x0_implies_x0, &[1], &[Rc::clone(&x0)]) == None);
        assert!(instantiate_internal(&appx0x0, &[0], &[Rc::clone(&x0)]) == None);
        assert!(instantiate_internal(&appx0x0, &[1], &[Rc::clone(&x0)]) == None);
        assert!(instantiate_internal(&existsx0x0, &[0], &[Rc::clone(&X0)]) == None);
        assert!(instantiate_internal(&existsx0x0, &[1], &[Rc::clone(&X0)]) == None);
        assert!(instantiate_internal(&muX0x0, &[0], &[Rc::clone(&x0)]) == None);
        assert!(instantiate_internal(&muX0x0, &[1], &[Rc::clone(&x0)]) == None);

        let phi0 = metavar_unconstrained(0);
        let phi0_implies_phi0 = implies(Rc::clone(&phi0), Rc::clone(&phi0));
        let appphi0phi0 = app(Rc::clone(&phi0), Rc::clone(&phi0));
        let existsx0phi0 = exists(0, Rc::clone(&phi0));
        let muX0phi0 = mu(0, Rc::clone(&phi0));
        let existsx0X0 = exists(0, Rc::clone(&X0));
        assert!(
            instantiate_internal(&phi0_implies_phi0, &[0], &[Rc::clone(&x0)])
                == Some(Rc::clone(&x0_implies_x0))
        );
        assert!(instantiate_internal(&phi0_implies_phi0, &[1], &[Rc::clone(&x0)]) == None);
        assert_eq!(
            instantiate_internal(&appphi0phi0, &[0], &[Rc::clone(&x0)]),
            Some(Rc::clone(&appx0x0))
        );
        assert_eq!(
            instantiate_internal(&appphi0phi0, &[1], &[Rc::clone(&x0)]),
            None
        );
        assert_eq!(
            instantiate_internal(&existsx0phi0, &[0], &[Rc::clone(&x0)]),
            Some(Rc::clone(&existsx0x0))
        );
        assert_eq!(
            instantiate_internal(&existsx0phi0, &[1], &[Rc::clone(&x0)]),
            None
        );
        assert_eq!(
            instantiate_internal(&muX0phi0, &[0], &[Rc::clone(&x0)]),
            Some(Rc::clone(&muX0x0))
        );
        assert_eq!(
            instantiate_internal(&muX0phi0, &[1], &[Rc::clone(&x0)]),
            None
        );

        // Simultaneous instantiations
        let phi1 = metavar_unconstrained(1);
        let muX0phi1 = mu(0, Rc::clone(&phi1));
        let muX0X0 = mu(0, Rc::clone(&X0));
        // Empty substs have no effect
        assert!(
            instantiate_internal(&existsx0phi0, &[1, 2], &[Rc::clone(&x0), Rc::clone(&X0)]) == None
        );
        assert!(
            instantiate_internal(&existsx0phi0, &[2, 1], &[Rc::clone(&x0), Rc::clone(&X0)]) == None
        );
        assert!(
            instantiate_internal(&muX0phi0, &[1, 2], &[Rc::clone(&x0), Rc::clone(&X0)]) == None
        );
        assert!(
            instantiate_internal(&muX0phi0, &[2, 1], &[Rc::clone(&x0), Rc::clone(&X0)]) == None
        );

        // Order matters if corresponding value is not moved
        assert!(
            instantiate_internal(&existsx0phi0, &[1, 0], &[Rc::clone(&x0), Rc::clone(&X0)])
                == Some(Rc::clone(&existsx0X0))
        );
        assert!(
            instantiate_internal(&existsx0phi0, &[0, 1], &[Rc::clone(&x0), Rc::clone(&X0)])
                == Some(Rc::clone(&existsx0x0))
        );
        assert!(
            instantiate_internal(&muX0phi0, &[1, 0], &[Rc::clone(&x0), Rc::clone(&X0)])
                == Some(Rc::clone(&muX0X0))
        );
        assert!(
            instantiate_internal(&muX0phi0, &[0, 1], &[Rc::clone(&x0), Rc::clone(&X0)])
                == Some(Rc::clone(&muX0x0))
        );

        // Order does not matter if corresponding value is moved
        let muX0phi0_implies_ph1 = implies(Rc::clone(&muX0phi0), Rc::clone(&phi1));
        let muX0x0_implies_X0 = implies(Rc::clone(&muX0x0), Rc::clone(&X0));
        assert!(
            instantiate_internal(
                &muX0phi0_implies_ph1,
                &[0, 1],
                &[Rc::clone(&x0), Rc::clone(&X0)]
            ) == Some(Rc::clone(&muX0x0_implies_X0))
        );
        assert!(
            instantiate_internal(
                &muX0phi0_implies_ph1,
                &[1, 0],
                &[Rc::clone(&X0), Rc::clone(&x0)]
            ) == Some(Rc::clone(&muX0x0_implies_X0))
        );
        let muX0phi0_app_ph1 = app(Rc::clone(&muX0phi0), Rc::clone(&phi1));
        let muX0x0_app_X0 = app(Rc::clone(&muX0x0), Rc::clone(&X0));
        assert!(
            instantiate_internal(
                &muX0phi0_app_ph1,
                &[0, 1],
                &[Rc::clone(&x0), Rc::clone(&X0)]
            ) == Some(Rc::clone(&muX0x0_app_X0))
        );
        assert!(
            instantiate_internal(
                &muX0phi0_app_ph1,
                &[1, 0],
                &[Rc::clone(&X0), Rc::clone(&x0)]
            ) == Some(Rc::clone(&muX0x0_app_X0))
        );

        // No side-effects
        let muX0ph1_implies_X0 = implies(Rc::clone(&muX0phi1), Rc::clone(&X0));
        assert!(
            instantiate_internal(
                &muX0phi0_implies_ph1,
                &[0, 1],
                &[Rc::clone(&phi1), Rc::clone(&X0)]
            ) == Some(Rc::clone(&muX0ph1_implies_X0))
        );
        assert!(
            instantiate_internal(
                &muX0phi0_implies_ph1,
                &[1, 0],
                &[Rc::clone(&X0), Rc::clone(&phi1)]
            ) == Some(Rc::clone(&muX0ph1_implies_X0))
        );
        let muX0ph1_app_X0 = app(Rc::clone(&muX0phi1), Rc::clone(&X0));
        assert!(
            instantiate_internal(
                &muX0phi0_app_ph1,
                &[0, 1],
                &[Rc::clone(&phi1), Rc::clone(&X0)]
            ) == Some(Rc::clone(&muX0ph1_app_X0))
        );
        assert!(
            instantiate_internal(
                &muX0phi0_app_ph1,
                &[1, 0],
                &[Rc::clone(&X0), Rc::clone(&phi1)]
            ) == Some(Rc::clone(&muX0ph1_app_X0))
        );

        // First comes first
        assert!(
            instantiate_internal(
                &muX0phi0_app_ph1,
                &[0, 1, 1],
                &[Rc::clone(&phi1), Rc::clone(&X0), Rc::clone(&x0)]
            ) == Some(Rc::clone(&muX0ph1_app_X0))
        );
        assert!(
            instantiate_internal(
                &muX0phi0_app_ph1,
                &[1, 0, 0],
                &[Rc::clone(&X0), Rc::clone(&phi1), Rc::clone(&x0)]
            ) == Some(Rc::clone(&muX0ph1_app_X0))
        );

        // Extra values are ignored
        assert!(
            instantiate_internal(
                &muX0phi0_app_ph1,
                &[0, 1, 1],
                &[
                    Rc::clone(&phi1),
                    Rc::clone(&X0),
                    Rc::clone(&x0),
                    Rc::clone(&x0),
                    Rc::clone(&x0),
                    Rc::clone(&x0),
                    Rc::clone(&x0),
                    Rc::clone(&x0)
                ]
            ) == Some(Rc::clone(&muX0ph1_app_X0))
        );
        assert!(
            instantiate_internal(
                &muX0phi0_app_ph1,
                &[0, 1, 2],
                &[Rc::clone(&phi1), Rc::clone(&X0)]
            ) == Some(Rc::clone(&muX0ph1_app_X0))
        );

        // Instantiate with concrete patterns applies pending substitutions
        let val = esubst(Rc::clone(&phi0), 0, Rc::clone(&c0));
        assert_eq!(
            instantiate_internal(&val, &[0], &[Rc::clone(&x0)]),
            Some(Rc::clone(&c0))
        );
        let val = ssubst(Rc::clone(&phi0), 0, Rc::clone(&c0));
        assert_eq!(
            instantiate_internal(&val, &[0], &[Rc::clone(&X0)]),
            Some(Rc::clone(&c0))
        );
        let val = ssubst(
            Rc::clone(&esubst(Rc::clone(&phi0), 0, Rc::clone(&X0))),
            0,
            Rc::clone(&c0),
        );
        assert_eq!(
            instantiate_internal(&val, &[0], &[Rc::clone(&X0)]),
            Some(Rc::clone(&c0))
        );

        // Instantiate with metavar keeps pending substitutions
        let val = esubst(Rc::clone(&phi0), 0, Rc::clone(&c0));
        assert_eq!(
            instantiate_internal(&val, &[0], &[Rc::clone(&phi1)]),
            Some(esubst(Rc::clone(&phi1), 0, Rc::clone(&c0)))
        );
        let val = ssubst(Rc::clone(&phi0), 0, Rc::clone(&c0));
        assert_eq!(
            instantiate_internal(&val, &[0], &[Rc::clone(&phi1)]),
            Some(ssubst(Rc::clone(&phi1), 0, Rc::clone(&c0)))
        );

        // The plug in a subst. needs to be instantiated as well
        let val = ssubst(Rc::clone(&phi0), 0, Rc::clone(&phi0));
        assert_eq!(
            instantiate_internal(&val, &[0], &[Rc::clone(&X0)]),
            Some(Rc::clone(&X0))
        );
        let val = ssubst(Rc::clone(&phi0), 0, Rc::clone(&phi1));
        assert_eq!(
            instantiate_internal(&val, &[0, 1], &[Rc::clone(&X0), Rc::clone(&c0)]),
            Some(c0)
        );
    }

    #[test]
    #[should_panic]
    fn test_illformed_instantiation() {
        let phi0 = metavar_unconstrained(0);
        instantiate_internal(&phi0, &[1, 0], &[Rc::clone(&phi0)]);
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
            vec![Term::Pattern(Rc::new(Pattern::Implies {
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

            let phi1 = Rc::new(Pattern::MetaVar {
                id: 1,
                e_fresh: cons[0].clone(),
                s_fresh: cons[1].clone(),
                positive: cons[2].clone(),
                negative: cons[3].clone(),
                app_ctx_holes: cons[4].clone(),
            });

            assert_eq!(
                stack,
                vec![Term::Pattern(Rc::new(Pattern::Implies {
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
            vec![Term::Proved(Rc::new(Pattern::Implies {
                left: Rc::clone(&phi0),
                right: Rc::clone(&phi0)
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
        let test_cases: Vec<(Rc<Pattern>, u8, Rc<Pattern>, Rc<Pattern>)> = vec![
            // Atomic cases
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
            //(metavar_(0).with_e_fresh((evar(0), evar(1))), 0, symbol(1), metavar_unconstrained(0).with_e_fresh((evar(0), evar(1)))),
            // Subst on substs should stack
            (
                esubst(metavar_unconstrained(0), 0, symbol(1)),
                0,
                symbol(1),
                esubst(esubst(metavar_unconstrained(0), 0, symbol(1)), 0, symbol(1)),
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
            let result = apply_esubst(pattern, *evar_id, plug); // Replace with the actual function or code you want to test

            // Assert that the result matches the expected value
            assert_eq!(result, *expected);
        }
    }

    #[rstest]
    #[should_panic]
    // Test that eVar substitution is capture avoiding
    #[case(exists(0, evar(1)), 1, evar(0))]
    fn test_apply_esubst_negative(
        #[case] pattern: Rc<Pattern>,
        #[case] evar_id: Id,
        #[case] plug: Rc<Pattern>,
    ) {
        _ = apply_esubst(&pattern, evar_id, &plug);
    }

    #[test]
    fn test_apply_ssubst() {
        let test_cases: Vec<(Rc<Pattern>, u8, Rc<Pattern>, Rc<Pattern>)> = vec![
            // Atomic cases
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
            // Subst when evar_id is fresh should do nothing
            //(metavar_unconstrained(0).with_s_fresh((svar(0), svar(1))), 0, symbol(1), metavar_unconstrained(0).with_s_fresh((svar(0), svar(1)))),
            // Subst on substs should stack
            (
                esubst(metavar_unconstrained(0), 0, symbol(1)),
                0,
                symbol(1),
                ssubst(esubst(metavar_unconstrained(0), 0, symbol(1)), 0, symbol(1)),
            ),
            (
                ssubst(metavar_unconstrained(0), 0, symbol(1)),
                0,
                symbol(1),
                ssubst(ssubst(metavar_unconstrained(0), 0, symbol(1)), 0, symbol(1)),
            ),
        ];

        for (pattern, svar_id, plug, expected) in test_cases {
            assert_eq!(apply_ssubst(&pattern, svar_id, &plug), expected);
        }
    }

    #[rstest]
    #[should_panic]
    // Test that sVar substitution is capture avoiding
    #[case(mu(0, svar(1)), 1, svar(0))]
    fn test_apply_ssubst_negative(
        #[case] pattern: Rc<Pattern>,
        #[case] svar_id: Id,
        #[case] plug: Rc<Pattern>,
    ) {
        _ = apply_ssubst(&pattern, svar_id, &plug);
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
