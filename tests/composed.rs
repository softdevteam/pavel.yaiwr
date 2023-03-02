#[cfg(test)]
mod tests {
    use yaiwr::Calc;

    #[test]
    fn eval_mul_and_plus_expressions() {
        assert_eq!(
            Calc::eval(Calc::from_str("2*3+2").unwrap()),
            Ok(8),
            "expected 2*3+2=8"
        );
        assert_eq!(
            Calc::eval(Calc::from_str("2+3*2").unwrap()),
            Ok(8),
            "expected 2+3*2=8"
        );
        assert_eq!(
            Calc::eval(Calc::from_str("(2+3)*2").unwrap()),
            Ok(10),
            "expected (2+3)*2=8"
        );
    }
}
