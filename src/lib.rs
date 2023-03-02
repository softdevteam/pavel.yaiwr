use lrlex::{lrlex_mod, DefaultLexerTypes};
use lrpar::{lrpar_mod, LexParseError, NonStreamingLexer};

lrlex_mod!("calc.l");
lrpar_mod!("calc.y");

mod ast;

use ast::Opcode;

pub struct Calc {}

impl Calc {
    pub fn from_str(input: &str) -> Result<Vec<Opcode>, String> {
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

    pub fn eval(opcodes: Vec<Opcode>) -> Result<u64, String> {
        for opcode in opcodes {
            return Self::eval_exp(opcode);
        }
        return Err(String::from("Couldn't evaluate given opcodes."));
    }

    fn eval_exp(exp: Opcode) -> Result<u64, String> {
        match exp {
            Opcode::Add { lhs, rhs } => Self::eval_exp(*lhs)?
                .checked_add(Self::eval_exp(*rhs)?)
                .ok_or("overflowed".to_string()),
            Opcode::Mul { lhs, rhs } => Self::eval_exp(*lhs)?
                .checked_mul(Self::eval_exp(*rhs)?)
                .ok_or("overflowed".to_string()),
            Opcode::Number { value } => Ok(value),
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
}
