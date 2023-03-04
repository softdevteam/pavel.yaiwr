use std::{
    env,
    io::{self, stdout, BufRead, Write},
};
use yaiwr::Calc;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        eval_print(&args[1]);
    } else {
        let stdin = io::stdin();
        loop {
            print!("👉 ");
            stdout().flush().ok();
            match stdin.lock().lines().next() {
                Some(Ok(ref l)) => {
                    if l.trim().is_empty() {
                        continue;
                    }
                    eval_print(l);
                }
                _ => break,
            }
        }
    }
}

fn eval_print(input: &str) {
    let ast = Calc::from_str(input);
    match ast {
        Ok(opcode) => match Calc::eval(opcode) {
            Ok(i) => println!("Result: {}", i),
            Err(msg) => eprintln!("Evaluation error: {}", msg),
        },
        Err(msg) => eprintln!("Evaluation error: {}", msg),
    }
}
