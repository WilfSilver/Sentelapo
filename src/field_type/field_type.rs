pub trait FieldType {
    fn get_value(&self, input: &[&str]) -> (String, usize, f32); // TODO: Should return possibilities
}