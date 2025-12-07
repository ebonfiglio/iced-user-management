use iced::{
    Element, Font, Length, Task, Theme,
    widget::{Button, button, column, container, row, scrollable, text, text_input},
};

pub fn main() -> iced::Result {
    iced::application("Iced Editor", UserManager::update, UserManager::view)
        .theme(UserManager::theme)
        .default_font(Font::MONOSPACE)
        .run_with(UserManager::new)
}

#[derive(Default, Clone)]
struct User {
    id: usize,
    name: String,
}

impl User {
    fn new() -> User {
        User {
            id: 0,
            name: String::new(),
        }
    }
}

struct UserManager {
    current_user: User,
    user_list: Vec<User>,
    is_edit: bool,
}

impl UserManager {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                current_user: User::new(),
                user_list: Vec::new(),
                is_edit: false,
            },
            Task::none(),
        )
    }
}

#[derive(Debug, Clone)]
enum Message {
    NameChanged(String),
    CreateUser,
    UpdateUser,
    DeleteUser(usize),
    LoadUser(usize),
}

impl UserManager {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::CreateUser => {
                self.current_user.id = self.user_list.len() + 1;
                self.user_list.push(std::mem::take(&mut self.current_user));
                Task::none()
            }
            Message::NameChanged(name) => {
                self.current_user.name = name;
                Task::none()
            }
            Message::DeleteUser(id) => {
                if let Some(index) = self.user_list.iter().position(|u| u.id == id) {
                    self.user_list.remove(index);
                }
                Task::none()
            }
            Message::LoadUser(id) => {
                if let Some(user) = self.user_list.iter().find(|u| u.id == id).cloned() {
                    self.current_user = user;
                    self.is_edit = true;
                }
                Task::none()
            }
            Message::UpdateUser => {
                if let Some(index) = self
                    .user_list
                    .iter()
                    .position(|u| u.id == self.current_user.id)
                {
                    self.user_list[index] = std::mem::take(&mut self.current_user);
                }
                self.is_edit = false;
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let input = text_input("User", &self.current_user.name).on_input(Message::NameChanged);
        let save_btn = row![UserManager::get_button(&self)];

        let header_row = row![
            text("ID").width(Length::FillPortion(1)),
            text("Name").width(Length::FillPortion(2)),
            text("Action")
        ];
        let user_list = scrollable(self.user_list.iter().enumerate().fold(
            column![header_row].spacing(2),
            |col, (_, user)| {
                col.push(
                    row![
                        text(user.id).width(Length::FillPortion(1)),
                        text(&user.name).width(Length::FillPortion(2)),
                        button("Edit")
                            .style(button::primary)
                            .on_press(Message::LoadUser(user.id)),
                        button("Delete")
                            .style(button::danger)
                            .on_press(Message::DeleteUser(user.id)),
                    ]
                    .spacing(10)
                    .padding(5),
                )
            },
        ))
        .height(Length::Fill);

        container(column![input, save_btn, user_list].spacing(10))
            .padding(10)
            .into()
    }

    fn get_button(&self) -> Button<'_, Message> {
        if self.is_edit {
            button("Update").on_press(Message::UpdateUser)
        } else {
            button("Create").on_press(Message::CreateUser)
        }
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
