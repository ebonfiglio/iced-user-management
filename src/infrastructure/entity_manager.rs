use crate::domain::Entity;

#[derive(Debug, Clone)]
pub struct EntityManager<T: Entity> {
    pub current: T,
    pub list: Vec<T>,
    pub is_edit: bool,
}

impl<T: Entity> EntityManager<T> {
    pub fn new() -> Self {
        Self {
            current: T::default(),
            list: Vec::new(),
            is_edit: false,
        }
    }

    pub fn create(&mut self) -> Result<(), ()> {
        self.current.validate();
        self.current.set_id(self.list.len() + 1);
        self.list.push(std::mem::take(&mut self.current));
        self.is_edit = false;
        Ok(())
    }

    pub fn update(&mut self) -> Result<(), ()> {
        self.current.validate();
        if let Some(index) = self.list.iter().position(|e| e.id() == self.current.id()) {
            self.list[index] = std::mem::take(&mut self.current);
            self.is_edit = false;
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn delete(&mut self, id: usize) -> Result<(), ()> {
        if let Some(index) = self.list.iter().position(|e| e.id() == id) {
            self.list.remove(index);
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn load(&mut self, id: usize) -> Result<(), ()> {
        if let Some(entity) = self.list.iter().find(|e| e.id() == id).cloned() {
            self.current = entity;
            self.is_edit = true;
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn name_changed(&mut self, name: String) {
        self.current.set_name(name);
        self.current.validate_property("name");
    }

    pub fn cancel_edit(&mut self) {
        self.current = T::default();
        self.is_edit = false;
        self.current.clear_errors();
    }
}
