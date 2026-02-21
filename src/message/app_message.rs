use crate::domain::services::{JobService, OrganizationService, UserService};
use crate::page::Page;
use iced::Theme;

#[derive(Debug, Clone)]
pub enum AppMessage {
    Navigate(Page),
    CancelEdit,
    ThemeChanged(Theme),
    Initialized(UserService, JobService, OrganizationService),
    InitializationError(String),
    SetStatusMessage(String),
}
