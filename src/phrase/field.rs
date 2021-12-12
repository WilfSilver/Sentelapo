use crate::field_type::FieldType;

use crate::phrase::Phrase;
use crate::phrase::PhraseResult;

use std::cell::RefCell;
use std::rc::Rc;

pub struct Field {
    name: String,
    field_type: Rc<RefCell<dyn FieldType>>,
}

impl Field {
    pub fn new(name: &String, field_type: Rc<RefCell<dyn FieldType>>) -> Field {
        Field {
            name: name.clone(),
            field_type: field_type,
        }
    }
}

impl Phrase for Field {
    fn matches(&self, phrase: &[&str]) -> PhraseResult {
        let (value, l, c) = self.field_type.borrow().get_value(phrase);

        let mut result = PhraseResult::new(c, l);

        result.add_values(&self.name, &vec![value]);

        return result;
    }

    fn get_name(&self) -> &String { &self.name }
}