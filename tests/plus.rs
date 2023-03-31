#[cfg(test)]
mod tests {
    use yaiwr::{instruction::Instruction, scope::Scope, Calc};

    #[test]
    fn eval_add_expression() {
        let calc = &mut Calc::new();
        let ast = calc.from_str("2+2").unwrap();
        let bytecode = Calc::ast_to_bytecode(ast);
        assert_eq!(calc.eval(&bytecode, &mut Scope::new()).unwrap(), Some(4));
    }

    #[test]
    fn eval_add_expressions() {
        let calc = &mut Calc::new();
        let ast = calc.from_str("2+2+2").unwrap();
        let bytecode = Calc::ast_to_bytecode(ast);
        assert_eq!(calc.eval(&bytecode, &mut Scope::new()).unwrap(), Some(6));
    }

    #[test]
    fn add_bytecode() {
        let calc = &mut Calc::new();
        let ast = calc.from_str("1+2").unwrap();
        let bytecode = Calc::ast_to_bytecode(ast);
        match bytecode.as_slice() {
            [first, second, third] => {
                assert_eq!(first, &Instruction::Push { value: 1 });
                assert_eq!(second, &Instruction::Push { value: 2 });
                assert_eq!(third, &Instruction::Add);
            }
            _ => panic!("expected bytecodes to be not empty!"),
        }
    }

    #[test]
    #[should_panic(expected = "overflowed")]
    fn add_overflow_max_u64() {
        let calc = &mut Calc::new();
        let input = format!("{}+{}", u64::MAX, 1);
        let ast = calc.from_str(input.as_str()).unwrap();
        let bytecode = Calc::ast_to_bytecode(ast);
        calc.eval(&bytecode, &mut Scope::new()).unwrap();
    }

    #[test]
    fn add_no_overflow() {
        let calc = &mut Calc::new();
        let input = format!("{}+{}", u64::MAX - 1, 1);
        let ast = calc.from_str(input.as_str()).unwrap();
        let bytecode = Calc::ast_to_bytecode(ast);
        calc.eval(&bytecode, &mut Scope::new()).unwrap();
    }
}
