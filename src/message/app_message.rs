use crate::domain::services::{JobService, UserService};
use iced::Theme;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Page {
    User,
    Organization,
    Job,
    Settings,
}

#[derive(Debug, Clone)]
pub enum AppMessage {
    Navigate(Page),
    CancelEdit,
    ThemeChanged(Theme),
    Initialized(UserService, JobService),
    InitializationError(String),
}
