use iced::{Task, Theme};

use crate::domain::{DomainEntity, Entity, Job, Organization, User};
use crate::infrastructure::{get_database_path, Database, EntityManager};
use crate::message::{Message, Page};
use sqlx::SqlitePool;

pub struct AppState {
    pub current_page: Page,
    pub active_entity: DomainEntity,
    pub users: EntityManager<User>,
    pub organizations: EntityManager<Organization>,
    pub jobs: EntityManager<Job>,
    pub theme: Theme,
    pub db_pool: Option<SqlitePool>,
    pub status_message: String,
}

impl AppState {
    pub fn new() -> (Self, Task<Message>) {
        let state = Self {
            current_page: Page::User,
            active_entity: DomainEntity::User,
            users: EntityManager::new(),
            organizations: EntityManager::new(),
            jobs: EntityManager::new(),
            theme: Theme::Dark,
            db_pool: None,
            status_message: String::from("Loading..."),
        };

        let task = Task::perform(
            async {
                let db_path = get_database_path();
                Database::new(db_path.to_str().unwrap()).await
            },
            |result| match result {
                Ok(db) => Message::DatabaseInitialized(db.pool),
                Err(e) => Message::DatabaseError(e.to_string()),
            },
        );

        (state, task)
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Navigate(page) => {
                self.set_current_page(page);
                with_manager!(self, cancel_edit);
            }
            Message::NameChanged(name) => {
                with_manager!(self, name_changed, name);
            }
            Message::JobSelected(job) => {
                self.users.current.set_job_id(job.id());
                self.users.current.validate_property("job_id");
            }
            Message::OrganizationSelected(organization) => {
                self.users.current.set_organization_id(organization.id());
                self.users.current.validate_property("organization_id");
            }
            Message::JobClicked(job_id) => {
                if let Some(job) = self.jobs.list.iter().find(|j| j.id() == job_id).cloned() {
                    self.set_current_page(Page::Job);
                    self.jobs.current = job;
                }
            }
            Message::OrganizationClicked(organization_id) => {
                if let Some(organization) = self
                    .organizations
                    .list
                    .iter()
                    .find(|j| j.id() == organization_id)
                    .cloned()
                {
                    self.set_current_page(Page::Organization);
                    self.organizations.current = organization;
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
            Message::ThemeChanged(theme) => {
                self.theme = theme;
            }
            Message::DatabaseInitialized(db_pool) => {
                self.db_pool = Some(db_pool);
                self.status_message = "Database connected".to_string();
            }
            Message::DatabaseError(err) => {
                self.status_message = format!("Database error: {}", err);
            }
        }
        Task::none()
    }

    pub fn set_current_page(&mut self, page: Page) {
        match page {
            Page::User => {
                self.current_page = Page::User;
                self.active_entity = DomainEntity::User;
            }
            Page::Job => {
                self.current_page = Page::Job;
                self.active_entity = DomainEntity::Job;
            }
            Page::Organization => {
                self.current_page = Page::Organization;
                self.active_entity = DomainEntity::Organization;
            }
            Page::Settings => {
                self.current_page = Page::Settings;
                self.active_entity = DomainEntity::None;
            }
        }
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
        match $self.active_entity {
            DomainEntity::User => { let _ = $self.users.$method($($arg),*); }
            DomainEntity::Organization => { let _ = $self.organizations.$method($($arg),*); }
            DomainEntity::Job => { let _ = $self.jobs.$method($($arg),*); }
            DomainEntity::None => {}
        }
    };
}

pub(crate) use with_manager;
