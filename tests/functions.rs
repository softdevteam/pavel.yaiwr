#[cfg(test)]
mod tests {
    use std::process::Command;

    use yaiwr::{instruction::Instruction, Calc};

    #[test]
    fn var_test_file_expect_output_1984() {
        let cmd = "'fun _some (){ 2+2 }'";
        let output = Command::new("cargo")
            .arg("run")
            .arg(cmd)
            .output()
            .expect(format!("comand 'cargo run {}' failed", cmd).as_str());

        assert_eq!(String::from_utf8_lossy(&output.stdout), "",);
    }
    #[test]
    fn function_declaration() {
        let prog = "fun _some (){ return (2+2); }";
        let calc = &mut Calc::new();
        let ast = calc.from_str(prog).unwrap();
        let bytecode = &mut vec![];
        calc.to_bytecode(ast, bytecode);
        match bytecode.as_slice() {
            [first] => {
                assert_eq!(
                    first,
                    &Instruction::Function {
                        id: "_some".to_string(),
                        params: vec![],
                        block: vec![Instruction::Return {
                            block: vec![
                                Instruction::Push { value: 2 },
                                Instruction::Push { value: 2 },
                                Instruction::Add
                            ]
                        }]
                    }
                );
            }
            _ => panic!("expected bytecodes to be not empty!"),
        }
    }

    #[test]
    fn test_functions_args() {
        use std::process::Command;
        let output = Command::new("cargo")
            .arg("run")
            .arg("programs/tests/functions_expect_output_15.yaiwr")
            .output()
            .expect("command 'cargo run programs/tests/functions_expect_output_15.yaiwr' failed");

        assert_eq!(String::from_utf8_lossy(&output.stdout), "15\n");
    }
}
