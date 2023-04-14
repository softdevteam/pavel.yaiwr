use std::{cell::RefCell, collections::HashMap};

use crate::{err::InterpError, instruction::StackValue};

#[derive(Debug)]
pub struct Scope<'a> {
    var_store: HashMap<String, StackValue>,
    outter_scope: Option<&'a RefCell<Box<Scope<'a>>>>,
}

impl<'a> Scope<'a> {
    pub fn new() -> Self {
        Scope {
            var_store: HashMap::new(),
            outter_scope: None,
        }
    }
    pub fn from_scope(other: &'a RefCell<Box<Scope<'a>>>) -> Self {
        let scope = Scope {
            var_store: HashMap::new(),
            outter_scope: Some(other),
        };
        return scope;
    }

    pub fn dec_var(&mut self, id: String) {
        self.var_store.insert(id, StackValue::Uninitialised);
    }

    pub fn set_var(&mut self, id: String, val: StackValue) {
        let var = self.var_store.get(&id.clone());
        match var {
            Some(..) => {
                self.var_store.insert(id, val);
            }
            None => {
                if let Some(out) = &self.outter_scope {
                    out.borrow_mut().set_var(id, val);
                }
            }
        }
    }

    pub fn assign(&mut self, kv: HashMap<&String, &StackValue>) {
        for kv in kv.iter() {
            self.set_var(kv.0.to_string(), **kv.1);
        }
    }

    pub fn get_var(&self, id: &String) -> Result<StackValue, InterpError> {
        let var = self.var_store.get(&id.clone());
        match var {
            Some(..) => Ok(*self.var_store.get(id).unwrap()),
            None => {
                if let Some(out) = &self.outter_scope {
                    let val = out.borrow_mut().get_var(id).unwrap();
                    return Ok(val);
                } else {
                    Err(InterpError::VariableNotFound(id.to_string()))
                }
            }
        }
    }
}
