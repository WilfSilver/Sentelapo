use crate::phrase::Phrase;
use crate::phrase::PhraseResult;

use std::rc::Rc;
use std::cell::RefCell;

pub struct GroupPhrase {
    name:    String,
    phrases: Vec<Rc<RefCell<dyn Phrase>>>,
}

impl GroupPhrase {
    pub fn new(name: &String, phrases: Vec<Rc<RefCell<dyn Phrase>>>) -> GroupPhrase {
        GroupPhrase {
            name:    name.clone(),
            phrases: phrases,
        }
    }

    pub fn push(&mut self, phrase: Rc<RefCell<dyn Phrase>>) {
        self.phrases.push(phrase.clone());
    }
}

impl Phrase for GroupPhrase {
    fn matches(&self, phrase: &[&str]) -> PhraseResult {
        let mut result = PhraseResult::new(0.0, 0);

        for p in &self.phrases {
            let r = p.borrow().matches(phrase);
            if r.len() != 0 && r.get_score() > result.get_score() {
                result = r;
            }
        }

        return result;
    }

    fn get_name(&self) -> &String { &self.name }
}