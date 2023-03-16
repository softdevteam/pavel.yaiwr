use log::debug;
use std::{
    env, fs,
    io::{self, stdout, BufRead, Write},
};
use yaiwr::{err::InterpError, Calc};

fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    debug!("cli args {:?}", &args[1..]);
    let calc = &mut Calc::new();
    if args.len() > 1 {
        let result;
        if args[1].ends_with(".yaiwr") {
            result = run_from_file(&args[1], calc)
        } else {
            result = eval_statement(&args[1], calc);
        }
        if let Err(e) = result {
            print_err(e);
        }
    } else {
        repl(calc);
    }
}

fn print_err(err: InterpError) {
    eprintln!("Evaluation error: {}", err)
}

pub fn run_from_file(file_name: &str, calc: &mut Calc) -> Result<Option<u64>, InterpError> {
    let file_path = file_name;
    match fs::read_to_string(file_name) {
        Ok(content) => eval_statement(content.as_str(), calc),
        Err(_) => Err(InterpError::ProgramFileNotFound(file_path.to_string())),
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
                match eval_statement(l, calc) {
                    Ok(Some(value)) => {
                        println!("{}", value);
                    }
                    Err(err) => print_err(err),
                    Ok(None) => {}
                }
            }
            _ => {}
        }
    }
}

fn eval_statement(input: &str, calc: &mut Calc) -> Result<Option<u64>, InterpError> {
    let statements: Vec<String> = input
        .replace("\n", "")
        .split(";")
        .map(|x| format!("{};", x))
        .collect();

    let mut result = None;
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
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
    return Ok(result);
}
