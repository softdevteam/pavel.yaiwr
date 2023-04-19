#[cfg(test)]
mod tests {
    use yaiwr::{
        hash::HashId,
        instruction::{BinaryOp, Instruction, StackValue},
        Calc,
    };

    #[test]
    fn var_assign_bc() {
        let calc = &mut Calc::new();
        let ast = calc.from_str("let _ABCDabc123 = 1984;").unwrap();
        let bytecode = Calc::ast_to_bytecode(ast);
        match bytecode.as_slice() {
            [bc1, bc2] => {
                assert_eq!(
                    bc1,
                    &Instruction::Push {
                        value: StackValue::Integer(1984)
                    }
                );

                assert_eq!(
                    bc2,
                    &Instruction::BinaryOp {
                        op: BinaryOp::Declare { 
                            name: "_ABCDabc123".to_string(),
                            id: "_ABCDabc123".id()
                        }
                    }
                );
            }
            _ => panic!("expected bytecodes to be not empty!"),
        }
    }
}
