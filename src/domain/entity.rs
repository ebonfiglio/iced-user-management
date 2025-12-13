pub trait Entity: Clone + Default + std::fmt::Debug {
    fn id(&self) -> usize;
    fn set_id(&mut self, id: usize);
    fn name(&self) -> &str;
    fn set_name(&mut self, name: String);

    fn validate(&self) -> Result<(), Vec<String>> {
        Ok(())
    }
}
