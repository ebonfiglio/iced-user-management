mod app;
mod domain;
mod infrastructure;
mod message;
mod view;

use app::AppState;

pub fn main() -> iced::Result {
    iced::application(AppState::new, AppState::update, AppState::view)
        .theme(|state: &AppState| state.get_theme().clone())
        .title("User Management")
        .run()
}
