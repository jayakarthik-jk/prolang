#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Relational {
    Equality,
    InEquality,
    LessThan,
    LessThanOrEqualTo,
    GreaterThan,
    GreaterThanOrEqualTo,
}
