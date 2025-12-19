mod app;
mod domain;
mod infrastructure;
mod message;
mod view;

use app::AppState;
use message::Message;

pub fn main() -> iced::Result {
    iced::application(AppState::new, AppState::update, AppState::view)
        .theme(|state: &AppState| state.theme.clone())
        .title("User Management")
        .run()
}
