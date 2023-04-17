use std::{collections::HashMap};

use crate::{instruction::StackValue, err::InterpError};

#[derive(Debug, Clone)]
pub struct Scope {
    pub var_store: HashMap<String, StackValue>,
    pub outter_scope: Option<Box<Scope>>
}

impl Scope {
    pub fn new() -> Self {
        Scope {
            var_store: HashMap::new(),
            outter_scope: None,
        }
    }

    pub fn from_scope(other: Box<Scope>) -> Self {
        let scope = Scope {
            var_store: HashMap::new(),
            outter_scope: Some(other),
        };
        return scope;
    }

    pub fn dec_var(&mut self, id: String) -> Option<StackValue> {
        self.var_store.insert(id, StackValue::Uninitialised)
    }

    pub fn set_var(&mut self, id: String, val: StackValue) {
        let var = self.var_store.get(&id.clone());
        match var {
            Some(..) => {
                self.var_store.insert(id, val);
            }
            None => {
                if let Some(out) = &mut self.outter_scope {
                    out.set_var(id, val);
                }
            }
        }
    }
    pub fn assign(&mut self, kv: HashMap<&String, &StackValue>) {
        for kv in kv.iter() {
            self.dec_var(kv.0.to_string());
            self.set_var(kv.0.to_string(), **kv.1);
        }
    }

    pub fn get_var(self, id: String) -> Result<StackValue, InterpError>{
        let var = self.var_store.get(&id.clone());
        match var {
            Some(x) => {
                return Ok(*x);
            }
            None => {
                if let Some(out) = self.outter_scope {
                    out.get_var(id)
                }else{
                    return Err(InterpError::VariableNotFound(id.to_string()));
                }
            }
        }
    }
}
