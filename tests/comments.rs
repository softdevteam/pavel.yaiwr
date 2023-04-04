mod print;

#[cfg(test)]
mod tests {
    use yaiwr::{scope::Scope, Calc};

    #[test]
    fn comment_no_evaluation_bc() {
        let calc = &mut Calc::new();
        let scope = &mut Scope::new();
        let ast = calc.from_str("// let _a = 5;\n").unwrap();
        let bytecode = Calc::ast_to_bytecode(ast);
        assert_eq!(calc.eval(&bytecode, scope).unwrap(), None);
        assert_eq!(bytecode.len(), 0);
    }
}
