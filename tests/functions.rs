#[cfg(test)]
mod tests {

    use std::{cell::RefCell, rc::Rc};

    use yaiwr::{
        err::InterpError,
        instruction::{BinaryOp, EvalResult, Instruction, StackValue},
        scope::Scope,
        Calc,
    };

    pub fn eval_prog<'a>(
        calc: &mut Calc,
        input: &str,
        scope: Rc<RefCell<Scope>>,
    ) -> Result<Option<EvalResult>, InterpError> {
        let ast = calc.from_str(input).unwrap();
        let bytecode = Calc::ast_to_bytecode(ast);
        return calc.eval(&bytecode, scope);
    }

    #[test]
    fn function_call_err() {
        let scope = Rc::new(RefCell::new(Scope::new()));
        let calc = &mut Calc::new();
        eval_prog(calc, "fun add1 (_p1){ return _p1 + 1; }", scope.clone()).unwrap();
        assert_eq!(
            eval_prog(calc, "add1();", scope),
            Err(InterpError::EvalError(
                "Unexpected number of function arguments. Expected: 1, Got: 0".to_string()
            ))
        );
    }

    #[test]
    fn function_undefined_err() {
        let scope = Rc::new(RefCell::new(Scope::new()));
        let calc = &mut Calc::new();
        assert_eq!(
            eval_prog(calc, "add1();", scope),
            Err(InterpError::UndefinedFunction("add1".to_string()))
        );
    }

    #[test]
    fn function_composition() {
        let scope = Rc::new(RefCell::new(Scope::new()));
        let calc = &mut Calc::new();
        eval_prog(calc, "fun add1 (_p1){ return _p1 + 1; }", scope.clone()).unwrap();
        eval_prog(calc, "fun add2 (_p1){ return _p1 + 2; }", scope.clone()).unwrap();
        assert_eq!(
            eval_prog(calc, "add2(add1(1));", scope).unwrap().unwrap(),
            EvalResult::Value(StackValue::Integer(4))
        );
    }

    #[test]
    fn function_multiple_params() {
        let scope = Rc::new(RefCell::new(Scope::new()));
        let calc = &mut Calc::new();
        eval_prog(
            calc,
            "fun add (p1, p2, p3){ return p1 + p2 +p3; }",
            scope.clone(),
        )
        .unwrap();
        assert_eq!(
            eval_prog(calc, "add(1,2,3);", scope).unwrap().unwrap(),
            EvalResult::Value(StackValue::Integer(6))
        );
    }

    #[test]
    fn function_declaration_no_params_bc() {
        let scope = Rc::new(RefCell::new(Scope::new()));
        let calc = &mut Calc::new();
        let prog1 = "fun some (){ return 2*2; }";
        let ast = calc.from_str(prog1).unwrap();
        let func_declare_bc = Calc::ast_to_bytecode(ast);
        calc.eval(&func_declare_bc, scope).unwrap();
        match func_declare_bc.as_slice() {
            [first] => {
                assert_eq!(
                    first,
                    &Instruction::Function {
                        id: "some".to_string(),
                        params: vec![],
                        block: vec![Instruction::Return {
                            block: vec![
                                Instruction::Push {
                                    value: StackValue::Integer(2)
                                },
                                Instruction::Push {
                                    value: StackValue::Integer(2)
                                },
                                Instruction::BinaryOp { op: BinaryOp::Mul }
                            ]
                        }]
                    }
                );
            }
            _ => panic!("expected bytecodes to be not empty!"),
        }
    }

    #[test]
    fn function_declaration_with_params_bc() {
        let scope = Rc::new(RefCell::new(Scope::new()));
        let calc = &mut Calc::new();
        let prog = "fun add (_p1, _p2){ return _p1 + _p2 + 1; }";
        let ast = calc.from_str(prog).unwrap();
        let func_declare_bc = Calc::ast_to_bytecode(ast);
        calc.eval(&func_declare_bc, scope).unwrap();
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
                                Instruction::BinaryOp { op: BinaryOp::Add },
                                Instruction::Push {
                                    value: StackValue::Integer(1)
                                },
                                Instruction::BinaryOp { op: BinaryOp::Add },
                            ]
                        }]
                    }
                );
            }
            _ => panic!("expected bytecodes to be not empty!"),
        }
    }

    #[test]
    fn function_declaration_with_params_call_bc() {
        let scope = Rc::new(RefCell::new(Scope::new()));
        let calc = &mut Calc::new();
        let prog_func_declaration = "fun add (_p1, _p2){ return _p1 + _p2; }";
        let ast = calc.from_str(prog_func_declaration).unwrap();
        let func_declaration_bc = Calc::ast_to_bytecode(ast);
        calc.eval(&func_declaration_bc, scope).unwrap();
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
                                Instruction::BinaryOp { op: BinaryOp::Add },
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
        let scope = Rc::new(RefCell::new(Scope::new()));
        calc.eval(&func_call_bc, scope).unwrap();
        match func_call_bc.as_slice() {
            [first] => {
                assert_eq!(
                    first,
                    &Instruction::FunctionCall {
                        id: "add".to_string(),
                        args: vec![
                            vec![Instruction::Push {
                                value: StackValue::Integer(1)
                            }],
                            vec![Instruction::Push {
                                value: StackValue::Integer(2)
                            }],
                        ]
                    }
                );
            }
            _ => panic!("expected bytecodes to be not empty!"),
        }
    }

    #[test]
    fn function_declaration_no_params_call_bc() {
        let calc = &mut Calc::new();
        let prog_func_declaration = "fun two_plus_two (){ return (2+2); }";
        let ast = calc.from_str(prog_func_declaration).unwrap();
        let func_declare_bc = Calc::ast_to_bytecode(ast);
        match func_declare_bc.as_slice() {
            [first] => {
                assert_eq!(
                    first,
                    &Instruction::Function {
                        id: "two_plus_two".to_string(),
                        params: vec![],
                        block: vec![Instruction::Return {
                            block: vec![
                                Instruction::Push {
                                    value: StackValue::Integer(2)
                                },
                                Instruction::Push {
                                    value: StackValue::Integer(2)
                                },
                                Instruction::BinaryOp { op: BinaryOp::Add },
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
}
