use iced::{
    Border, Color, Element,
    Length::{self, Fill, FillPortion},
    Task, Theme,
    widget::{Container, Row, button, column, container, row, scrollable, text, text_input},
};

pub fn main() -> iced::Result {
    iced::application(AppState::new, AppState::update, AppState::view)
        .theme(AppState::theme)
        .title("User Management")
        .run()
}

struct AppState {
    current_page: Page,
    users: EntityManager<User>,
    organizations: EntityManager<Organization>,
    jobs: EntityManager<Job>,
}

impl AppState {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                current_page: Page::User,
                users: EntityManager::new(),
                organizations: EntityManager::new(),
                jobs: EntityManager::new(),
            },
            Task::none(),
        )
    }
}

trait Entity: Clone + Default + std::fmt::Debug {
    fn id(&self) -> usize;
    fn set_id(&mut self, id: usize);
    fn name(&self) -> &str;
    fn set_name(&mut self, name: String);
}

#[derive(Debug, Clone)]
struct EntityManager<T: Entity> {
    current: T,
    list: Vec<T>,
    is_edit: bool,
}

impl<T: Entity> EntityManager<T> {
    fn new() -> Self {
        Self {
            current: T::default(),
            list: Vec::new(),
            is_edit: false,
        }
    }

    fn create(&mut self) -> Result<(), String> {
        self.current.set_id(self.list.len() + 1);
        self.list.push(std::mem::take(&mut self.current));
        self.is_edit = false;
        Ok(())
    }

    fn update(&mut self) -> Result<(), String> {
        if let Some(index) = self.list.iter().position(|e| e.id() == self.current.id()) {
            self.list[index] = std::mem::take(&mut self.current);
            self.is_edit = false;
            Ok(())
        } else {
            Err("Entity not found".to_string())
        }
    }

    fn delete(&mut self, id: usize) -> Result<(), String> {
        if let Some(index) = self.list.iter().position(|e| e.id() == id) {
            self.list.remove(index);
            Ok(())
        } else {
            Err("Entity not found".to_string())
        }
    }

    fn load(&mut self, id: usize) -> Result<(), String> {
        if let Some(entity) = self.list.iter().find(|e| e.id() == id).cloned() {
            self.current = entity;
            self.is_edit = true;
            Ok(())
        } else {
            Err("Entity not found".to_string())
        }
    }

    fn name_changed(&mut self, name: String) {
        self.current.set_name(name);
    }

    fn cancel_edit(&mut self) {
        self.current = T::default();
        self.is_edit = false;
    }
}

#[derive(Debug, Default, Clone)]
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

