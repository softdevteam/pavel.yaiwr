#[derive(Debug, Clone)]
pub enum AstNode {
    Add {
        lhs: Box<AstNode>,
        rhs: Box<AstNode>,
    },
    Mul {
        lhs: Box<AstNode>,
        rhs: Box<AstNode>,
    },
    Number {
        value: u64,
    },
}
