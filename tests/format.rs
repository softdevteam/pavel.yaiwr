#[cfg(test)]
mod tests {
    use yaiwr::{err::InterpError, scope::Scope, Calc};

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
    fn multiline_function_calls() {
        let calc = &mut Calc::new();
        let scope = &mut Scope::new();
        eval_prog(
            calc,
            "  
            fun f3(){ 
                println(2);
            }  
            fun f2(_a, _b){ 
                f3();
                return _a + _b;
            }
            fun f1(_a, _b){ 
                return f2(_a, 10); + _b;
            }
        ",
            scope,
        )
        .unwrap();
        assert_eq!(eval_prog(calc, "f1(1,2);", scope).unwrap().unwrap(), 13);
    }

    #[test]
    fn multistatments_single_line() {
        let calc = &mut Calc::new();
        let scope = &mut Scope::new();
        let result = eval_prog(calc, "let _a = 2; let _b = 4; (_a + _b)", scope).unwrap();
        assert_eq!(result.unwrap(), 6);
    }
}
