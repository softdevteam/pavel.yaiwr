#[cfg(test)]
mod tests {
    
    use yaiwr::{
        instruction::{BinaryOp, Instruction, StackValue},
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
        let input = format!("{}+{};", u64::MAX, 1);
        Calc::eval_input(input).unwrap();
    }

    #[test]
    fn add_no_overflow() {
        let input = format!("{}+{};", u64::MAX - 1, 1);
        Calc::eval_input(input).unwrap();
    }
}
