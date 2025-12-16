use std::collections::HashMap;

pub trait Entity: Clone + Default + std::fmt::Debug {
    fn id(&self) -> usize;
    fn set_id(&mut self, id: usize);
    fn name(&self) -> &str;
    fn set_name(&mut self, name: String);
    fn errors(&self) -> &HashMap<&'static str, &'static str>;
    fn validate(&mut self) -> Result<(), &HashMap<&'static str, &'static str>>;
    fn validate_property(&mut self, propery: &str);
    fn clear_errors(&mut self);
}
