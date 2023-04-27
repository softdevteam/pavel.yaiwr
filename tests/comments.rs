mod print;

#[cfg(test)]
mod tests {
    use yaiwr::{scope::Scope, YIWR};

    #[test]
    fn comment_no_evaluation_bc() {
        let scope = Scope::new();
        let yaiwr = &mut YIWR::new();

        let ast = yaiwr.from_str("// let _a = 5;\n").unwrap();
        let bytecode = YIWR::ast_to_bytecode(ast);

        assert_eq!(yaiwr.eval(&bytecode, scope).unwrap(), None);
        assert_eq!(bytecode.len(), 0);
    }
}
