#[cfg(test)]

mod tests {
    use yaiwr::{
        instruction::{EvalResult, StackValue},
        Calc,
    };

    #[test]
    fn eval_mul_and_plus_expressions() {
        assert_eq!(
            Calc::eval_input("2*3+2;".to_string()).unwrap().unwrap(),
            EvalResult::Value(StackValue::Integer(8)),
            "expected 2*3+2=8"
        );

        assert_eq!(
            Calc::eval_input("2+3*2;".to_string()).unwrap().unwrap(),
            EvalResult::Value(StackValue::Integer(8)),
            "expected 2+3*2=8"
        );

        assert_eq!(
            Calc::eval_input("(2+3)*2;".to_string()).unwrap().unwrap(),
            EvalResult::Value(StackValue::Integer(10)),
            "expected (2+3)*2=10"
        );
    }
}
