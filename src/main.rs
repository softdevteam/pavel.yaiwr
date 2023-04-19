use log::debug;
use std::{
    cell::RefCell,
    env, fs,
    io::{self, stdout, BufRead, Write},
    rc::Rc,
};
use yaiwr::{err::InterpError, instruction::EvalResult, scope::Scope, Calc};

fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    debug!("cli args {:?}", &args[1..]);
    let scope = Rc::new(RefCell::new(Scope::new()));
    let calc = &mut Calc::new();
    if args.len() > 1 {
        let result;
        if args[1].ends_with(".yaiwr") {
            result = run_from_file(&args[1], calc, scope.clone())
        } else {
            result = eval_statement(&args[1], calc, scope.clone());
        }
        if let Err(e) = result {
            print_err(e);
        }
    } else {
        repl(calc, scope.clone());
    }
}

fn print_err(err: InterpError) {
    eprintln!("Evaluation error: {}", err)
}

pub fn run_from_file<'a>(
    file_name: &str,
    calc: &mut Calc,
    scope: Rc<RefCell<Scope>>,
) -> Result<Option<EvalResult>, InterpError> {
    let file_path = file_name;
    match fs::read_to_string(file_name) {
        Ok(content) => eval_statement(content.as_str(), calc, scope),
        Err(_) => Err(InterpError::ProgramFileNotFound(file_path.to_string())),
    }
}

fn repl(calc: &mut Calc, scope: Rc<RefCell<Scope>>) {
    let stdin = io::stdin();
    loop {
        print!("👉 ");
        stdout().flush().ok();
        match stdin.lock().lines().next() {
            Some(Ok(ref l)) => {
                if l.trim().is_empty() {
                    continue;
                }
                match eval_statement(l, calc, scope.clone()) {
                    Ok(Some(EvalResult::Value(value))) => {
                        println!("{}", value);
                    }
                    Err(err) => print_err(err),
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

fn eval_statement(
    input: &str,
    calc: &mut Calc,
    scope: Rc<RefCell<Scope>>,
) -> Result<Option<EvalResult>, InterpError> {
    debug!("Statement: {:#?}", &input);
    let ast = calc.from_str(input);
    match ast {
        Ok(ast_node) => {
            debug!("AST: {:#?}", &ast_node);
            let bytecode = Calc::ast_to_bytecode(ast_node);

            debug!("Bytecode: {:#?}", &bytecode);
            match calc.eval(&bytecode, scope) {
                Ok(eval_result) => return Ok(eval_result),
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
