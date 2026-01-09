use std::collections::HashMap;

use super::Entity;

#[derive(Debug, Default, Clone)]
pub struct User {
    id: i64,
    name: String,
    job_id: i64,
    organization_id: i64,
    errors: HashMap<&'static str, &'static str>,
}

impl User {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_job_id(&mut self, job_id: i64) {
        self.job_id = job_id;
    }

    pub fn set_organization_id(&mut self, organization_id: i64) {
        self.organization_id = organization_id;
    }

    pub fn job_id(&self) -> i64 {
        self.job_id
    }

    pub fn organization_id(&self) -> i64 {
        self.organization_id
    }
}

impl Entity for User {
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

        if self.job_id == 0 {
            self.errors.insert("job_id", "Job selection is required");
        }

        if self.organization_id == 0 {
            self.errors
                .insert("organization_id", "Organization selection is required");
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
            "job_id" => {
                if self.job_id == 0 {
                    self.errors.insert("job_id", "Job selection is required");
                }
            }
            "organization_id" => {
                if self.organization_id == 0 {
                    self.errors
                        .insert("organization_id", "Organization selection is required");
                }
            }
            _ => {}
        }
    }
    fn clear_errors(&mut self) {
        self.errors.clear();
    }
}
