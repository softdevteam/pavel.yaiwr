#[cfg(test)]
mod tests {
    use yaiwr::{
        instruction::{BinaryOp, EvalResult, Instruction, StackValue},
        scope::Scope,
        Calc,
    };

    #[test]
    fn eval_mul_expression() {
        let scope = &mut Box::new(Scope::new());
        let calc = &mut Calc::new();
        let ast = calc.from_str("2*2;").unwrap();
        let bytecode = Calc::ast_to_bytecode(ast);
        assert_eq!(
            calc.eval(&bytecode, scope).unwrap(),
            Some(EvalResult::Value(StackValue::Integer(4)))
        );
    }

    #[test]
    fn eval_mul_expressions() {
        let scope = &mut Box::new(Scope::new());
        let calc = &mut Calc::new();
        let ast = calc.from_str("2*2*2;").unwrap();
        let bytecode = Calc::ast_to_bytecode(ast);
        assert_eq!(
            calc.eval(&bytecode, scope).unwrap(),
            Some(EvalResult::Value(StackValue::Integer(8)))
        );
    }

    #[test]
    fn mul_bc() {
        let calc = &mut Calc::new();
        let ast = calc.from_str("1*2;").unwrap();
        let bytecode = Calc::ast_to_bytecode(ast);
        match bytecode.as_slice() {
            [first, second, third] => {
                assert_eq!(
                    first,
                    &Instruction::Push {
                        value: StackValue::Integer(1)
                    }
                );
                assert_eq!(
                    second,
                    &Instruction::Push {
                        value: StackValue::Integer(2)
                    }
                );
                assert_eq!(third, &Instruction::BinaryOp { op: BinaryOp::Mul });
            }
            _ => panic!("expected bytecodes to be not empty!"),
        }
    }

    #[test]
    #[should_panic(expected = "overflowed")]
    fn mul_overflow() {
        let scope = &mut Box::new(Scope::new());
        let calc = &mut Calc::new();
        let input = format!("{}*{};", u64::MAX, 2);
        let ast = calc.from_str(input.as_str()).unwrap();
        let bytecode = Calc::ast_to_bytecode(ast);
        calc.eval(&bytecode, scope).unwrap();
    }

    #[test]
    fn mul_no_overflow() {
        let scope = &mut Box::new(Scope::new());
        let calc = &mut Calc::new();
        let input = format!("{}*{};", u64::MAX, 1);
        let ast = calc.from_str(input.as_str()).unwrap();
        let bytecode = Calc::ast_to_bytecode(ast);
        calc.eval(&bytecode, scope).unwrap();
    }
}
