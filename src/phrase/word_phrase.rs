use crate::phrase::Phrase;
use crate::phrase::PhraseResult;

pub struct WordPhrase {
    name:   String,
    phrase: Vec<String>,
}

impl WordPhrase {
    pub fn new(name: &String, phrase: String) -> WordPhrase {
        WordPhrase {
            name:   name.clone(),
            phrase: phrase.split(' ').map(|x| x.to_lowercase()).collect(),
        }
    }
}

impl Phrase for WordPhrase {
    fn matches(&self, phrase: &[&str]) -> PhraseResult {
        // TODO: Add confidence scale
        if self.phrase.len() > phrase.len() {
            return PhraseResult::new(0.0, 0);
        }

        for i in 0..self.phrase.len() {
            if self.phrase[i].ne(&phrase[i].to_lowercase()) {
                return PhraseResult::new(0.0, 0);
            }
        }

        return PhraseResult::new(1.0, self.phrase.len());
    }

    fn get_name(&self) -> &String {
        &self.name
    }
}