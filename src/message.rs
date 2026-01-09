use crate::domain::{Job, Organization};
use iced::Theme;
use sqlx::SqlitePool;

#[derive(Debug, Clone)]
pub enum Message {
    Navigate(Page),
    CancelEdit,
    ThemeChanged(Theme),
    DatabaseInitialized(SqlitePool),
    DatabaseError(String),

    JobClicked(i64),
    OrganizationClicked(i64),

    UserNameChanged(String),
    UserJobSelected(Job),
    UserOrganizationSelected(Organization),
    UserCreate,
    UserUpdate,
    UserDelete(i64),
    UserLoad(i64),

    JobNameChanged(String),
    JobOrganizationSelected(Organization),
    JobCreate,
    JobUpdate,
    JobDelete(i64),
    JobLoad(i64),

    OrganizationNameChanged(String),
    OrganizationCreate,
    OrganizationUpdate,
    OrganizationDelete(i64),
    OrganizationLoad(i64),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Page {
    User,
    Organization,
    Job,
    Settings,
}
