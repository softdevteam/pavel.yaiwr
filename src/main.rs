use std::io::{self, stdout, BufRead, Write};
use yaiwr::Calc;

fn main() {
    let stdin = io::stdin();
    loop {
        print!("👉 ");
        stdout().flush().ok();
        match stdin.lock().lines().next() {
            Some(Ok(ref l)) => {
                if l.trim().is_empty() {
                    continue;
                }
                let ast = Calc::from_str(l);
                match ast {
                    Ok(opcode) => match Calc::eval(opcode) {
                        Ok(i) => println!("Result: {}", i),
                        Err(msg) => eprintln!("Evaluation error: {}", msg),
                    },
                    Err(msg) => eprintln!("Evaluation error: {}", msg),
                }
            }
            _ => break,
        }
    }
}
