use std::collections::HashMap;

pub struct PhraseResult {
    confidence: f32,
    fields:     HashMap<String, Vec<String>>,
    length:     usize,
}

impl PhraseResult {
    pub fn new(confidence: f32, length: usize) -> PhraseResult {
        PhraseResult {
            confidence: confidence,
            fields: HashMap::new(),
            length: length,
        }
    }

    pub fn add_values(&mut self, name: &String, values: &Vec<String>) {
        match self.fields.get_mut(name) {
            Some(v) => { v.extend(values.clone()); },
            None    => { self.fields.insert(name.clone(), values.clone()); },
        }
    }

    pub fn get_confidence(&self) -> f32 {
        self.confidence
    }

    pub fn get_score(&self) -> f32 {
        self.confidence * (self.length as f32)
    }

    pub fn get_fields(&self) -> &HashMap<String, Vec<String>> {
        &self.fields
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn merge(&mut self, o: &Self) {
        self.confidence *= o.get_confidence();
        self.length += o.len();

        for (field, values) in o.get_fields() {
            self.add_values(field, values);
        }
    }
}