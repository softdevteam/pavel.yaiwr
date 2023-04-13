#[cfg(test)]
mod tests {
    use yaiwr::{instruction::StackValue, scope::Scope};

    #[test]
    fn scope_var_get() {
        let scope = &mut Scope::new();
        scope.set_var("id".to_string(), StackValue::Integer(42));
        let var = scope.get_var(&"id".to_string()).unwrap();
        assert_eq!(var, &StackValue::Integer(42));
    }

    #[test]
    fn outer_scope_var_get() {
        let outer_scope = &mut Scope::new();
        outer_scope.set_var("a".to_string(), StackValue::Integer(42));
        outer_scope.set_var("b".to_string(), StackValue::Integer(42));

        let scope = &mut Scope::from_scope(&outer_scope);

        assert_eq!(
            scope.get_var(&"a".to_string()).unwrap(),
            &StackValue::Integer(42)
        );
        assert_eq!(
            scope.get_var(&"b".to_string()).unwrap(),
            &StackValue::Integer(42)
        );
    }

    #[test]
    fn inner_scope_variable_set_no_outter_scope_mutation() {
        let outer_scope = &mut Scope::new();
        outer_scope.set_var("a".to_string(), StackValue::Integer(1));

        {
            let scope = &mut Scope::from_scope(&outer_scope);
            scope.set_var("a".to_string(), StackValue::Integer(2));
            assert_eq!(
                scope.get_var(&"a".to_string()).unwrap(),
                &StackValue::Integer(2)
            );
        }
        assert_eq!(
            outer_scope.get_var(&"a".to_string()).unwrap(),
            &StackValue::Integer(1)
        );
    }
}
