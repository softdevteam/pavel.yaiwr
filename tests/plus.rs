#[cfg(test)]
mod tests {
    use lrlex::lrlex_mod;
    use lrpar::lrpar_mod;
    use yaiwr::eval;

    lrlex_mod!("calc.l");
    lrpar_mod!("calc.y");

    #[test]
    fn eval_plus_exp() {
        let lexerdef = calc_l::lexerdef();
        assert_eq!(eval(&lexerdef, "1+1"), Ok("Result: 2".into()));
    }
}
