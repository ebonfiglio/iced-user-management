use crate::domain::{Job, Organization};

#[derive(Debug, Clone)]
pub enum Message {
    Navigate(Page),
    NameChanged(String),
    JobSelected(Job),
    OrganizationSelected(Organization),
    JobClicked(usize),
    OrganizationClicked(usize),
    Create,
    Update,
    Delete(usize),
    Load(usize),
    CancelEdit,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Page {
    User,
    Organization,
    Job,
}
