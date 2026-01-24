use crate::domain::Organization;

#[derive(Debug, Clone)]
pub enum OrganizationMessage {
    Clicked(i64),
    GetAll,
    SetAll(Vec<Organization>),
    NameChanged(String),
    Create,
    CreateSuccess,
    Update,
    Delete(i64),
    Load(i64),
    Loaded(Organization),
    NotFound,
    LoadError(String),
}
