use log::debug;
use std::{
    env, fs,
    io::{self, stdout, BufRead, Write},
};
use yaiwr::Calc;

fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    debug!("cli args {:?}", &args[1..]);
    let calc = &mut Calc::new();
    if args.len() > 1 {
        if args[1].ends_with(".yaiwr") {
            let contents =
                fs::read_to_string(&args[1]).expect("Should have been able to read the file");
            eval_statement(contents.as_str(), calc);
        } else {
            eval_statement(&args[1], calc);
        }
    } else {
        repl(calc);
    }
}

fn repl(calc: &mut Calc) {
    let stdin = io::stdin();
    loop {
        print!("👉 ");
        stdout().flush().ok();
        match stdin.lock().lines().next() {
            Some(Ok(ref l)) => {
                if l.trim().is_empty() {
                    continue;
                }
                if let Some(value) = eval_statement(l, calc) {
                    println!("{}", value);
                }
            }
            _ => break,
        }
    }
}

fn eval_statement(input: &str, calc: &mut Calc) -> Option<u64> {
    let statements: Vec<String> = input
        .replace("\n", "")
        .split(";")
        .map(|x| format!("{};", x))
        .collect();

    let mut result: Option<u64> = None;
    for statement in statements {
        if statement == ";" {
            continue;
        }
        debug!("statement: {:?}", &statement);
        let ast = calc.from_str(statement.as_str());
        match ast {
            Ok(ast_node) => {
                debug!("AST: {:?}", &ast_node);
                let bytecode = &mut vec![];
                calc.to_bytecode(ast_node, bytecode);
                debug!("Bytecode: {:?}", &bytecode);
                match calc.eval(bytecode) {
                    Ok(eval_result) => {
                        result = eval_result;
                    }
                    Err(msg) => {
                        eprintln!("Evaluation error: {}", msg);
                        return None;
                    }
                }
            }
            Err(msg) => {
                eprintln!("Evaluation error: {}", msg);
                return None;
            }
        }
    }
    return result;
}
