use crate::field_type::FieldType;
use crate::phrase::Phrase;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Custom {
    phrases: HashMap<String, Rc<RefCell<dyn Phrase>>>,
}

impl Custom {
    pub fn new() -> Custom {
        Custom {
            phrases: HashMap::new(),
        }
    }

    pub fn insert_synonym(&mut self, name: String, phrases: Rc<RefCell<dyn Phrase>>) {
        self.phrases.insert(name, phrases);
    }
}

impl FieldType for Custom {
    fn get_value(&self, input: &[&str]) -> (String, usize, f32) {

        let mut result = (String::new(), 0, 0.0);

        for (name, phrase) in &self.phrases {
            let r = phrase.borrow().matches(input);

            if r.len() != 0 && r.get_score() > result.2 * (result.1 as f32) {
                result = (name.clone(), r.len(), r.get_confidence());
            }

        }

        return result;
    }
}