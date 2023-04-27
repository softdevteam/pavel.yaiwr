#[cfg(test)]
mod tests {
    use yaiwr::{
        instruction::{BinaryOp, EvalResult, Instruction, StackValue},
        scope::Scope,
        YIWR,
    };

    #[test]
    fn eval_mul_expression() {
        let scope = Scope::new();
        let yaiwr = &mut YIWR::new();
        let ast = yaiwr.from_str("2*2;").unwrap();
        let bytecode = YIWR::ast_to_bytecode(ast);
        assert_eq!(
            yaiwr.eval(&bytecode, scope).unwrap(),
            Some(EvalResult::Value(StackValue::Integer(4)))
        );
    }

    #[test]
    fn eval_mul_expressions() {
        let scope = Scope::new();
        let yaiwr = &mut YIWR::new();
        let ast = yaiwr.from_str("2*2*2;").unwrap();
        let bytecode = YIWR::ast_to_bytecode(ast);
        assert_eq!(
            yaiwr.eval(&bytecode, scope).unwrap(),
            Some(EvalResult::Value(StackValue::Integer(8)))
        );
    }

    #[test]
    fn mul_bc() {
        let yaiwr = &mut YIWR::new();
        let ast = yaiwr.from_str("1*2;").unwrap();
        let bytecode = YIWR::ast_to_bytecode(ast);
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
        let scope = Scope::new();
        let yaiwr = &mut YIWR::new();
        let input = format!("{}*{};", u64::MAX, 2);
        let ast = yaiwr.from_str(input.as_str()).unwrap();
        let bytecode = YIWR::ast_to_bytecode(ast);
        yaiwr.eval(&bytecode, scope).unwrap();
    }

    #[test]
    fn mul_no_overflow() {
        let scope = Scope::new();
        let yaiwr = &mut YIWR::new();
        let input = format!("{}*{};", u64::MAX, 1);
        let ast = yaiwr.from_str(input.as_str()).unwrap();
        let bytecode = YIWR::ast_to_bytecode(ast);
        yaiwr.eval(&bytecode, scope).unwrap();
    }
}
