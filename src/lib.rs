use instruction::Instruction;
use lrlex::{lrlex_mod, DefaultLexerTypes};
use lrpar::{lrpar_mod, LexParseError, NonStreamingLexer};

lrlex_mod!("calc.l");
lrpar_mod!("calc.y");

pub mod ast;
pub mod instruction;

use ast::AstNode;

pub struct Calc {}

impl Calc {
    pub fn from_str(input: &str) -> Result<AstNode, String> {
        let lexer_def = calc_l::lexerdef();
        let lexer = lexer_def.lexer(input);
        let (ast_exp, errs) = calc_y::parse(&lexer);

        let err_msg = Self::get_parse_err(&lexer, errs);
        if err_msg.is_empty() == false {
            return Err(err_msg);
        }

        match ast_exp {
            Some(res) => match res {
                Ok(exp) => Ok(exp),
                Err(_) => Err(err_msg),
            },
            None => Err(err_msg),
        }
    }

    fn get_parse_err(
        lexer: &dyn NonStreamingLexer<DefaultLexerTypes>,
        errs: Vec<LexParseError<u32, DefaultLexerTypes>>,
    ) -> String {
        let msgs = errs
            .iter()
            .map(|e| e.pp(lexer, &calc_y::token_epp))
            .collect::<Vec<String>>();
        return msgs.join("\n");
    }

    pub fn to_bytecode(ast_node: AstNode, prog: &mut Vec<Instruction>) {
        match ast_node {
            AstNode::Add { lhs, rhs } => {
                Self::to_bytecode(*lhs, prog);
                Self::to_bytecode(*rhs, prog);
                prog.push(Instruction::Add {})
            }
            AstNode::Mul { lhs, rhs } => {
                Self::to_bytecode(*lhs, prog);
                Self::to_bytecode(*rhs, prog);
                prog.push(Instruction::Mul {})
            }
            AstNode::Number { value } => prog.push(Instruction::Push { value: value }),
            AstNode::PrintLn { rhs } => {
                Self::to_bytecode(*rhs, prog);
                prog.push(Instruction::PrintLn {})
            }
        }
    }

    pub fn eval(instructions: &Vec<Instruction>) -> Result<Option<u64>, String> {
        let mut stack = vec![];
        for a in instructions {
            match a {
                Instruction::Push { value } => stack.push(*value),
                Instruction::PrintLn {} => {
                    println!("{}", stack.pop().expect("cannot pop from empty stack"))
                }
                Instruction::Mul {} => {
                    let result = stack
                        .pop()
                        .expect("cannot pop from empty stack")
                        .checked_mul(stack.pop().expect("cannot pop from empty stack"))
                        .ok_or("overflowed".to_string())?;
                    stack.push(result)
                }
                Instruction::Add {} => {
                    let result = stack
                        .pop()
                        .expect("cannot pop from empty stack")
                        .checked_add(stack.pop().expect("cannot pop from empty stack"))
                        .ok_or("overflowed".to_string())?;
                    stack.push(result)
                }
            }
        }
        if stack.is_empty() {
            return Ok(None);
        }
        return Ok(Some(stack.pop().unwrap()));
    }
}
