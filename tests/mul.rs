#[cfg(test)]
mod tests {
    use yaiwr::Calc;

    #[test]
    fn eval_mul_expression() {
        let ast = Calc::from_str("2*2").unwrap();
        assert_eq!(Calc::eval(ast), Ok(4));
    }

    #[test]
    fn eval_mul_expressions() {
        let ast = Calc::from_str("2*2*2").unwrap();
        assert_eq!(Calc::eval(ast), Ok(8));
    }
}