impl Entity for User {
    fn id(&self) -> usize {
        self.id
    }
    fn set_id(&mut self, id: usize) {
        self.id = id;
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

#[derive(Default, Clone, Debug)]
struct Organization {
    id: usize,
    name: String,
}

impl Entity for Organization {
    fn id(&self) -> usize {
        self.id
    }
    fn set_id(&mut self, id: usize) {
        self.id = id;
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

#[derive(Default, Clone, Debug)]
struct Job {
    id: usize,
    name: String,
}

impl Entity for Job {
    fn id(&self) -> usize {
        self.id
    }
    fn set_id(&mut self, id: usize) {
        self.id = id;
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

#[derive(Debug, Clone)]
enum Message {
    Navigate(Page),

    NameChanged(String),
    Create,
    Update,
    Delete(usize),
    Load(usize),
    CancelEdit,
}

#[derive(Debug, Clone)]
enum Page {
    User,
    Organization,
    Job,
}

macro_rules! with_manager {
    ($self:expr, $method:ident $(, $arg:expr)*) => {
        match $self.current_page {
            Page::User => $self.users.$method($($arg),*),
            Page::Organization => $self.organizations.$method($($arg),*),
            Page::Job => $self.jobs.$method($($arg),*),
        }
    };
}

impl AppState {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Navigate(page) => {
                self.current_page = page;
                with_manager!(self, cancel_edit);
            }
            Message::NameChanged(name) => {
                with_manager!(self, name_changed, name);
            }
            Message::Create => {
                with_manager!(self, create);
            }
            Message::Update => {
                with_manager!(self, update);
            }
            Message::Delete(id) => {
                with_manager!(self, delete, id);
            }
            Message::Load(id) => {
                with_manager!(self, load, id);
            }
            Message::CancelEdit => {
                with_manager!(self, cancel_edit);
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let navigation = container(
            column![
                row![
                    button(container("Users").center_x(30).center_y(30))
                        .width(Length::Fill)
                        .on_press(Message::Navigate(Page::User))
                ],
                row![
                    button(container("Organizations").center_x(30).center_y(30))
                        .width(Length::Fill)
                        .on_press(Message::Navigate(Page::Organization))
                ],
                row![
                    button(container("Jobs").center_x(30).center_y(30))
                        .width(Length::Fill)
                        .on_press(Message::Navigate(Page::Job))
                ],
            ]
            .spacing(10)
            .height(Fill),
        )
        .padding(10)
        .style(|_theme: &Theme| container::Style {
            border: Border {
                color: Color::from_rgb(0.5, 0.5, 0.5),
                width: 1.0,
                radius: 0.into(),
            },
            ..Default::default()
        })
        .width(FillPortion(1));

        container(row![navigation, self.entity_form()].spacing(10))
            .padding(10)
            .into()
    }

    fn entity_form(&self) -> Container<'_, Message> {
        match self.current_page {
            Page::Organization => self.organization_form(),
            Page::User => self.user_form(),
            Page::Job => self.job_form(),
        }
    }

    fn job_form(&self) -> Container<'_, Message> {
        let input = text_input("Job", &self.jobs.current.name).on_input(Message::NameChanged);

        let header_row = row![
            text("ID").width(Length::FillPortion(1)),
            text("Name").width(Length::FillPortion(2)),
            text("Action")
        ];
        let job_list = scrollable(self.jobs.list.iter().enumerate().fold(
            column![header_row].spacing(2),
            |col, (_, job)| {
                col.push(
                    row![
                        text(job.id).width(Length::FillPortion(1)),
                        text(&job.name).width(Length::FillPortion(2)),
                        button("Edit")
                            .style(button::primary)
                            .on_press(Message::Load(job.id)),
                        button("Delete")
                            .style(button::danger)
                            .on_press(Message::Delete(job.id)),
                    ]
                    .spacing(10)
                    .padding(5),
                )
            },
        ))
        .height(Length::Fill);

        container(column![input, self.get_form_buttons(self.jobs.is_edit), job_list].spacing(10))
            .width(FillPortion(4))
    }

    fn organization_form(&self) -> Container<'_, Message> {
        let input = text_input("Organization", &self.organizations.current.name)
            .on_input(Message::NameChanged);

        let header_row = row![
            text("ID").width(Length::FillPortion(1)),
            text("Name").width(Length::FillPortion(2)),
            text("Action")
        ];
        let organization_list = scrollable(self.organizations.list.iter().enumerate().fold(
            column![header_row].spacing(2),
            |col, (_, organization)| {
                col.push(
                    row![
                        text(organization.id).width(Length::FillPortion(1)),
                        text(&organization.name).width(Length::FillPortion(2)),
                        button("Edit")
                            .style(button::primary)
                            .on_press(Message::Load(organization.id)),
                        button("Delete")
                            .style(button::danger)
                            .on_press(Message::Delete(organization.id)),
                    ]
                    .spacing(10)
                    .padding(5),
                )
            },
        ))
        .height(Length::Fill);

        container(
            column![
                input,
                self.get_form_buttons(self.organizations.is_edit),
                organization_list
            ]
            .spacing(10),
        )
        .width(FillPortion(4))
    }

    fn user_form(&self) -> Container<'_, Message> {
        let input = text_input("User", &self.users.current.name).on_input(Message::NameChanged);

        let header_row = row![
            text("ID").width(Length::FillPortion(1)),
            text("Name").width(Length::FillPortion(2)),
            text("Action")
        ];
        let user_list = scrollable(self.users.list.iter().enumerate().fold(
            column![header_row].spacing(2),
            |col, (_, user)| {
                col.push(
                    row![
                        text(user.id).width(Length::FillPortion(1)),
                        text(&user.name).width(Length::FillPortion(2)),
                        button("Edit")
                            .style(button::primary)
                            .on_press(Message::Load(user.id)),
                        button("Delete")
                            .style(button::danger)
                            .on_press(Message::Delete(user.id)),
                    ]
                    .spacing(10)
                    .padding(5),
                )
            },
        ))
        .height(Length::Fill);
        container(column![input, self.get_form_buttons(self.users.is_edit), user_list].spacing(10))
            .width(FillPortion(4))
    }

    fn get_form_buttons(&self, is_edit: bool) -> Row<'_, Message> {
        if is_edit {
            row![
                button("Update").on_press(Message::Update),
                button("Cancel")
                    .style(button::danger)
                    .on_press(Message::CancelEdit)
            ]
            .spacing(10)
        } else {
            row![button("Create").on_press(Message::Create)]
        }
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
