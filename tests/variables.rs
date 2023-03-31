#[cfg(test)]
mod tests {
    use std::process::Command;
    use yaiwr::{scope::Scope, Calc};

    #[test]
    fn var_single_numeric() {
        let calc = &mut Calc::new();
        let ast = calc.from_str("let _a = 2;").unwrap();
        let bytecode = Calc::ast_to_bytecode(ast);
        let scope = &mut Scope::new();
        calc.eval(&bytecode, scope).unwrap();
        assert_eq!(scope.get_var(&"_a".to_string()).unwrap(), &2);
    }
    #[test]
    fn var_expression() {
        let calc = &mut Calc::new();
        let ast = calc.from_str("let _b = (1+2*3);").unwrap();
        let bytecode = Calc::ast_to_bytecode(ast);
        let scope = &mut Scope::new();
        calc.eval(&bytecode, scope).unwrap();
        assert_eq!(scope.get_var(&"_b".to_string()).unwrap(), &7);
    }

    #[test]
    fn var_multiple_lower_upper_numeric() {
        let calc = &mut Calc::new();
        let ast = calc.from_str("let _ABCDabc123 = 1984;").unwrap();
        let bytecode = Calc::ast_to_bytecode(ast);
        let scope = &mut Scope::new();
        calc.eval(&bytecode, scope).unwrap();
        assert_eq!(scope.get_var(&"_ABCDabc123".to_string()).unwrap(), &1984);
    }

    #[test]
    fn var_single_lower_upper_numeric() {
        let calc = &mut Calc::new();
        let ast = calc.from_str("let _aB1 = 1984;").unwrap();
        let bytecode = Calc::ast_to_bytecode(ast);
        let scope = &mut Scope::new();
        calc.eval(&bytecode, scope).unwrap();
        assert_eq!(scope.get_var(&"_aB1".to_string()).unwrap(), &1984);
    }

    #[test]
    fn var_test_file_expect_output_10() {
        let file_path = "./programs/tests/var_expect_output_10.yaiwr";
        let output = Command::new("cargo")
            .arg("run")
            .arg(file_path)
            .output()
            .expect(format!("comand 'cargo run {}' failed", file_path).as_str());

        assert_eq!(String::from_utf8_lossy(&output.stdout), "10\n",);
    }

    #[test]
    fn var_test_file_expect_output_1984() {
        let file_path = "./programs/tests/var_expect_output_1984.yaiwr";
        let output = Command::new("cargo")
            .arg("run")
            .arg(file_path)
            .output()
            .expect(format!("comand 'cargo run {}' failed", file_path).as_str());

        assert_eq!(String::from_utf8_lossy(&output.stdout), "1984\n",);
    }
}
