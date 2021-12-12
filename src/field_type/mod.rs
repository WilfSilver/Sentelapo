mod first_name;
mod last_name;
mod custom;
mod field_type;

pub use crate::field_type::field_type::FieldType;
pub use crate::field_type::custom::Custom;
pub use crate::field_type::first_name::FirstName;
pub use crate::field_type::last_name::LastName;

use std::rc::Rc;
use std::cell::RefCell;

pub fn get_default_type(name: &str) -> Option<Rc<RefCell<dyn FieldType>>> {
    match name {
        "FIRST_NAME" => Some(Rc::new(RefCell::new(FirstName::new()))),
        "LAST_NAME"  => Some(Rc::new(RefCell::new(LastName::new()))),
        _            => None,
    }
}