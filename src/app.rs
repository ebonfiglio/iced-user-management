use iced::Task;

use crate::domain::{Entity, Job, Organization, User};
use crate::infrastructure::EntityManager;
use crate::message::{Message, Page};

pub struct AppState {
    pub current_page: Page,
    pub users: EntityManager<User>,
    pub organizations: EntityManager<Organization>,
    pub jobs: EntityManager<Job>,
}

impl AppState {
    pub fn new() -> (Self, Task<Message>) {
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

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Navigate(page) => {
                self.current_page = page;
                with_manager!(self, cancel_edit);
            }
            Message::NameChanged(name) => {
                with_manager!(self, name_changed, name);
            }
            Message::JobSelected(job) => {
                self.users.current.set_job_id(job.id());
            }
            Message::OrganizationSelected(organization) => {
                self.users.current.set_organization_id(organization.id());
            }
            Message::JobClicked(job_id) => {
                if let Some(job) = self.jobs.list.iter().find(|j| j.id() == job_id) {
                    self.current_page = Page::Job;
                    self.jobs.current = job.clone();
                }
            }
            Message::OrganizationClicked(organization_id) => {
                if let Some(organization) = self
                    .organizations
                    .list
                    .iter()
                    .find(|j| j.id() == organization_id)
                {
                    self.current_page = Page::Organization;
                    self.organizations.current = organization.clone();
                }
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

    pub fn get_job_name(&self, job_id: usize) -> String {
        self.jobs
            .list
            .iter()
            .find(|j| j.id() == job_id)
            .map(|j| j.name().to_string())
            .unwrap_or_else(|| "None".to_string())
    }

    pub fn get_organization_name(&self, organization_id: usize) -> String {
        self.organizations
            .list
            .iter()
            .find(|o| o.id() == organization_id)
            .map(|o| o.name().to_string())
            .unwrap_or_else(|| "None".to_string())
    }
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

pub(crate) use with_manager;
