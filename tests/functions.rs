#[cfg(test)]
mod tests {
    use yaiwr::{err::InterpError, instruction::Instruction, Calc, scope::Scope};

    pub fn eval_prog(
        calc: &mut Calc,
        input: &str,
        scope: &mut Scope,
    ) -> Result<Option<u64>, InterpError> {
        let ast = calc.from_str(input).unwrap();
        let bytecode = Calc::ast_to_bytecode(ast);
        return calc.eval(&bytecode, scope);
    }

    #[test]
    fn function_call_err() {
        let calc = &mut Calc::new();
        let scope = &mut Scope::new();
        eval_prog(calc, "fun add1 (_p1){ return _p1 + 1; }", scope).unwrap();
        assert_eq!(
            eval_prog(calc, "add1();", scope),
            Err(InterpError::EvalError(
                "Unexpected number of function arguments. Expected: 1, Got: 0".to_string()
            ))
        );
    }

    #[test]
    fn function_undefined_err() {
        let scope = &mut Scope::new();
        let calc = &mut Calc::new();
        assert_eq!(
            eval_prog(calc, "add1();", scope),
            Err(InterpError::UndefinedFunction("add1".to_string()))
        );
    }

    #[test]
    fn function_composition() {
        let calc = &mut Calc::new();
        let scope = &mut Scope::new();
        eval_prog(calc, "fun add1 (_p1){ return _p1 + 1; }", scope).unwrap();
        eval_prog(calc, "fun add2 (_p1){ return _p1 + 2; }", scope).unwrap();
        assert_eq!(
            eval_prog(calc, "add2(add1(1););", scope).unwrap().unwrap(),
            4
        );
    }

    #[test]
    fn function_multiple_params() {
        let scope = &mut Scope::new();
        let calc = &mut Calc::new();
        eval_prog(
            calc,
            "fun add (_p1, _p2, _p3){ return _p1 + _p2 +_p3; }",
            scope,
        )
        .unwrap();
        assert_eq!(eval_prog(calc, "add(1,2,3);", scope).unwrap().unwrap(), 6);
    }

    #[test]
    fn function_params_as_variables() {
        let scope = &mut Scope::new();
        let calc = &mut Calc::new();
        eval_prog(calc, "let _x = 2;", scope).unwrap();
        eval_prog(calc, "let _y = 3;", scope).unwrap();
        eval_prog(
            calc,
            "fun add (_arg1, _arg2){ return _arg1 + _arg2; }",
            scope,
        )
        .unwrap();
        assert_eq!(eval_prog(calc, "add(_x, _y);", scope).unwrap().unwrap(), 5);
    }

    #[test]
    fn function_declaration_no_params_bytecode() {
        let calc = &mut Calc::new();
        let prog1 = "fun some (){ return 2*2; }";
        let ast = calc.from_str(prog1).unwrap();
        let func_declare_bc = Calc::ast_to_bytecode(ast);
        calc.eval(&func_declare_bc, &mut Scope::new()).unwrap();
        match func_declare_bc.as_slice() {
            [first] => {
                assert_eq!(
                    first,
                    &Instruction::Function {
                        id: "some".to_string(),
                        params: vec![],
                        block: vec![Instruction::Return {
                            block: vec![
                                Instruction::Push { value: 2 },
                                Instruction::Push { value: 2 },
                                Instruction::Mul
                            ]
                        }]
                    }
                );
            }
            _ => panic!("expected bytecodes to be not empty!"),
        }
    }

