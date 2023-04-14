#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use yaiwr::{
        instruction::{BinaryOp, Instruction, StackValue},
        scope::Scope,
        Calc,
    };

    #[test]
    fn add_bc() {
        let calc = &mut Calc::new();
        let ast = calc.from_str("1+2;").unwrap();
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
                assert_eq!(third, &Instruction::BinaryOp { op: BinaryOp::Add });
            }
            _ => panic!("expected bytecodes to be not empty!"),
        }
    }

    #[test]
    #[should_panic(expected = "overflowed")]
    fn add_overflow_max_u64() {
        let scope = &RefCell::new(Box::new(Scope::new()));
        let calc = &mut Calc::new();
        let input = format!("{}+{};", u64::MAX, 1);
        let ast = calc.from_str(input.as_str()).unwrap();
        let bytecode = Calc::ast_to_bytecode(ast);
        calc.eval(&bytecode, scope).unwrap();
    }

    #[test]
    fn add_no_overflow() {
        let scope = &RefCell::new(Box::new(Scope::new()));
        let calc = &mut Calc::new();
        let input = format!("{}+{};", u64::MAX - 1, 1);
        let ast = calc.from_str(input.as_str()).unwrap();
        let bytecode = Calc::ast_to_bytecode(ast);
        calc.eval(&bytecode, scope).unwrap();
    }
}
