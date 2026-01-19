use crate::domain::Job;

#[derive(Debug, Clone)]
pub enum JobMessage {
    Clicked(i64),
    GetAll,
    SetAll(Vec<Job>),
    NameChanged(String),
    Create,
    CreateSuccess,
    Update,
    Delete(i64),
    Load(i64),
    Loaded(Job),
    NotFound,
    LoadError(String),
}
