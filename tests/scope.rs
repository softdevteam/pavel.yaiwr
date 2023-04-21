#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};
    use yaiwr::{instruction::StackValue, scope::Scope};

    #[test]
    fn scope_var_get() {
        let scope = Rc::new(RefCell::new(Scope::new()));
        scope
            .borrow_mut()
            .dec_var("id".to_string(), StackValue::Integer(0));
        scope
            .borrow_mut()
            .set_var("id".to_string(), StackValue::Integer(1))
            .unwrap();
        let var = scope.borrow().get_var("id".to_string()).unwrap();
        assert_eq!(var, StackValue::Integer(1));
    }

    #[test]
    fn outer_scope_var_get() {
        let outer_scope = Rc::new(RefCell::new(Scope::new()));

        outer_scope
            .borrow_mut()
            .dec_var("a".to_string(), StackValue::Integer(0));
        outer_scope
            .borrow_mut()
            .set_var("a".to_string(), StackValue::Integer(1))
            .unwrap();

        let inner = Rc::new(RefCell::new(Scope::from_scope(outer_scope)));
        inner
            .borrow_mut()
            .dec_var("b".to_string(), StackValue::Integer(0));
        inner
            .borrow_mut()
            .set_var("b".to_string().to_string(), StackValue::Integer(2))
            .unwrap();

        assert_eq!(
            inner.borrow().get_var("a".to_string()).unwrap(),
            StackValue::Integer(1)
        );
        assert_eq!(
            inner.borrow().get_var("b".to_string()).unwrap(),
            StackValue::Integer(2)
        );
    }

    #[test]
    fn outter_scope_mutation_only() {
        let outer_scope = Rc::new(RefCell::new(Scope::new()));

        outer_scope
            .borrow_mut()
            .dec_var("a".to_string(), StackValue::Integer(0));
        let outer_scope_clone = outer_scope.clone();

        let inner = Rc::new(RefCell::new(Scope::from_scope(outer_scope)));
        inner
            .borrow_mut()
            .set_var("a".to_string(), StackValue::Integer(2))
            .unwrap();

        assert_eq!(
            outer_scope_clone.borrow().get_var("a".to_string()).unwrap(),
            StackValue::Integer(2)
        );

        assert_eq!(outer_scope_clone.borrow().get_var_store_len(), 1);
        assert_eq!(inner.clone().borrow().get_var_store_len(), 0);
    }

    #[test]
    fn error_setting_undeclared_variable() {
        let outer_scope = Rc::new(RefCell::new(Scope::new()));
        let result = outer_scope
            .borrow_mut()
            .set_var("a".to_string(), StackValue::Boolean(false));
        assert_eq!(result, None)
    }
}
