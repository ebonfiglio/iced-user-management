use std::collections::HashMap;

use super::Entity;

#[derive(Default, Clone, Debug, PartialEq)]
pub struct Organization {
    id: i64,
    name: String,
    errors: HashMap<&'static str, &'static str>,
}

impl Organization {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Entity for Organization {
    fn id(&self) -> i64 {
        self.id
    }

    fn set_id(&mut self, id: i64) {
        self.id = id;
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn errors(&self) -> &HashMap<&'static str, &'static str> {
        &self.errors
    }

    fn validate(&mut self) -> Result<(), &HashMap<&'static str, &'static str>> {
        self.errors.clear();
        if self.name.trim().is_empty() {
            self.errors.insert("name", "Name is required");
        } else if self.name.len() < 3 {
            self.errors
                .insert("name", "Name must be at least 3 characters");
        } else if self.name.len() > 50 {
            self.errors
                .insert("name", "Name must be under 50 characters");
        }

        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(&self.errors)
        }
    }

    fn validate_property(&mut self, propery: &str) {
        match propery {
            "name" => {
                self.errors.remove("name");
                if self.name.trim().is_empty() {
                    self.errors.insert("name", "Name is required");
                } else if self.name.len() < 3 {
                    self.errors
                        .insert("name", "Name must be at least 3 characters");
                } else if self.name.len() > 50 {
                    self.errors
                        .insert("name", "Name must be under 50 characters");
                }
            }
            _ => {}
        }
    }
    fn clear_errors(&mut self) {
        self.errors.clear();
    }
}

impl std::fmt::Display for Organization {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
