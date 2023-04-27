#[cfg(test)]
mod tests {
    use yaiwr::{
        instruction::StackValue,
        scope::{Object, Scope},
    };

    #[test]
    fn scope_var_get() {
        let scope = Scope::new();
        scope.dec_var("id".to_string(), StackValue::Integer(0));
        scope.dec_var("id".to_string(), StackValue::Integer(1));
        let var = scope.get_var("id".to_string()).unwrap();
        assert_eq!(
            var,
            Object::Value {
                value: StackValue::Integer(1)
            }
        );
    }

    #[test]
    fn outer_scope_var_get() {
        let outer_scope = Scope::new();

        outer_scope.dec_var("a".to_string(), StackValue::Integer(0));
        outer_scope
            .set_var("a".to_string(), StackValue::Integer(1))
            .unwrap();

        let inner = Scope::from_scope("not-root".to_string(), outer_scope);
        inner.dec_var("b".to_string(), StackValue::Integer(0));
        inner
            .set_var("b".to_string().to_string(), StackValue::Integer(2))
            .unwrap();

        assert_eq!(
            inner.get_var("a".to_string()).unwrap(),
            Object::Value {
                value: StackValue::Integer(1)
            }
        );
        assert_eq!(
            inner.get_var("b".to_string()).unwrap(),
            Object::Value {
                value: StackValue::Integer(2)
            }
        );
    }

    #[test]
    fn outter_scope_mutation_only() {
        let outer_scope = Scope::new();

        outer_scope.dec_var("a".to_string(), StackValue::Integer(0));
        let outer_scope_clone = outer_scope.clone();

        let inner = Scope::from_scope("not-root".to_string(), outer_scope);
        inner
            .set_var("a".to_string(), StackValue::Integer(2))
            .unwrap();

        assert_eq!(
            outer_scope_clone.get_var("a".to_string()).unwrap(),
            Object::Value {
                value: StackValue::Integer(2)
            }
        );

        assert_eq!(outer_scope_clone.get_store_len(), 1);
        assert_eq!(inner.clone().get_store_len(), 0);
    }

    #[test]
    fn error_setting_undeclared_variable() {
        let outer_scope = Scope::new();
        let result = outer_scope.set_var("a".to_string(), StackValue::Boolean(false));
        assert_eq!(result, None)
    }
}
