use std::collections::HashMap;

use crate::{err::InterpError, instruction::StackValue};

#[derive(Debug)]
pub struct Scope<'a> {
    var_store: HashMap<String, StackValue>,
    outter_scope: Option<&'a Scope<'a>>,
}

impl<'a> Scope<'a> {
    pub fn new() -> Self {
        Scope {
            var_store: HashMap::new(),
            outter_scope: None,
        }
    }

    fn get_scope_var(&self, scope: &'a Scope, id: &String) -> Result<&'a Scope<'a>, InterpError> {
        let var = scope
            .var_store
            .get(id);

        match var {
            Some(_) => Ok(scope),
            None => {
                if let Some(s) = scope.outter_scope {
                    scope.get_scope_var(s, id)
                } else {
                    Err(InterpError::VariableNotFound(id.to_string()))
                }
            }
        }
    }

    pub fn get_var(&self, id: &String) -> Result<&StackValue, InterpError> {
        let scope = self.get_scope_var(self, id)?;

        scope
            .var_store
            .get(id)
            .ok_or(InterpError::VariableNotFound(id.to_string()))
    }

    pub fn set_var(&mut self, id: String, val: StackValue) -> Option<StackValue> {
        self.var_store.insert(id, val)
    }

    pub fn from_scope(other: &'a Scope) -> Self {
        let mut scope = Scope {
            var_store: HashMap::new(),
            outter_scope: Some(other),
        };
        for (k, v) in other.var_store.iter() {
            scope.set_var(k.to_string(), *v);
        }
        return scope;
    }

    pub fn assign(&mut self, kv: HashMap<&String, &StackValue>) {
        for kv in kv.iter() {
            self.set_var(kv.0.to_string(), **kv.1);
        }
    }
}
