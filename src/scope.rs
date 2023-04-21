use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::instruction::StackValue;

#[derive(Debug, Clone)]
pub struct Scope {
    var_store: HashMap<String, StackValue>,
    outter_scope: Option<Rc<RefCell<Scope>>>,
}

impl Scope {
    pub fn new() -> Self {
        Scope {
            var_store: HashMap::new(),
            outter_scope: None,
        }
    }

    pub fn get_var_store_len(&self) -> usize {
        self.var_store.len()
    }

    pub fn from_scope(outer_scope: Rc<RefCell<Scope>>) -> Self {
        let scope = Scope {
            var_store: HashMap::new(),
            outter_scope: Some(outer_scope),
        };
        return scope;
    }

    pub fn dec_var(&mut self, id: String, val: StackValue) -> Option<StackValue> {
        self.var_store.insert(id, val)
    }

    pub fn set_var(&mut self, id: String, val: StackValue) -> Option<StackValue> {
        let var = self.var_store.get(&id.clone());
        match var {
            Some(..) => {
                self.var_store.insert(id, val.clone());
                Some(val)
            }
            None => {
                if let Some(out) = &mut self.outter_scope {
                    return out.borrow_mut().set_var(id, val);
                } else {
                    return None;
                }
            }
        }
    }

    pub fn get_var(&self, id: String) -> Option<StackValue> {
        let var = self.var_store.get(&id.clone());
        match var {
            Some(x) => {
                return Some(x.clone());
            }
            None => {
                if let Some(out) = self.outter_scope.clone() {
                    out.borrow().get_var(id)
                } else {
                    None
                }
            }
        }
    }
}
