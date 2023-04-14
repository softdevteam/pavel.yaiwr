#[cfg(test)]
mod tests {
    use std::{cell::RefCell};

    use yaiwr::{instruction::StackValue, scope::Scope};

    #[test]
    fn scope_var_get() {
        let scope = &mut Scope::new();
        scope.dec_var("id".to_string());
        scope.set_var("id".to_string(), StackValue::Integer(42));
        let var = scope.get_var(&"id".to_string()).unwrap();
        assert_eq!(var, StackValue::Integer(42));
    }

    #[test]
    fn outer_scope_var_get() {
        let outer_scope = &mut Scope::new();
        
        outer_scope.dec_var("a".to_string());
        outer_scope.set_var("a".to_string(), StackValue::Integer(42));
    
        let inner = RefCell::new(Box::new(outer_scope));
        inner.borrow_mut().dec_var("b".to_string());
        
        inner.borrow_mut().set_var("b".to_string(), StackValue::Integer(43));        

        assert_eq!(inner.borrow().get_var(&"a".to_string()).unwrap(), StackValue::Integer(42));
        assert_eq!(inner.borrow().get_var(&"b".to_string()).unwrap(), StackValue::Integer(43));
    }

    #[test]
    fn inner_scope_variable_set_no_outter_scope_mutation() { 
        let outer_scope = &mut Scope::new();
        
        outer_scope.dec_var("a".to_string());
        
    
        let inner = RefCell::new(Box::new(outer_scope));
        
        inner.borrow_mut().set_var("a".to_string(), StackValue::Integer(43));        

        assert_eq!(inner.borrow().get_var(&"a".to_string()).unwrap(), StackValue::Integer(43));
    }
}
