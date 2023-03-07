#[cfg(test)]
mod tests {
    use yaiwr::Calc;

    #[test]
    fn eval_mul_and_plus_expressions() {
        let bytecode = &mut vec![];
        Calc::to_bytecode(Calc::from_str("2*3+2").unwrap(), bytecode);
        assert_eq!(Calc::eval(bytecode), Ok(8), "expected 2*3+2=8");
        let bytecode = &mut vec![];
        Calc::to_bytecode(Calc::from_str("2+3*2").unwrap(), bytecode);
        assert_eq!(Calc::eval(bytecode), Ok(8), "expected 2+3*2=8");
        let bytecode = &mut vec![];
        Calc::to_bytecode(Calc::from_str("(2+3)*2").unwrap(), bytecode);
        assert_eq!(Calc::eval(bytecode), Ok(10), "expected (2+3)*2=10");
    }
}
