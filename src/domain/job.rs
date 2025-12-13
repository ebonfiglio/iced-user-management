use super::Entity;

#[derive(Default, Clone, Debug, PartialEq)]
pub struct Job {
    id: usize,
    name: String,
}

impl Job {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Entity for Job {
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

    fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.name.trim().is_empty() {
            errors.push("Job title is required".to_string());
        } else if self.name.len() < 2 {
            errors.push("Job title must be at least 2 characters".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl std::fmt::Display for Job {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
