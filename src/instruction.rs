use std::fmt::{Display, Error, Formatter};

use crate::err::InterpError;

#[derive(Debug, Clone, Copy)]
pub enum StackValue {
    Integer(u64),
    Boolean(bool),
}

impl StackValue {
    pub fn as_int(&self) -> Result<u64, InterpError> {
        match self {
            StackValue::Integer(v) => Ok(*v),
            a => Err(InterpError::EvalError(
                format!("Expected StackValue Integer stack, got {}!", a).to_string(),
            )),
        }
    }
    pub fn as_bool(&self) -> Result<bool, InterpError> {
        match self {
            StackValue::Boolean(v) => Ok(*v),
            a => Err(InterpError::EvalError(
                format!("Expected StackValue Boolean, got {}!", a).to_string(),
            )),
        }
    }
    pub fn is_same_type(&self, other: &Self) -> bool {
        match self {
            StackValue::Boolean(..) => {
                if let StackValue::Boolean(..) = other {
                    return true;
                }
                false
            }
            StackValue::Integer(..) => {
                if let StackValue::Integer(..) = other {
                    return true;
                }
                false
            }
        }
    }
}

impl PartialEq for StackValue {
    fn eq(&self, other: &Self) -> bool {
        match self {
            StackValue::Boolean(self_val) => match other {
                StackValue::Boolean(other_val) => return self_val == other_val,
                _ => false,
            },
            StackValue::Integer(self_val) => match other {
                StackValue::Integer(other_val) => return self_val == other_val,
                _ => false,
            },
        }
    }

    fn ne(&self, other: &Self) -> bool {
        return !self.eq(other);
    }
}

impl Display for StackValue {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let a = match self {
            StackValue::Integer(val) => f.write_str(format!("{}", val).as_str()),
            StackValue::Boolean(val) => f.write_str(format!("{}", val).as_str()),
        };
        return a;
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOp {
    LessThan,
    GreaterThan,
    Add,
    Mul,
    Equal,
    NotEqual,
    Assign { id: String },
}

impl Display for BinaryOp {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            BinaryOp::LessThan => f.write_str("LessThan"),
            BinaryOp::GreaterThan => f.write_str("GreaterThan"),
            BinaryOp::Add => f.write_str("Add"),
            BinaryOp::Mul => f.write_str("Mul"),
            BinaryOp::Assign { .. } => f.write_str("Assign"),
            BinaryOp::Equal => f.write_str("Equal"),
            BinaryOp::NotEqual => f.write_str("NotEqual"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
    BinaryOp {
        op: BinaryOp,
    },
    Push {
        value: StackValue,
    },
    PrintLn,
    Load {
        id: String,
    },
    Return {
        block: Vec<Instruction>,
    },
    Function {
        id: String,
        params: Vec<String>,
        block: Vec<Instruction>,
    },
    FunctionCall {
        id: String,
        args: Vec<Vec<Instruction>>,
    },
    Conditional {
        condition: Vec<Instruction>,
        block: Vec<Instruction>,
        alternative: Option<Vec<Instruction>>,
    },
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Instruction::Conditional { .. } => f.write_str("Conditional"),
            Instruction::Push { value: _ } => f.write_str("Push"),
            Instruction::PrintLn => f.write_str("PrintLn"),
            Instruction::Load { .. } => f.write_str("Load"),
            Instruction::Return { .. } => f.write_str("Return"),
            Instruction::Function {
                id: _,
                params: _,
                block: _,
            } => f.write_str("Function"),
            Instruction::FunctionCall { .. } => f.write_str("FunctionCall"),
            Instruction::BinaryOp { op } => f.write_str(format!("BinaryOp({})", op).as_str()),
        }
    }
}
