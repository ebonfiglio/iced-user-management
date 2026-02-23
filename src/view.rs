use iced::{
    widget::{
        button, column, container, pick_list, row, scrollable, text, text_input, Container, Row,
    },
    Border, Color, Element, Fill, FillPortion, Length, Theme,
};

use crate::app::AppState;
use crate::domain::Entity;
use crate::message::{
    app_message::AppMessage, job_message::JobMessage, organization_message::OrganizationMessage,
    user_message::UserMessage, Message,
};
use crate::page::Page;

impl AppState {
    pub fn view(&self) -> Element<'_, Message> {
        let navigation = container(
            column![
                row![button(container("Users").center_x(30).center_y(30))
                    .width(Length::Fill)
                    .on_press(Message::App(AppMessage::Navigate(Page::User)))],
                row![button(container("Organizations").center_x(30).center_y(30))
                    .width(Length::Fill)
                    .on_press(Message::App(AppMessage::Navigate(Page::Organization)))],
                row![button(container("Jobs").center_x(30).center_y(30))
                    .width(Length::Fill)
                    .on_press(Message::App(AppMessage::Navigate(Page::Job)))],
                row![button(container("Settings").center_x(30).center_y(30))
                    .width(Length::Fill)
                    .on_press(Message::App(AppMessage::Navigate(Page::Settings)))],
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

        let status_bar = container(text(self.status_message().to_string()).size(12))
            .padding(5)
            .width(Length::Fill)
            .style(|theme: &Theme| container::Style {
                text_color: Some(if self.status_message().to_lowercase().contains("error") {
                    theme.palette().danger
                } else if self.status_message().to_lowercase().contains("connected") {
                    theme.palette().success
                } else {
                    theme.palette().text
                }),
                ..Default::default()
            });

        container(column![
            row![navigation, self.set_current_form()].spacing(10),
            row![status_bar]
        ])
        .padding(10)
        .into()
    }

    fn set_current_form(&self) -> Container<'_, Message> {
        match self.current_page() {
            Page::Organization => self.organization_form(),
            Page::User => self.user_form(),
            Page::Job => self.job_form(),
            Page::Settings => self.settings_form(),
        }
    }

    fn job_form(&self) -> Container<'_, Message> {
        let name_input = column![
            text_input("Job", &self.get_job_entity_state().current().name())
                .on_input(|name| Message::Job(JobMessage::NameChanged(name))),
            if let Some(error) = self.get_job_entity_state().current().errors().get("name") {
                text(error.to_string())
                    .size(12)
                    .style(|_theme| text::Style {
                        color: Some(Color::from_rgb(0.8, 0.2, 0.2)),
                    })
            } else {
                text("").height(0)
            }
        ];
        let header_row = row![
            text("ID").width(Length::FillPortion(1)),
            text("Name").width(Length::FillPortion(2)),
            text("Action")
        ];
        let job_list = scrollable(self.get_job_entity_state().list().iter().enumerate().fold(
            column![header_row].spacing(2),
            |col, (_, job)| {
                col.push(
                    row![
                        text(job.id()).width(Length::FillPortion(1)),
                        text(job.name().to_string()).width(Length::FillPortion(2)),
                        button("Edit")
                            .style(button::primary)
                            .on_press(Message::Job(JobMessage::Load(job.id()))),
                        button("Delete")
                            .style(button::danger)
                            .on_press(Message::Job(JobMessage::Delete(job.id()))),
                    ]
                    .spacing(10)
                    .padding(5),
                )
            },
        ))
        .height(Length::Fill);

