use crate::domain::Entity;

#[derive(Debug, Clone)]
pub struct EntityState<T: Entity> {
    current: T,
    list: Vec<T>,
    is_edit: bool,
}

impl<T: Entity> EntityState<T> {
    pub fn new() -> Self {
        Self {
            current: T::default(),
            list: Vec::new(),
            is_edit: false,
        }
    }

    pub fn current(&self) -> &T {
        &self.current
    }

    pub fn current_mut(&mut self) -> &mut T {
        &mut self.current
    }

    pub fn set_current(&mut self, current: T) {
        self.current = current;
    }

    pub fn list(&self) -> &Vec<T> {
        &self.list
    }

    pub fn set_list(&mut self, list: Vec<T>) {
        self.list = list;
    }

    pub fn is_edit(&self) -> bool {
        self.is_edit
    }

    pub fn set_is_edit(&mut self, is_edit: bool) {
        self.is_edit = is_edit;
    }

    pub fn clear_entity_state(&mut self) {
        self.current = T::default();
        self.is_edit = false;
        self.current.clear_errors();
    }
}
