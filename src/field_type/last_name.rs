use crate::field_type::FieldType;

pub struct LastName {}

impl LastName {
    pub fn new() -> LastName { LastName {} }
}

impl FieldType for LastName {
    fn get_value(&self, input: &[&str]) -> (String, usize, f32) {
        match input.get(0) {
            Some(w) => {
                let mut confidence = 0.5;
                if w.chars().nth(0).unwrap().is_uppercase() {
                    confidence = 0.9;
                }
                return (w.to_string(), 1, confidence);
            }
            None    => return (String::new(), 0, 0.0),
        }
    }
}