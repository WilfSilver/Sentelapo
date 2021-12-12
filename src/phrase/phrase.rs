use crate::phrase::PhraseResult;

pub trait Phrase {
    // Returns if the phrase matches with a given one, returning the length and confidence
    fn matches(&self, phrase: &[&str]) -> PhraseResult;
    // Returns the name of phrase
    fn get_name(&self) -> &String;
}