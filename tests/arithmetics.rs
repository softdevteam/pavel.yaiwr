#[cfg(test)]

mod tests {
    use std::cell::RefCell;

    use yaiwr::{
        instruction::{EvalResult, StackValue},
        scope::Scope,
        Calc,
    };

    #[test]
    fn eval_mul_and_plus_expressions() {
        let scope = RefCell::new(Box::new(Scope::new()));
        let mut c = Calc::new();
        let bytecode = Calc::ast_to_bytecode(c.from_str("2*3+2;").unwrap());
        assert_eq!(
            c.eval(&bytecode, &scope).unwrap().unwrap(),
            EvalResult::Value(StackValue::Integer(8)),
            "expected 2*3+2=8"
        );

        let bytecode = Calc::ast_to_bytecode(c.from_str("2+3*2;").unwrap());
        assert_eq!(
            c.eval(&bytecode, &scope).unwrap().unwrap(),
            EvalResult::Value(StackValue::Integer(8)),
            "expected 2+3*2=8"
        );

        let bytecode = Calc::ast_to_bytecode(c.from_str("(2+3)*2;").unwrap());
        assert_eq!(
            c.eval(&bytecode, &scope).unwrap().unwrap(),
            EvalResult::Value(StackValue::Integer(10)),
            "expected (2+3)*2=8"
        );
    }
}
