use std::{
    cell::RefCell,
    fmt::{Display, Error, Formatter},
    mem::discriminant,
    rc::Rc,
};

use crate::{err::InterpError, scope::Scope};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StackValue {
    Integer(u64),
    Function(u64),
    Boolean(bool),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum JumpInstruction {
    Return,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EvalResult {
    Value(StackValue),
    Jump(JumpInstruction),
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
        discriminant(self) == discriminant(other)
    }
}

impl Display for StackValue {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let a = match self {
            StackValue::Integer(val) => f.write_str(format!("{}", val).as_str()),
            StackValue::Boolean(val) => f.write_str(format!("{}", val).as_str()),
            StackValue::Function(_) => f.write_str(format!("Function").as_str()),
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
    LogicalAnd,
    LogicalOr,
    Assign { name: String, id: u64 },
    Declare { name: String, id: u64 },
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
            BinaryOp::LogicalAnd => f.write_str("LogicalAnd"),
            BinaryOp::LogicalOr => f.write_str("LogicalOr"),
            BinaryOp::Declare { .. } => f.write_str("Declare"),
        }
    }
}

#[derive(Debug, Clone)]
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
        name: String,
        id: u64,
        params: Vec<String>,
        block: Vec<Instruction>,
        scope: Option<Rc<RefCell<Scope>>>,
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
impl PartialEq for Instruction {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::BinaryOp { op: l_op }, Self::BinaryOp { op: r_op }) => l_op == r_op,
            (Self::Push { value: l_value }, Self::Push { value: r_value }) => l_value == r_value,
            (Self::Load { id: l_id }, Self::Load { id: r_id }) => l_id == r_id,
            (Self::Return { block: l_block }, Self::Return { block: r_block }) => {
                l_block == r_block
            }
            (
                Self::Function {
                    id: l_id,
                    name: _,
                    params: l_params,
                    block: l_block,
                    scope: _,
                },
                Self::Function {
                    id: r_id,
                    name: _,
                    params: r_params,
                    block: r_block,
                    scope: _,
                },
            ) => l_id == r_id && l_params == r_params && l_block == r_block,
            (
                Self::FunctionCall {
                    id: l_id,
                    args: l_args,
                },
                Self::FunctionCall {
                    id: r_id,
                    args: r_args,
                },
            ) => l_id == r_id && l_args == r_args,
            (
                Self::Conditional {
                    condition: l_condition,
                    block: l_block,
                    alternative: l_alternative,
                },
                Self::Conditional {
                    condition: r_condition,
                    block: r_block,
                    alternative: r_alternative,
                },
            ) => l_condition == r_condition && l_block == r_block && l_alternative == r_alternative,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Instruction::Conditional { .. } => f.write_str("Conditional"),
            Instruction::Push { value: _ } => f.write_str("Push"),
            Instruction::PrintLn => f.write_str("PrintLn"),
            Instruction::Load { .. } => f.write_str("Load"),
            Instruction::Return { .. } => f.write_str("Return"),
            Instruction::Function { .. } => f.write_str("Function"),
            Instruction::FunctionCall { .. } => f.write_str("FunctionCall"),
            Instruction::BinaryOp { op } => f.write_str(format!("BinaryOp({})", op).as_str()),
        }
    }
}
