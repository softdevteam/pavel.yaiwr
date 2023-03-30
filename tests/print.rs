#[cfg(test)]
mod tests {
    use yaiwr::{instruction::Instruction, scope::Scope, Calc};

    #[test]
    fn eval_println_statement_add() {
        let calc = &mut Calc::new();
        let ast = calc.from_str("println(2+2);").unwrap();
        let bytecode = Calc::ast_to_bytecode(ast);
        assert_eq!(calc.eval(&bytecode, &mut Scope::new()).unwrap(), None);
    }

    #[test]
    fn eval_println_statement_mul() {
        let calc = &mut Calc::new();
        let ast = calc.from_str("println(2*2);").unwrap();
        let bytecode = Calc::ast_to_bytecode(ast);
        assert_eq!(calc.eval(&bytecode, &mut Scope::new()).unwrap(), None);
    }

    #[test]
    fn println_statement_numeric_bytecode() {
        let calc = &mut Calc::new();
        let ast = calc.from_str("println(1);").unwrap();
        let bytecode = Calc::ast_to_bytecode(ast);
        match bytecode.as_slice() {
            [first, second] => {
                assert_eq!(first, &Instruction::Push { value: 1 });
                assert_eq!(second, &Instruction::PrintLn);
            }
            _ => panic!("expected bytecodes to be not empty!"),
        }
    }

    #[test]
    fn print_statement_add_bytecode() {
        let calc = &mut Calc::new();
        let ast = calc.from_str("println (1+1);").unwrap();
        let bytecode = Calc::ast_to_bytecode(ast);
        match bytecode.as_slice() {
            [c1, c2, c3, c4] => {
                assert_eq!(c1, &Instruction::Push { value: 1 });
                assert_eq!(c2, &Instruction::Push { value: 1 });
                assert_eq!(c3, &Instruction::Add {});
                assert_eq!(c4, &Instruction::PrintLn {});
            }
            _ => panic!("expected bytecodes to be not empty!"),
        }
    }

    #[test]
    #[should_panic]
    fn eval_println_statement_add_parsing_error() {
        let calc = &mut Calc::new();
        let ast = calc.from_str("println 2+2").unwrap();
        let bytecode = Calc::ast_to_bytecode(ast);
        assert_eq!(calc.eval(&bytecode, &mut Scope::new()).unwrap(), None);
    }
    #[test]
    #[should_panic]
    fn eval_println_statement_mul_parsing_error() {
        let calc = &mut Calc::new();
        let ast = calc.from_str("println 2*2").unwrap();
        let bytecode = Calc::ast_to_bytecode(ast);
        assert_eq!(calc.eval(&bytecode, &mut Scope::new()).unwrap(), None);
    }

    #[test]
    fn println_cmd() {
        use std::process::Command;
        let output = Command::new("cargo")
            .arg("run")
            .arg("println(2);")
            .output()
            .expect("command 'cargo run println(2);' failed");

        assert_eq!(String::from_utf8_lossy(&output.stdout), "2\n");
    }
}
