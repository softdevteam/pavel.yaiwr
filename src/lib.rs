use lrlex::{lrlex_mod, DefaultLexerTypes, LRNonStreamingLexerDef};
use lrpar::lrpar_mod;

lrlex_mod!("calc.l");
lrpar_mod!("calc.y");

pub fn eval(
    lexerdef: &LRNonStreamingLexerDef<DefaultLexerTypes>,
    input: &str,
) -> Result<String, String> {
    let lexer = lexerdef.lexer(input);
    let (res, errs) = calc_y::parse(&lexer);
    for e in errs {
        println!("{}", e.pp(&lexer, &calc_y::token_epp));
    }
    match res {
        Some(Ok(r)) => Ok(format!("Result: {:?}", r)),
        // TODO: prvide more infomraiton about the exact error.
        _ => Err(String::from("Unable to evaluate expression.")),
    }
}
