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
    UpdateSuccess,
    Delete(i64),
    DeleteSuccess,
    Load(i64),
    Loaded(Organization),
    NotFound,
    LoadError(String),
}
