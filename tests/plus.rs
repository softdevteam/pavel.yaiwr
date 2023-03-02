#[cfg(test)]
mod tests {
    use yaiwr::Calc;
    #[test]
    fn eval_plus_expression() {
        let ast = Calc::from_str("2+2").unwrap();
        assert_eq!(Calc::eval(ast), Ok(4));
    }

    #[test]
    fn eval_plus_expressions() {
        let ast = Calc::from_str("2+2+2").unwrap();
        assert_eq!(Calc::eval(ast), Ok(6));
    }
}
