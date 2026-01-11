use iced::{Task, Theme};

use crate::domain::{DomainEntity, Entity, Job, Organization, User, UserService};
use crate::infrastructure::job_repository::JobSqliteRepository;
use crate::infrastructure::organization_repository::OrganizationSqliteRepository;
use crate::infrastructure::user_repository::{self, UserSqliteRepository};
use crate::infrastructure::{get_database_path, Database, EntityState};
use crate::message::{Message, Page};
use std::sync::Arc;

pub struct AppState {
    pub current_page: Page,
    pub active_entity: DomainEntity,
    pub users: EntityState<User>,
    pub organizations: EntityState<Organization>,
    pub jobs: EntityState<Job>,
    pub theme: Theme,
    pub status_message: String,
    pub user_service: Option<UserService>,
}

impl AppState {
    pub fn new() -> (Self, Task<Message>) {
        let task = Task::perform(
            async {
                let db_path = get_database_path();
                let database = Database::new(db_path.to_str().unwrap()).await?;

                let pool = database.pool;
                let user_repo = Arc::new(UserSqliteRepository::new(pool.clone()));
                let job_repo = Arc::new(JobSqliteRepository::new(pool.clone()));
                let org_repo = Arc::new(OrganizationSqliteRepository::new(pool.clone()));

                let user_service = UserService::new(user_repo, job_repo, org_repo);

                Ok::<UserService, sqlx::Error>(user_service)
            },
            |result| match result {
                Ok(user_service) => Message::AppInitialized(user_service),
                Err(e) => Message::InitializationError(e.to_string()),
            },
        );

        let state = Self {
            current_page: Page::User,
            active_entity: DomainEntity::User,
            users: EntityState::new(),
            organizations: EntityState::new(),
            jobs: EntityState::new(),
            theme: Theme::Dark,
            status_message: String::from("Loading..."),
            user_service: None,
        };

        (state, task)
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Navigate(page) => self.set_current_page(page),
            Message::UserNameChanged(name) => {
                self.users.current.set_name(name);
                self.users.current.validate_property("name");
            }
            Message::UserJobSelected(job) => {
                self.users.current.set_job_id(job.id());
                self.users.current.validate_property("job_id");
            }
            Message::UserOrganizationSelected(organization) => {
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
            Message::UserCreate => match self.users.current.validate() {
                Ok(()) => {
                    let user_to_create = self.users.current.clone();
                    if let Some(service) = &self.user_service {
                        let service = service.clone();
                        return Task::perform(
                            async move { service.create_user(user_to_create).await },
                            |result| match result {
                                Ok(user) => Message::UserLoaded(user),
                                Err(e) => Message::UserLoadError(e.to_string()),
                            },
                        );
                    } else {
                        self.status_message = "Service not initialized".to_string();
                    }
                }
                Err(msg) => self.status_message = "Validation Errors".to_string(),
            },
            Message::UserUpdate => {}
            Message::UserDelete(id) => {}

            Message::UserLoad(id) => {
                if let Some(service) = &self.user_service {
                    let service = service.clone();
                    return Task::perform(
                        async move { service.get_user_by_id(id).await },
                        |result| match result {
                            Ok(Some(user)) => Message::UserLoaded(user),
                            Ok(None) => Message::UserNotFound,
                            Err(e) => Message::UserLoadError(e.to_string()),
                        },
                    );
                } else {
                    self.status_message = "Service not initialized".to_string();
                }
            }
            Message::UserLoaded(user) => {
                self.users.current = user;
                self.status_message = "User loaded".to_string();
            }
            Message::UserNotFound => {
                self.status_message = "User not found".to_string();
                self.users.current = User::new();
            }
            Message::UserLoadError(err) => {
                self.status_message = format!("Error loading user: {}", err);
                self.users.current = User::new();
            }
            Message::CancelEdit => match self.current_page {
                Page::User => self.users.cancel_edit(),
                Page::Job => self.jobs.cancel_edit(),
                Page::Organization => self.organizations.cancel_edit(),
                _ => {}
            },
            Message::ThemeChanged(theme) => {
                self.theme = theme;
            }

            Message::AppInitialized(user_service) => {
                self.user_service = Some(user_service);
                self.status_message = "Ready".to_string();
            }
            Message::InitializationError(err) => self.status_message = err,
        }
        Task::none()
    }

    pub fn set_current_page(&mut self, page: Page) {
        match page {
            Page::User => {
                self.users.cancel_edit();
                self.current_page = Page::User;
                self.active_entity = DomainEntity::User;
            }
            Page::Job => {
                self.jobs.cancel_edit();
                self.current_page = Page::Job;
                self.active_entity = DomainEntity::Job;
            }
            Page::Organization => {
                self.organizations.cancel_edit();
                self.current_page = Page::Organization;
                self.active_entity = DomainEntity::Organization;
            }
            Page::Settings => {
                self.current_page = Page::Settings;
                self.active_entity = DomainEntity::None;
            }
        }
    }

    pub fn get_job_name(&self, job_id: i64) -> String {
        self.jobs
            .list
            .iter()
            .find(|j| j.id() == job_id)
            .map(|j| j.name().to_string())
            .unwrap_or_else(|| "None".to_string())
    }

    pub fn get_organization_name(&self, organization_id: i64) -> String {
        self.organizations
            .list
            .iter()
            .find(|o| o.id() == organization_id)
            .map(|o| o.name().to_string())
            .unwrap_or_else(|| "None".to_string())
    }
}
