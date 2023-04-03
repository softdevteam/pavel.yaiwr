use bytecode::block_to_bytecode;
use instruction::Instruction;
use log::debug;
use lrlex::{lrlex_mod, DefaultLexerTypes};
use lrpar::{lrpar_mod, LexParseError, NonStreamingLexer};
use scope::Scope;
use std::collections::HashMap;

lrlex_mod!("calc.l");
lrpar_mod!("calc.y");

pub mod ast;
pub mod bytecode;
pub mod err;
pub mod instruction;
pub mod scope;

use ast::AstNode;
use err::InterpError;

pub struct Calc {
    fun_store: HashMap<String, Instruction>,
    stack: Vec<u64>,
}

impl Calc {
    pub fn new() -> Self {
        Calc {
            fun_store: HashMap::new(),
            stack: vec![],
        }
    }

    pub fn stack_pop(&mut self) -> Result<u64, InterpError> {
        return Ok(self.stack.pop().ok_or(InterpError::EmptyStack)?);
    }

    pub fn stack_push(&mut self, val: u64) {
        self.stack.push(val);
    }

    pub fn from_str(&self, input: &str) -> Result<Vec<AstNode>, InterpError> {
        let lexer_def = calc_l::lexerdef();
        let lexer = lexer_def.lexer(input);
        let (ast_exp, errs) = calc_y::parse(&lexer);

        let err_msg = self.get_parse_err(&lexer, errs);
        if err_msg.is_empty() == false {
            return Err(InterpError::ParseError(err_msg));
        }

        match ast_exp {
            Some(res) => match res {
                Ok(exp) => Ok(exp),
                Err(_) => Err(InterpError::ParseError(err_msg)),
            },
            None => Err(InterpError::ParseError(err_msg)),
        }
    }

    fn get_parse_err(
        &self,
        lexer: &dyn NonStreamingLexer<DefaultLexerTypes>,
        errs: Vec<LexParseError<u32, DefaultLexerTypes>>,
    ) -> String {
        let msgs = errs
            .iter()
            .map(|e| e.pp(lexer, &calc_y::token_epp))
            .collect::<Vec<String>>();
        return msgs.join("\n");
    }

    pub fn ast_to_bytecode(ast: Vec<AstNode>) -> Vec<Instruction> {
        return block_to_bytecode(ast);
    }

    fn eval_function_args(
        &mut self,
        args: &Vec<Vec<Instruction>>,
        scope: &mut Scope,
    ) -> Result<Vec<u64>, InterpError> {
        let mut result = vec![];
        for arg_set in args {
            match self.eval(arg_set, scope) {
                Ok(Some(x)) => result.push(x),
                Ok(None) => {}
                Err(e) => return Err(e),
            }
        }
        return Ok(result);
    }

    fn eval_function_call(
        &mut self,
        args: &Vec<u64>,
        id: &String,
        outer_scope: &mut Scope,
    ) -> Result<Option<u64>, InterpError> {
        let function = self
            .fun_store
            .get(id)
            .ok_or(InterpError::UndefinedFunction(id.to_string()))?;
        match function {
            Instruction::Function {
                id: _,
                params,
                block: body,
            } => {
                if params.len() != args.len() {
                    return Err(InterpError::EvalError(format!(
                        "Unexpected number of function arguments. Expected: {}, Got: {}",
                        params.len(),
                        args.len()
                    )));
                }
                let func_scope = &mut Scope::from_scope(outer_scope);
                func_scope.assign(HashMap::from_iter(params.iter().zip(args)));
                return self.eval(&body.clone(), func_scope);
            }
            _ => {
                return Err(InterpError::EvalError(
                    "Unexpected type registrated as a function!".to_string(),
                ));
            }
        }
    }

    pub fn eval(
        &mut self,
        instructions: &Vec<Instruction>,
        scope: &mut Scope,
    ) -> Result<Option<u64>, InterpError> {
        for instruction in instructions {
            debug!("eval: {:?}. scope: {:?}", instruction, scope);
            match instruction {
                Instruction::Return { block } => {
                    let val = self.eval(block, scope)?;
                    if let Some(x) = val {
                        self.stack_push(x);
                    }
                }
                Instruction::Function {
                    block: body,
                    id,
                    params,
                } => {
                    if let None = self.fun_store.get(id) {
                        self.fun_store.insert(
                            id.to_string(),
                            Instruction::Function {
                                id: id.to_string(),
                                params: params.to_vec(),
                                block: body.to_vec(),
                            },
                        );
                    } else {
                        return Err(InterpError::EvalError(format!(
                            "Function with the id: '{}' already defined!",
                            id
                        )));
                    }
                }
                Instruction::FunctionCall { id, args } => {
                    let arg_list = self.eval_function_args(&args, scope)?;
                    let res = self.eval_function_call(&arg_list, id, scope)?;
                    if let Some(x) = res {
                        self.stack_push(x);
                    }
                }
                Instruction::Push { value } => self.stack.push(*value),
                Instruction::PrintLn => {
                    println!("{}", self.stack.pop().unwrap());
                }
                Instruction::Mul {} => {
                    let val = self
                        .stack_pop()?
                        .checked_mul(self.stack_pop()?)
                        .ok_or(InterpError::Numeric("overflowed".to_string()))?;
                    self.stack_push(val);
                }
                Instruction::Add {} => {
                    let val = self
                        .stack_pop()?
                        .checked_add(self.stack_pop()?)
                        .ok_or(InterpError::Numeric("overflowed".to_string()))?;
                    self.stack_push(val);
                }
                Instruction::Assign { id } => {
                    let val = self.stack_pop()?;
                    scope.var_store.insert(id.to_string(), val);
                }
                Instruction::Load { id } => {
                    let val = scope.get_var(id)?;
                    self.stack_push(*val);
                }
            }
        }
        if self.stack.is_empty() {
            return Ok(None);
        }
        return Ok(Some(self.stack.pop().unwrap()));
    }
}
