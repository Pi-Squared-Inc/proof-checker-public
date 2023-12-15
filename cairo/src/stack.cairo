use core::option::OptionTrait;
use ml_checker_cairo::term::Term;
use ml_checker_cairo::term::Pattern;
use ml_checker_cairo::verifier::bot;

#[derive(Drop)]
struct StackStructure<T> {
    elements: Array<T>,
    len: u32,
}

trait StackTrait {
    fn new() -> StackStructure<Term>;
    fn push(ref self: StackStructure<Term>, term: Term);
    fn pop(ref self: StackStructure<Term>) -> Term;
    fn is_empty(ref self: StackStructure<Term>) -> bool;
    fn clear(ref self: StackStructure<Term>);
    fn len(ref self: StackStructure<Term>) -> u32;
    fn last(ref self: StackStructure<Term>) -> Option<@Term>;
}

impl StackTraitImpl of StackTrait {
    fn new() -> StackStructure<Term> {
        return StackStructure { elements: array![], len: 0, };
    }

    fn push(ref self: StackStructure<Term>, term: Term) {
        self.elements.append(term);
        self.len += 1;
    }
    fn pop(ref self: StackStructure<Term>) -> Term {
        if self.is_empty() {
            panic!("Insufficient stack items.");
        }

        let mut new_stack = array![];
        let mut pop_term: Term = Term::Pattern(bot());
        let mut i = 0;

        loop {
            let term = self.elements.pop_front().expect('Insufficient stack items.');
            if i == self.len - 1 {
                pop_term = term;
                break;
            }
            new_stack.append(term);
            i += 1;
        };

        self.elements = new_stack;
        self.len -= 1;

        return pop_term;
    }

    fn is_empty(ref self: StackStructure<Term>) -> bool {
        return self.len == 0;
    }

    fn clear(ref self: StackStructure<Term>) {
        self.elements = array![];
        self.len = 0;
    }

    fn len(ref self: StackStructure<Term>) -> u32 {
        return self.len;
    }

    fn last(ref self: StackStructure<Term>) -> Option<@Term> {
        if self.is_empty() {
            return Option::None;
        }

        let mut span_array = self.elements.span();
        let mut term: @Term = span_array.get(self.len - 1).unwrap().unbox();

        return Option::Some(term);
    }
}

trait ClaimTrait {
    fn new() -> StackStructure<Pattern>;
    fn push(ref self: StackStructure<Pattern>, term: Pattern);
    fn pop(ref self: StackStructure<Pattern>) -> Pattern;
    fn is_empty(ref self: StackStructure<Pattern>) -> bool;
    fn clear(ref self: StackStructure<Pattern>);
    fn len(ref self: StackStructure<Pattern>) -> u32;
}

impl ClaimTraitImpl of ClaimTrait {
    fn new() -> StackStructure<Pattern> {
        return StackStructure { elements: array![], len: 0, };
    }

    fn push(ref self: StackStructure<Pattern>, term: Pattern) {
        self.elements.append(term);
        self.len += 1;
    }
    fn pop(ref self: StackStructure<Pattern>) -> Pattern {
        if self.is_empty() {
            panic!("Insufficient stack items.");
        }

        let mut new_stack = array![];
        let mut pop_term: Pattern = bot();
        let mut i = 0;

        loop {
            let term = self.elements.pop_front();
            match term {
                Option::Some(term) => {
                    if i == self.len - 1 {
                        pop_term = term;
                        break;
                    }
                    new_stack.append(term);
                    i += 1;
                },
                Option::None => { break; }
            }
        };

        self.elements = new_stack;
        self.len -= 1;

        return pop_term;
    }

    fn is_empty(ref self: StackStructure<Pattern>) -> bool {
        return self.len == 0;
    }

    fn clear(ref self: StackStructure<Pattern>) {
        self.elements = array![];
        self.len = 0;
    }

    fn len(ref self: StackStructure<Pattern>) -> u32 {
        return self.len;
    }
}
