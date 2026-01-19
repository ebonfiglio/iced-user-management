use crate::domain::{Job, Organization, User};

#[derive(Debug, Clone)]
pub enum UserMessage {
    NameChanged(String),
    JobSelected(Job),
    OrganizationSelected(Organization),
    Create,
    Update,
    Delete(i64),
    Load(i64),
    Loaded(User),
    NotFound,
    LoadError(String),
}
