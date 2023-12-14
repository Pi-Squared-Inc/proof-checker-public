use ml_checker_cairo::pattern::Pattern;

/// Terms
/// =====
///
/// Terms define the in-memory representation of matching logic patterns and proofs.
/// However, since we only implement a proof checker in this program we do not need
/// an explicit representation of the entire hilbert proof tree.
/// We only need to store the conclusion of things that are proved so far.
/// We use the `Proved` variant for this.

#[derive(Drop)]
enum Term {
    Pattern: Pattern,
    Proved: Pattern,
}

#[derive(Drop)]
enum Entry {
    Pattern: Pattern,
    Proved: Pattern,
}
