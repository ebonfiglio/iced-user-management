mod app;
mod domain;
mod infrastructure;
mod message;
mod view;

use app::AppState;
use message::Message;

pub fn main() -> iced::Result {
    iced::application(AppState::new, AppState::update, AppState::view)
        .theme(AppState::theme)
        .title("User Management")
        .run()
}
