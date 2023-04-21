#[cfg(test)]
mod tests {
    use yaiwr::{
        instruction::StackValue,
        instruction::{BinaryOp, Instruction},
        YIWR,
    };

    #[test]
    fn conditional_no_alternative_bc() {
        let yaiwr = &mut YIWR::new();
        let ast = yaiwr
            .from_str(
                "
            if (5 > 1){
                println(9);
            }
        ",
            )
            .unwrap();

        let bytecode = YIWR::ast_to_bytecode(ast);
        match bytecode.as_slice() {
            [first] => {
                assert_eq!(
                    first,
                    &Instruction::Conditional {
                        condition: vec![
                            Instruction::Push {
                                value: StackValue::Integer(5)
                            },
                            Instruction::Push {
                                value: StackValue::Integer(1)
                            },
                            Instruction::BinaryOp {
                                op: BinaryOp::GreaterThan {},
                            }
                        ],
                        alternative: None,
                        block: vec![
                            Instruction::Push {
                                value: StackValue::Integer(9)
                            },
                            Instruction::PrintLn,
                        ]
                    }
                );
            }
            _ => panic!("expected bytecodes to be not empty!"),
        }
    }

    #[test]
    fn conditional_with_alternative_bc() {
        let yaiwr = &mut YIWR::new();
        let ast = yaiwr
            .from_str(
                "
            if (5 > 1){
                println(1);
            }else{
                println(2);
            }
        ",
            )
            .unwrap();

        let bytecode = YIWR::ast_to_bytecode(ast);
        match bytecode.as_slice() {
            [first] => {
                assert_eq!(
                    first,
                    &Instruction::Conditional {
                        condition: vec![
                            Instruction::Push {
                                value: StackValue::Integer(5)
                            },
                            Instruction::Push {
                                value: StackValue::Integer(1)
                            },
                            Instruction::BinaryOp {
                                op: BinaryOp::GreaterThan {},
                            }
                        ],
                        block: vec![
                            Instruction::Push {
                                value: StackValue::Integer(1)
                            },
                            Instruction::PrintLn,
                        ],
                        alternative: Some(vec![
                            Instruction::Push {
                                value: StackValue::Integer(2)
                            },
                            Instruction::PrintLn,
                        ]),
                    }
                );
            }
            _ => panic!("expected bytecodes to be not empty!"),
        }
    }
}
