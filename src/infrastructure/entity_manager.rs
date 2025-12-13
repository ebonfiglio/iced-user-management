use crate::domain::Entity;

#[derive(Debug, Clone)]
pub struct EntityManager<T: Entity> {
    pub current: T,
    pub list: Vec<T>,
    pub is_edit: bool,
    pub validation_errors: Vec<String>,
}

impl<T: Entity> EntityManager<T> {
    pub fn new() -> Self {
        Self {
            current: T::default(),
            list: Vec::new(),
            is_edit: false,
            validation_errors: Vec::new(),
        }
    }

    pub fn create(&mut self) -> Result<(), Vec<String>> {
        match self.current.validate() {
            Ok(()) => {
                self.current.set_id(self.list.len() + 1);
                self.list.push(std::mem::take(&mut self.current));
                self.is_edit = false;
                Ok(())
            }
            Err(errors) => {
                self.validation_errors = errors.clone();
                Err(errors)
            }
        }
    }

    pub fn update(&mut self) -> Result<(), Vec<String>> {
        match self.current.validate() {
            Ok(()) => {
                if let Some(index) = self.list.iter().position(|e| e.id() == self.current.id()) {
                    self.list[index] = std::mem::take(&mut self.current);
                    self.is_edit = false;
                    Ok(())
                } else {
                    Err(vec!["Entity not found".to_string()])
                }
            }
            Err(errors) => {
                self.validation_errors = errors.clone();
                Err(errors)
            }
        }
    }

    pub fn delete(&mut self, id: usize) -> Result<(), Vec<String>> {
        if let Some(index) = self.list.iter().position(|e| e.id() == id) {
            self.list.remove(index);
            Ok(())
        } else {
            Err(vec!["Entity not found".to_string()])
        }
    }

    pub fn load(&mut self, id: usize) -> Result<(), Vec<String>> {
        if let Some(entity) = self.list.iter().find(|e| e.id() == id).cloned() {
            self.current = entity;
            self.is_edit = true;
            self.validation_errors.clear();
            Ok(())
        } else {
            Err(vec!["Entity not found".to_string()])
        }
    }

    pub fn name_changed(&mut self, name: String) {
        self.current.set_name(name);
        self.validation_errors.clear();
    }

    pub fn cancel_edit(&mut self) {
        self.current = T::default();
        self.is_edit = false;
        self.validation_errors.clear();
    }
}
