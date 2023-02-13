#[cfg(test)]
mod tests {
    use lrlex::lrlex_mod;
    use lrpar::lrpar_mod;
    use yaiwr::eval;

    lrlex_mod!("calc.l");
    lrpar_mod!("calc.y");

    #[test]
    fn evaluation_err() {
        let lexerdef = calc_l::lexerdef();
        assert_eq!(
            eval(&lexerdef, "non numperic input"),
            Err("Unable to evaluate expression.".into())
        );
    }
}
