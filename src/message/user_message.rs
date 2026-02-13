use crate::domain::{Job, Organization, User};

#[derive(Debug, Clone)]
pub enum UserMessage {
    GetAll,
    SetAll(Vec<User>),
    NameChanged(String),
    JobSelected(Job),
    OrganizationSelected(Organization),
    Create,
    CreateSuccess,
    Update,
    UpdateSuccess,
    Delete(i64),
    DeleteSuccess,
    Load(i64),
    Loaded(User),
    NotFound,
    LoadError(String),
}
