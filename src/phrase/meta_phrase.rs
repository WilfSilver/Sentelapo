use crate::phrase::Phrase;
use crate::phrase::PhraseResult;

use std::cell::RefCell;
use std::rc::Rc;

pub struct MetaPhrase {
    name:    String,
    phrases: Vec<Rc<RefCell<dyn Phrase>>>,
}

impl MetaPhrase {
    pub fn new(name: &String, phrases: Vec<Rc<RefCell<dyn Phrase>>>) -> MetaPhrase {
        MetaPhrase {
            name: name.clone(),
            phrases: phrases,
        }
    }
}

impl Phrase for MetaPhrase {
    fn matches(&self, phrase: &[&str]) -> PhraseResult {
        let mut result = PhraseResult::new(1.0, 0);

        let mut next_phrase = phrase;
        for p in &self.phrases {
            let r = p.borrow().matches(next_phrase);

            if r.len() == 0 { return PhraseResult::new(0.0, 0); }

            result.merge(&r);

            next_phrase = &phrase[r.len()..];
        }

        return result;
    }

    fn get_name(&self) -> &String { &self.name }
}