        container(
            column![
                name_input,
                self.get_form_buttons(self.get_job_entity_state().is_edit()),
                job_list
            ]
            .spacing(10),
        )
        .width(FillPortion(4))
    }

    fn organization_form(&self) -> Container<'_, Message> {
        let name_input = column![
            text_input(
                "Organization",
                &self.get_organization_entity_state().current().name()
            )
            .on_input(|name| Message::Organization(OrganizationMessage::NameChanged(name))),
            if let Some(error) = self
                .get_organization_entity_state()
                .current()
                .errors()
                .get("name")
            {
                text(error.to_string())
                    .size(12)
                    .style(|_theme| text::Style {
                        color: Some(Color::from_rgb(0.8, 0.2, 0.2)),
                    })
            } else {
                text("").height(0)
            }
        ];
        let header_row = row![
            text("ID").width(Length::FillPortion(1)),
            text("Name").width(Length::FillPortion(2)),
            text("Action")
        ];
        let organization_list = scrollable(
            self.get_organization_entity_state()
                .list()
                .iter()
                .enumerate()
                .fold(column![header_row].spacing(2), |col, (_, organization)| {
                    col.push(
                        row![
                            text(organization.id()).width(Length::FillPortion(1)),
                            text(organization.name().to_string()).width(Length::FillPortion(2)),
                            button("Edit")
                                .style(button::primary)
                                .on_press(Message::Organization(OrganizationMessage::Load(
                                    organization.id()
                                ))),
                            button("Delete")
                                .style(button::danger)
                                .on_press(Message::Organization(OrganizationMessage::Delete(
                                    organization.id()
                                ))),
                        ]
                        .spacing(10)
                        .padding(5),
                    )
                }),
        )
        .height(Length::Fill);

        container(
            column![
                name_input,
                self.get_form_buttons(self.get_organization_entity_state().is_edit()),
                organization_list
            ]
            .spacing(10),
        )
        .width(FillPortion(4))
    }

    fn user_form(&self) -> Container<'_, Message> {
        let name_input = column![
            text_input("User", &self.get_user_entity_state().current().name())
                .on_input(|name| Message::User(UserMessage::NameChanged(name))),
            if let Some(error) = self.get_user_entity_state().current().errors().get("name") {
                text(error.to_string())
                    .size(12)
                    .style(|_theme| text::Style {
                        color: Some(Color::from_rgb(0.8, 0.2, 0.2)),
                    })
            } else {
                text("").height(0)
            }
        ];
        let job_input = column![
            pick_list(
                &self.get_job_entity_state().list()[..],
                self.get_job_entity_state()
                    .list()
                    .iter()
                    .find(|j| j.id() == self.get_user_entity_state().current().job_id()),
                |job| Message::User(UserMessage::JobSelected(job)),
            ),
            if let Some(error) = self
                .get_user_entity_state()
                .current()
                .errors()
                .get("job_id")
            {
                text(error.to_string())
                    .size(12)
                    .style(|_theme| text::Style {
                        color: Some(Color::from_rgb(0.8, 0.2, 0.2)),
                    })
            } else {
                text("").height(0)
            }
        ];
        let organization_input = column![
            pick_list(
                &self.get_organization_entity_state().list()[..],
                self.get_organization_entity_state()
                    .list()
                    .iter()
                    .find(|k| k.id() == self.get_user_entity_state().current().organization_id()),
                |org| Message::User(UserMessage::OrganizationSelected(org)),
            ),
            if let Some(error) = self
                .get_user_entity_state()
                .current()
                .errors()
                .get("organization_id")
            {
                text(error.to_string())
                    .size(12)
                    .style(|_theme| text::Style {
                        color: Some(Color::from_rgb(0.8, 0.2, 0.2)),
                    })
            } else {
                text("").height(0)
            }
        ];
        let header_row = row![
            text("ID").width(Length::FillPortion(1)),
            text("Name").width(Length::FillPortion(2)),
            text("Job").width(Length::FillPortion(2)),
            text("Organization").width(Length::FillPortion(2)),
            text("Action").width(Length::FillPortion(2)),
        ];
        let user_list = scrollable(self.get_user_entity_state().list().iter().enumerate().fold(
            column![header_row].spacing(2),
            |col, (_, user)| {
                let job_name = self.get_job_name(user.job_id());
                let organization_name = self.get_organization_name(user.organization_id());

                col.push(
                    row![
                        text(user.id()).width(Length::FillPortion(1)),
                        text(user.name().to_string()).width(Length::FillPortion(2)),
                        button(text(job_name))
                            .style(button::text)
                            .on_press(Message::Job(JobMessage::Clicked(user.job_id())))
                            .width(Length::FillPortion(2)),
                        button(text(organization_name))
                            .style(button::text)
                            .on_press(Message::Organization(OrganizationMessage::Clicked(
                                user.organization_id()
                            )))
                            .width(Length::FillPortion(2)),
                        button("Edit")
                            .style(button::primary)
                            .on_press(Message::User(UserMessage::Load(user.id())))
                            .width(Length::FillPortion(1)),
                        button("Delete")
                            .style(button::danger)
                            .on_press(Message::User(UserMessage::Delete(user.id())))
                            .width(Length::FillPortion(1)),
                    ]
                    .spacing(10)
                    .padding(5),
                )
            },
        ))
        .height(Length::Fill);
        container(
            column![
                name_input,
                job_input,
                organization_input,
                self.get_form_buttons(self.get_user_entity_state().is_edit()),
                user_list
            ]
            .spacing(10),
        )
        .width(FillPortion(4))
    }

    fn get_form_buttons(&self, is_edit: bool) -> Row<'_, Message> {
        let update_button = button("Update");
        if is_edit {
            let update_button_with_action = match self.current_page() {
                Page::User => update_button.on_press(Message::User(UserMessage::Update)),
                Page::Organization => {
                    update_button.on_press(Message::Organization(OrganizationMessage::Update))
                }
                Page::Job => update_button.on_press(Message::Job(JobMessage::Update)),
                Page::Settings => update_button,
            };
            row![
                update_button_with_action,
                button("Cancel")
                    .style(button::danger)
                    .on_press(Message::App(AppMessage::CancelEdit))
            ]
            .spacing(10)
        } else {
            let create_button = match self.current_page() {
                Page::User => row![button("Create").on_press(Message::User(UserMessage::Create))],
                Page::Organization => {
                    row![button("Create")
                        .on_press(Message::Organization(OrganizationMessage::Create))]
                }
                Page::Job => row![button("Create").on_press(Message::Job(JobMessage::Create))],
                Page::Settings => row![button("Create")],
            };
            create_button
        }
    }

    fn settings_form(&self) -> Container<'_, Message> {
        let theme_input = pick_list(Theme::ALL, Some(self.get_theme()), |theme| {
            Message::App(AppMessage::ThemeChanged(theme))
        })
        .width(220);
        container(column![theme_input]).width(FillPortion(4))
    }
}
