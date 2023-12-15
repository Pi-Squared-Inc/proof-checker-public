type Id = u8;
type IdList = Array<Id>;

#[derive(PartialEq, Drop, Clone)]
struct ImpliesType {
    left: Option<Box<Pattern>>,
    right: Option<Box<Pattern>>,
}

#[derive(PartialEq, Drop, Clone)]
struct AppType {
    left: Option<Box<Pattern>>,
    right: Option<Box<Pattern>>,
}

#[derive(PartialEq, Drop, Clone)]
struct ExistsType {
    var: Id,
    subpattern: Option<Box<Pattern>>,
}

#[derive(PartialEq, Drop, Clone)]
struct MuType {
    var: Id,
    subpattern: Option<Box<Pattern>>,
}

#[derive(PartialEq, Drop, Clone)]
struct MetaVarType {
    id: Id,
    e_fresh: IdList,
    s_fresh: IdList,
    positive: IdList,
    negative: IdList,
    app_ctx_holes: IdList,
}

#[derive(PartialEq, Drop, Clone)]
struct ESubstType {
    pattern: Option<Box<Pattern>>,
    evar_id: Id,
    plug: Option<Box<Pattern>>,
}

#[derive(PartialEq, Drop, Clone)]
struct SSubstType {
    pattern: Option<Box<Pattern>>,
    svar_id: Id,
    plug: Option<Box<Pattern>>,
}

#[derive(PartialEq, Drop, Clone)]
enum Pattern {
    EVar: Id,
    SVar: Id,
    Symbol: Id,
    Implies: ImpliesType, // left, right
    App: AppType, // left, right
    Exists: ExistsType, // var, subpattern
    Mu: MuType, // var, subpattern
    MetaVar: MetaVarType, // id, e_fresh, s_fresh, positive, negative, app_ctx_holes
    ESubst: ESubstType, // pattern, evar_id, plug
    SSubst: SSubstType // pattern, svar_id, plug
}

/// Pattern construction utilities
/// ------------------------------
fn evar(id: Id) -> Pattern {
    return Pattern::EVar(id);
}

fn svar(id: Id) -> Pattern {
    return Pattern::SVar(id);
}

fn symbol(id: Id) -> Pattern {
    return Pattern::Symbol(id);
}

fn implies(left: Pattern, right: Pattern) -> Pattern {
    let left = Option::Some(BoxTrait::new(left));
    let right = Option::Some(BoxTrait::new(right));
    return Pattern::Implies(ImpliesType { left: left, right: right });
}

fn app(left: Pattern, right: Pattern) -> Pattern {
    let left = Option::Some(BoxTrait::new(left));
    let right = Option::Some(BoxTrait::new(right));
    return Pattern::App(AppType { left: left, right: right });
}

fn exists(var: Id, subpattern: Pattern) -> Pattern {
    let subpattern = Option::Some(BoxTrait::new(subpattern));
    return Pattern::Exists(ExistsType { var: var, subpattern: subpattern });
}

fn mu(var: Id, subpattern: Pattern) -> Pattern {
    let subpattern = Option::Some(BoxTrait::new(subpattern));
    return Pattern::Mu(MuType { var: var, subpattern: subpattern });
}

fn metavar(
    id: Id,
    e_fresh: IdList,
    s_fresh: IdList,
    positive: IdList,
    negative: IdList,
    app_ctx_holes: IdList
) -> Pattern {
    return Pattern::MetaVar(
        MetaVarType {
            id: id,
            e_fresh: e_fresh,
            s_fresh: s_fresh,
            positive: positive,
            negative: negative,
            app_ctx_holes: app_ctx_holes
        }
    );
}

fn metavar_unconstrained(id: Id) -> Pattern {
    let e_fresh: IdList = array![];
    let s_fresh: IdList = array![];
    let positive: IdList = array![];
    let negative: IdList = array![];
    let app_ctx_holes: IdList = array![];
    return Pattern::MetaVar(
        MetaVarType {
            id: id,
            e_fresh: e_fresh,
            s_fresh: s_fresh,
            positive: positive,
            negative: negative,
            app_ctx_holes: app_ctx_holes
        }
    );
}

fn metavar_s_fresh(var_id: Id, fresh: Id, positive: IdList, negative: IdList) -> Pattern {
    let e_fresh: IdList = array![];
    let s_fresh: IdList = array![fresh];
    let app_ctx_holes: IdList = array![];
    return Pattern::MetaVar(
        MetaVarType {
            id: var_id,
            e_fresh: e_fresh,
            s_fresh: s_fresh,
            positive: positive,
            negative: negative,
            app_ctx_holes: app_ctx_holes
        }
    );
}

fn metavar_e_fresh(var_id: Id, fresh: Id, positive: IdList, negative: IdList) -> Pattern {
    let e_fresh: IdList = array![fresh];
    let s_fresh: IdList = array![];
    let app_ctx_holes: IdList = array![];
    return Pattern::MetaVar(
        MetaVarType {
            id: var_id,
            e_fresh: e_fresh,
            s_fresh: s_fresh,
            positive: positive,
            negative: negative,
            app_ctx_holes: app_ctx_holes
        }
    );
}

fn esubst(pattern: Pattern, evar_id: Id, plug: Pattern) -> Pattern {
    let pattern = Option::Some(BoxTrait::new(pattern));
    let plug = Option::Some(BoxTrait::new(plug));
    return Pattern::ESubst(ESubstType { pattern: pattern, evar_id: evar_id, plug: plug });
}

fn ssubst(pattern: Pattern, svar_id: Id, plug: Pattern) -> Pattern {
    let pattern = Option::Some(BoxTrait::new(pattern));
    let plug = Option::Some(BoxTrait::new(plug));
    return Pattern::SSubst(SSubstType { pattern: pattern, svar_id: svar_id, plug: plug });
}

impl PatternOptionBoxPartialEq of PartialEq<Option<Box<Pattern>>> {
    fn eq(lhs: @Option<Box<Pattern>>, rhs: @Option<Box<Pattern>>) -> core::bool {
        match lhs {
            Option::Some(lhs) => {
                match (rhs) {
                    Option::Some(rhs) => { lhs.as_snapshot().unbox() == rhs.as_snapshot().unbox() },
                    Option::None => { false }
                }
            },
            Option::None => {
                match (rhs) {
                    Option::Some(_) => { false },
                    Option::None => { true }
                }
            }
        }
    }


    fn ne(lhs: @Option<Box<Pattern>>, rhs: @Option<Box<Pattern>>) -> core::bool {
        !(lhs == rhs)
    }
}

impl PatternOptionBoxClone of Clone<Option<Box<Pattern>>> {
    fn clone(self: @Option<Box<Pattern>>) -> Option<Box<Pattern>> {
        match self {
            Option::Some(box_pat) => {
                let mut result: Pattern = box_pat.as_snapshot().unbox().clone();
                return Option::Some(BoxTrait::new(result));
            },
            Option::None => { Option::None }
        }
    }
}
