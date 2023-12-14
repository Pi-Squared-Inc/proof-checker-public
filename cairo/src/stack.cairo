use ml_checker_cairo::term::Term;
use ml_checker_cairo::verifier::bot;

#[derive(Drop)]
struct Stack {
    elements: Array<Term>,
    len: u32,
}

trait StackTrait {
    fn new() -> Stack;
    fn push(ref self: Stack, term: Term);
    fn pop(ref self: Stack) -> Term;
    fn is_empty(ref self: Stack) -> bool;
    fn clear(ref self: Stack);
    fn len(ref self: Stack) -> u32;
}

impl StackTraitImpl of StackTrait {
    fn new() -> Stack {
        return Stack { elements: array![], len: 0, };
    }

    fn push(ref self: Stack, term: Term) {
        self.elements.append(term);
        self.len += 1;
    }
    fn pop(ref self: Stack) -> Term {
        if self.is_empty() {
            panic!("Insufficient stack items.");
        }

        let mut new_stack = array![];
        let mut pop_term: Term = Term::Pattern(bot());
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

    fn is_empty(ref self: Stack) -> bool {
        return self.len == 0;
    }

    fn clear(ref self: Stack) {
        self.elements = array![];
        self.len = 0;
    }

    fn len(ref self: Stack) -> u32 {
        return self.len;
    }
}
