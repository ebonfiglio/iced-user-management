use crate::domain::{Job, Organization};
use iced::Theme;
use sqlx::SqlitePool;

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
    ThemeChanged(Theme),
    DatabaseInitialized(SqlitePool),
    DatabaseError(String),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Page {
    User,
    Organization,
    Job,
    Settings,
}