    #[test]
    fn function_declaration_with_params_bytecode() {
        let calc = &mut Calc::new();
        let prog = "fun add (_p1, _p2){ return _p1 + _p2 + 1; }";
        let ast = calc.from_str(prog).unwrap();
        let func_declare_bc = Calc::ast_to_bytecode(ast);
        calc.eval(&func_declare_bc, &mut Scope::new()).unwrap();
        match func_declare_bc.as_slice() {
            [first] => {
                assert_eq!(
                    first,
                    &Instruction::Function {
                        id: "add".to_string(),
                        params: vec!["_p1".to_string(), "_p2".to_string()],
                        block: vec![Instruction::Return {
                            block: vec![
                                Instruction::Load {
                                    id: "_p1".to_string()
                                },
                                Instruction::Load {
                                    id: "_p2".to_string()
                                },
                                Instruction::Add,
                                Instruction::Push { value: 1 },
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
    fn function_declaration_with_params_call_bytecode() {
        let calc = &mut Calc::new();
        let prog_func_declaration = "fun add (_p1, _p2){ return _p1 + _p2; }";
        let ast = calc.from_str(prog_func_declaration).unwrap();
        let func_declaration_bc = Calc::ast_to_bytecode(ast);
        calc.eval(&func_declaration_bc, &mut Scope::new()).unwrap();
        match func_declaration_bc.as_slice() {
            [first] => {
                assert_eq!(
                    first,
                    &Instruction::Function {
                        id: "add".to_string(),
                        params: vec!["_p1".to_string(), "_p2".to_string()],
                        block: vec![Instruction::Return {
                            block: vec![
                                Instruction::Load {
                                    id: "_p1".to_string()
                                },
                                Instruction::Load {
                                    id: "_p2".to_string()
                                },
                                Instruction::Add
                            ]
                        }]
                    }
                );
            }
            _ => panic!("expected bytecodes to be not empty!"),
        }

        let prog_func_call = "add(1,2);";
        let ast = calc.from_str(prog_func_call).unwrap();
        let func_call_bc = Calc::ast_to_bytecode(ast);
        calc.eval(&func_call_bc, &mut Scope::new()).unwrap();
        match func_call_bc.as_slice() {
            [first] => {
                assert_eq!(
                    first,
                    &Instruction::FunctionCall {
                        id: "add".to_string(),
                        args: vec![
                            vec![Instruction::Push { value: 1 }],
                            vec![Instruction::Push { value: 2 }],
                        ]
                    }
                );
            }
            _ => panic!("expected bytecodes to be not empty!"),
        }
    }

    #[test]
    fn function_declaration_no_params_call_bytecode() {
        let calc = &mut Calc::new();
        let prog_func_declaration = "fun two_plus_two (){ return (2+2); }";
        let ast = calc.from_str(prog_func_declaration).unwrap();
        let func_declare_bc = Calc::ast_to_bytecode(ast);
        calc.eval(&func_declare_bc, &mut Scope::new()).unwrap();
        match func_declare_bc.as_slice() {
            [first] => {
                assert_eq!(
                    first,
                    &Instruction::Function {
                        id: "two_plus_two".to_string(),
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
        let prog_func_call = "two_plus_two();";
        let ast = calc.from_str(prog_func_call).unwrap();
        let func_call_bc = Calc::ast_to_bytecode(ast);
        calc.eval(&func_call_bc, &mut Scope::new()).unwrap();
        match func_call_bc.as_slice() {
            [first] => {
                assert_eq!(
                    first,
                    &Instruction::FunctionCall {
                        id: "two_plus_two".to_string(),
                        args: vec![]
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

    #[test]
    fn function_scope_outter() {
        let calc = &mut Calc::new();
        let scope = &mut Scope::new();
        eval_prog(calc, "let _outter_var = 666;", scope).unwrap();
        eval_prog(calc, "fun add1 (){ return _outter_var + 1; }", scope).unwrap();
        assert_eq!(eval_prog(calc, "add1();", scope).unwrap().unwrap(), 667);
    }

    #[test]
    fn function_scope_variables_not_leaking() {
        let calc = &mut Calc::new();
        let scope = &mut Scope::new();
        eval_prog(calc, "fun f1 (){ let _a = 1; }", scope).unwrap();
        eval_prog(calc, "fun f2 (){ return _a + 1; }", scope).unwrap();
        assert_eq!(
            eval_prog(calc, "f2();", scope),
            Err(InterpError::VariableNotFound("_a".to_string()))
        );
    }

    #[test]
    fn function_outter_scope_with_function_call() {
        let calc = &mut Calc::new();
        let scope = &mut Scope::new();
        eval_prog(calc, "let _a = 1;", scope).unwrap();
        eval_prog(calc, "fun f1 (){ return _a; }", scope).unwrap();
        eval_prog(calc, "fun f2 (){ return f1(); + _a; }", scope).unwrap();
        assert_eq!(eval_prog(calc, "f2();", scope).unwrap().unwrap(), 2);
    }

    #[test]
    fn function_body_multiline_statements() {
        let calc = &mut Calc::new();
        let scope = &mut Scope::new();
        eval_prog(calc,  "fun do_some (){ 
            let _a = 2; 
            let _b = 2; 
            return _a + _b; 
        }", scope).unwrap();
        assert_eq!(eval_prog(calc, "do_some();", scope).unwrap().unwrap(), 4);
    }

    #[test]
    fn function_body_multiline_function_calls() {
        let calc = &mut Calc::new();
        let scope = &mut Scope::new();
        eval_prog(calc,  "    
            fun add(_a, _b){ 
                return _a + _b;
            }
            fun f1(_a, _b){ 
                return add(_a, 10); + _b;
            }
        ", scope).unwrap();
        assert_eq!(eval_prog(calc, "f1(1,2);", scope).unwrap().unwrap(), 13);
    }

    #[test]
    fn function_body_multiline_function_scope() {
        let calc = &mut Calc::new();
        let scope = &mut Scope::new();
        eval_prog(calc,  "    
            fun set_a(){ 
                let _a = 10;
            }
            fun f1(_a){ 
                set_a();
                return _a;
            }
        ", scope).unwrap();
        assert_eq!(eval_prog(calc, "f1(1);", scope).unwrap().unwrap(), 1);
    }
}
