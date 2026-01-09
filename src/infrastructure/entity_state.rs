use crate::domain::Entity;

#[derive(Debug, Clone)]
pub struct EntityState<T: Entity> {
    pub current: T,
    pub list: Vec<T>,
    pub is_edit: bool,
}

impl<T: Entity> EntityState<T> {
    pub fn new() -> Self {
        Self {
            current: T::default(),
            list: Vec::new(),
            is_edit: false,
        }
    }

    pub fn cancel_edit(&mut self) {
        self.current = T::default();
        self.is_edit = false;
        self.current.clear_errors();
    }
}
