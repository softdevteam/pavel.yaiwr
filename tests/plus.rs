#[cfg(test)]
mod tests {
    use yaiwr::Calc;
    #[test]
    fn eval_plus_expression() {
        let ast = Calc::from_str("2+2").unwrap();
        let bytecode = &mut vec![];
        Calc::to_bytecode(ast, bytecode);
        assert_eq!(Calc::eval(bytecode), Ok(4));
    }

    #[test]
    fn eval_plus_expressions() {
        let ast = Calc::from_str("2+2+2").unwrap();
        let bytecode = &mut vec![];
        Calc::to_bytecode(ast, bytecode);
        assert_eq!(Calc::eval(bytecode), Ok(6));
    }
}
