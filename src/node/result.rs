use crate::phrase::PhraseResult;

use std::collections::HashMap;

pub struct NodeResult {
    confidence: f32,
    fields: HashMap<String, Vec<String>>,
    length: usize,
    task: String,
}

impl NodeResult {
    pub fn new(task: &String, confidence: f32, length: usize) -> NodeResult {
        NodeResult {
            confidence: confidence,
            fields: HashMap::new(),
            length: length,
            task: task.clone(),
        }
    }

    pub fn new_from_phrase(task: &String, result: &PhraseResult) -> NodeResult {
        NodeResult {
            confidence: result.get_confidence(),
            fields:     result.get_fields().clone(),
            length:     result.len(),
            task:       task.clone(),
        }
    }

    pub fn get_task(&self) -> &String {
        &self.task
    }

    pub fn get_confidence(&self) -> f32 {
        self.confidence
    }

    pub fn get_fields(&self) -> &HashMap<String, Vec<String>> {
        &self.fields
    }

    pub fn len(&self) -> usize {
        self.length
    }

    // Gives a score based off of the length and confidence
    pub fn get_score(&self) -> f32 {
        (self.length as f32) * self.confidence
    }

    pub fn add_fields(&mut self, fields: &HashMap<String, Vec<String>>) {
        for (field, values) in fields {
            match self.fields.get_mut(field) {
                Some(v) => { v.extend(values.clone()); },
                None    => { self.fields.insert(field.clone(), values.clone()); },
            }
        }
    }

    pub fn add_length(&mut self, length: usize) {
        self.length += length;
    }

    pub fn mult_confidence(&mut self, confidence: f32) {
        self.confidence *= confidence;
    }
}