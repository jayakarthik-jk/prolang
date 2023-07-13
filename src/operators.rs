#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operator {
    ArithmeticOperator(Arithmetic),
    RelationalOperator(Relational),
    AssingmentOperator(Assingment),
    LogicalOperator(Logical),
}

impl Operator {
    pub fn evaluate(&self, a: i32, b: i32) -> i32 {
        match self {
            Operator::ArithmeticOperator(arithmetic) => match arithmetic {
                Arithmetic::Addition => a + b,
                Arithmetic::Subtraction => a - b,
                Arithmetic::Multiplication => a * b,
                Arithmetic::Division => a / b,
                Arithmetic::Modulo => a % b,
            },
            _ => {
                panic!("not implemented")
            }
        }
    }
    pub fn get_precedence(&self) -> u8 {
        match self {
            Operator::ArithmeticOperator(operator) => match operator {
                Arithmetic::Addition => 4,
                Arithmetic::Subtraction => 4,
                Arithmetic::Multiplication => 5,
                Arithmetic::Division => 5,
                Arithmetic::Modulo => 5,
            },
            Operator::RelationalOperator(_) => 3,
            Operator::AssingmentOperator(_) => 2,
            Operator::LogicalOperator(_) => 1,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Arithmetic {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulo,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Relational {
    Equality,
    InEquality,
    LessThan,
    LessThanOrEqualTo,
    GreaterThan,
    GreaterThanOrEqualTo,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Assingment {
    EqualTo,
    AdditionAssignment,
    SubtractionAssignment,
    MultiplicationAssignment,
    DivisionAssignment,
    ModuloAssignment,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Logical {
    And,
    Or,
    Not,
    Xor,
}
