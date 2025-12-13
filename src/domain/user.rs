use super::Entity;

#[derive(Debug, Default, Clone)]
pub struct User {
    id: usize,
    name: String,
    job_id: usize,
    organization_id: usize,
}

impl User {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_job_id(&mut self, job_id: usize) {
        self.job_id = job_id;
    }

    pub fn set_organization_id(&mut self, organization_id: usize) {
        self.organization_id = organization_id;
    }

    pub fn job_id(&self) -> usize {
        self.job_id
    }

    pub fn organization_id(&self) -> usize {
        self.organization_id
    }

    pub fn validate_name_field(name: &str) -> Option<String> {
        if name.trim().is_empty() {
            Some("Name is required".to_string())
        } else if name.len() < 3 {
            Some("Name must be at least 3 characters".to_string())
        } else if name.len() > 50 {
            Some("Name cannot exceed 50 characters".to_string())
        } else {
            None
        }
    }

    pub fn validate_job_field(job_id: usize) -> Option<String> {
        if job_id == 0 {
            Some("Job selection is required".to_string())
        } else {
            None
        }
    }

    pub fn validate_organization_field(organization_id: usize) -> Option<String> {
        if organization_id == 0 {
            Some("Organization selection is required".to_string())
        } else {
            None
        }
    }

    pub fn validate_all_fields(&self) -> UserValidation {
        UserValidation {
            name_error: Self::validate_name_field(&self.name),
            job_error: Self::validate_job_field(self.job_id),
            organization_error: Self::validate_organization_field(self.organization_id),
        }
    }
}

impl Entity for User {
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
        let validation = self.validate_all_fields();

        if validation.has_errors() {
            Err(validation.get_all_errors())
        } else {
            Ok(())
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct UserValidation {
    pub name_error: Option<String>,
    pub job_error: Option<String>,
    pub organization_error: Option<String>,
}

impl UserValidation {
    pub fn clear(&mut self) {
        self.name_error = None;
        self.job_error = None;
        self.organization_error = None;
    }

    pub fn has_errors(&self) -> bool {
        self.name_error.is_some() || self.job_error.is_some() || self.organization_error.is_some()
    }

    pub fn get_all_errors(&self) -> Vec<String> {
        let mut errors = Vec::new();
        if let Some(e) = &self.name_error {
            errors.push(e.clone());
        }
        if let Some(e) = &self.job_error {
            errors.push(e.clone());
        }
        if let Some(e) = &self.organization_error {
            errors.push(e.clone());
        }
        errors
    }
}
