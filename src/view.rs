use iced::{
    widget::{
        button, column, container, pick_list, row, scrollable, text, text_input, Container, Row,
    },
    Border, Color, Element, Fill, FillPortion, Length, Theme,
};

use crate::app::AppState;
use crate::domain::Entity;
use crate::message::{Message, Page};

impl AppState {
    pub fn view(&self) -> Element<'_, Message> {
        let navigation = container(
            column![
                row![button(container("Users").center_x(30).center_y(30))
                    .width(Length::Fill)
                    .on_press(Message::Navigate(Page::User))],
                row![button(container("Organizations").center_x(30).center_y(30))
                    .width(Length::Fill)
                    .on_press(Message::Navigate(Page::Organization))],
                row![button(container("Jobs").center_x(30).center_y(30))
                    .width(Length::Fill)
                    .on_press(Message::Navigate(Page::Job))],
                row![button(container("Settings").center_x(30).center_y(30))
                    .width(Length::Fill)
                    .on_press(Message::Navigate(Page::Settings))],
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

        let status_bar = container(text(&self.status_message).size(12))
            .padding(5)
            .width(Length::Fill)
            .style(|theme: &Theme| container::Style {
                text_color: Some(
                    if self.status_message.contains("error")
                        || self.status_message.contains("Error")
                    {
                        theme.palette().danger
                    } else if self.status_message.contains("connected") {
                        theme.palette().success
                    } else {
                        theme.palette().text
                    },
                ),
                ..Default::default()
            });

        container(column![
            row![navigation, self.current_page()].spacing(10),
            row![status_bar]
        ])
        .padding(10)
        .into()
    }

    fn current_page(&self) -> Container<'_, Message> {
        match self.current_page {
            Page::Organization => self.organization_form(),
            Page::User => self.user_form(),
            Page::Job => self.job_form(),
            Page::Settings => self.settings_form(),
        }
    }

    fn job_form(&self) -> Container<'_, Message> {
        let name_input = column![
            text_input("Job", &self.jobs.current.name()).on_input(Message::UserNameChanged),
            if let Some(error) = self.jobs.current.errors().get("name") {
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
        let job_list = scrollable(self.jobs.list.iter().enumerate().fold(
            column![header_row].spacing(2),
            |col, (_, job)| {
                col.push(
                    row![
                        text(job.id()).width(Length::FillPortion(1)),
                        text(job.name().to_string()).width(Length::FillPortion(2)),
                        button("Edit")
                            .style(button::primary)
                            .on_press(Message::UserLoad(job.id())),
                        button("Delete")
                            .style(button::danger)
                            .on_press(Message::UserDelete(job.id())),
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
                self.get_form_buttons(self.jobs.is_edit),
                job_list
            ]
            .spacing(10),
        )
        .width(FillPortion(4))
    }

    fn organization_form(&self) -> Container<'_, Message> {
        let name_input = column![
            text_input("Organization", &self.organizations.current.name())
                .on_input(Message::UserNameChanged),
            if let Some(error) = self.organizations.current.errors().get("name") {
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
        let organization_list = scrollable(self.organizations.list.iter().enumerate().fold(
            column![header_row].spacing(2),
            |col, (_, organization)| {
                col.push(
                    row![
                        text(organization.id()).width(Length::FillPortion(1)),
                        text(organization.name().to_string()).width(Length::FillPortion(2)),
                        button("Edit")
                            .style(button::primary)
                            .on_press(Message::UserLoad(organization.id())),
                        button("Delete")
                            .style(button::danger)
                            .on_press(Message::UserDelete(organization.id())),
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
                self.get_form_buttons(self.organizations.is_edit),
                organization_list
            ]
            .spacing(10),
        )
        .width(FillPortion(4))
    }

    fn user_form(&self) -> Container<'_, Message> {
        let name_input = column![
            text_input("User", &self.users.current.name()).on_input(Message::UserNameChanged),
            if let Some(error) = self.users.current.errors().get("name") {
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
                &self.jobs.list[..],
                self.jobs
                    .list
                    .iter()
                    .find(|j| j.id() == self.users.current.job_id()),
                Message::UserJobSelected,
            ),
            if let Some(error) = self.users.current.errors().get("job_id") {
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
                &self.organizations.list[..],
                self.organizations
                    .list
                    .iter()
                    .find(|k| k.id() == self.users.current.organization_id()),
                Message::UserOrganizationSelected,
            ),
            if let Some(error) = self.users.current.errors().get("organization_id") {
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
        let user_list = scrollable(self.users.list.iter().enumerate().fold(
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
                            .on_press(Message::JobClicked(user.job_id()))
                            .width(Length::FillPortion(2)),
                        button(text(organization_name))
                            .style(button::text)
                            .on_press(Message::OrganizationClicked(user.organization_id()))
                            .width(Length::FillPortion(2)),
                        button("Edit")
                            .style(button::primary)
                            .on_press(Message::UserLoad(user.id()))
                            .width(Length::FillPortion(1)),
                        button("Delete")
                            .style(button::danger)
                            .on_press(Message::UserDelete(user.id()))
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
                self.get_form_buttons(self.users.is_edit),
                user_list
            ]
            .spacing(10),
        )
        .width(FillPortion(4))
    }

    fn get_form_buttons(&self, is_edit: bool) -> Row<'_, Message> {
        if is_edit {
            row![
                button("Update").on_press(Message::UserUpdate),
                button("Cancel")
                    .style(button::danger)
                    .on_press(Message::CancelEdit)
            ]
            .spacing(10)
        } else {
            row![button("Create").on_press(Message::UserCreate)]
        }
    }

    fn settings_form(&self) -> Container<'_, Message> {
        let theme_input =
            pick_list(Theme::ALL, Some(&self.theme), Message::ThemeChanged).width(220);
        container(column![theme_input]).width(FillPortion(4))
    }
}
