use std::collections::HashMap;

use super::Entity;

#[derive(Default, Clone, Debug, PartialEq)]
pub struct Organization {
    id: usize,
    name: String,
    errors: HashMap<&'static str, &'static str>,
}

impl Organization {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Entity for Organization {
    fn id(&self) -> usize {
        self.id
    }

    fn set_id(&mut self, id: usize) {
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
}

impl std::fmt::Display for Organization {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
