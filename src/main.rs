use iced::{
    Element, Font, Length, Task, Theme,
    widget::{button, column, container, row, scrollable, text, text_input},
};

pub fn main() -> iced::Result {
    iced::application("Iced Editor", UserManager::update, UserManager::view)
        .theme(UserManager::theme)
        .default_font(Font::MONOSPACE)
        .run_with(UserManager::new)
}

#[derive(Default)]
struct User {
    name: String,
}

impl User {
    fn new() -> User {
        User {
            name: String::new(),
        }
    }
}

struct UserManager {
    current_user: User,
    user_list: Vec<User>,
}

impl UserManager {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                current_user: User::new(),
                user_list: Vec::new(),
            },
            Task::none(),
        )
    }
}

#[derive(Debug, Clone)]
enum Message {
    EditName(String),
    Save,
    Delete(usize),
}

impl UserManager {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Save => {
                self.user_list.push(std::mem::take(&mut self.current_user));
                Task::none()
            }
            Message::EditName(name) => {
                self.current_user.name = name;
                Task::none()
            }
            Message::Delete(index) => {
                self.user_list.remove(index);
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let input = text_input("User", &self.current_user.name).on_input(Message::EditName);
        let save_btn = row![button("Save").on_press(Message::Save)];

        let user_list = scrollable(self.user_list.iter().enumerate().fold(
            column![].spacing(2),
            |col, (index, user)| {
                col.push(
                    row![
                        text(&user.name).width(Length::FillPortion(1)),
                        button("Delete").on_press(Message::Delete(index)),
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

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
