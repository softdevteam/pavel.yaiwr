#[cfg(test)]
mod tests {
    use yaiwr::{
        instruction::{Instruction, StackValue},
        YIWR,
    };

    #[test]
    fn integer_literal_bc() {
        let yaiwr = &mut YIWR::new();
        let ast = yaiwr.from_str("1;").unwrap();
        let bytecode = YIWR::ast_to_bytecode(ast);
        match bytecode.as_slice() {
            [first] => {
                assert_eq!(
                    first,
                    &Instruction::Push {
                        value: StackValue::Integer(1)
                    }
                );
            }
            _ => panic!("expected bytecodes to be not empty!"),
        }
    }
}
