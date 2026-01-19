use iced::{Task, Theme};

use crate::domain::services::JobService;
use crate::domain::services::OrganizationService;
use crate::domain::{DomainEntity, Entity, Job, Organization, User, UserService};
use crate::infrastructure::job_repository::JobSqliteRepository;
use crate::infrastructure::organization_repository::OrganizationSqliteRepository;
use crate::infrastructure::user_repository::UserSqliteRepository;
use crate::infrastructure::{get_database_path, Database, EntityState};
use crate::message::{
    app_message::AppMessage, job_message::JobMessage, organization_message::OrganizationMessage,
    user_message::UserMessage, Message, Page,
};
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
    pub job_service: Option<JobService>,
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

                let user_service = UserService::new(user_repo, job_repo.clone(), org_repo);
                let job_service = JobService::new(job_repo.clone());

                Ok::<(UserService, JobService), sqlx::Error>((user_service, job_service))
            },
            |result| match result {
                Ok((user_service, job_service)) => {
                    Message::App(AppMessage::Initialized(user_service, job_service))
                }
                Err(e) => Message::App(AppMessage::InitializationError(e.to_string())),
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
            job_service: None,
        };

        (state, task)
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::App(app_msg) => match app_msg {
                AppMessage::Navigate(page) => self.set_current_page(page),
                AppMessage::ThemeChanged(theme) => {
                    self.theme = theme;
                }
                AppMessage::Initialized(user_service, job_service) => {
                    self.user_service = Some(user_service);
                    self.job_service = Some(job_service);
                    return Task::done(Message::Job(JobMessage::GetAll)).chain(Task::done(
                        Message::App(AppMessage::SetStatusMessage("Ready".to_string())),
                    ));
                }
                AppMessage::InitializationError(err) => self.status_message = err,
                AppMessage::CancelEdit => {
                    match self.current_page {
                        Page::User => self.users.clear_entity_state(),
                        Page::Job => self.jobs.clear_entity_state(),
                        Page::Organization => self.organizations.clear_entity_state(),
                        _ => {}
                    };
                    return Task::done(Message::App(AppMessage::SetStatusMessage(
                        "Ready".to_string(),
                    )));
                }
                AppMessage::SetStatusMessage(msg) => self.status_message = msg,
            },
            Message::Job(job_msg) => match job_msg {
                JobMessage::GetAll => {
                    if let Some(service) = &self.job_service {
                        let service = service.clone();
                        return Task::perform(
                            async move { service.get_all_jobs().await },
                            |result| match result {
                                Ok(jobs) => Message::Job(JobMessage::SetAll(jobs)),
                                Err(e) => Message::Job(JobMessage::LoadError(e.to_string())),
                            },
                        );
                    }
                }
                JobMessage::SetAll(jobs) => {
                    self.jobs.list = jobs;
                }
                JobMessage::Clicked(job_id) => {
                    if let Some(job) = self.jobs.list.iter().find(|j| j.id() == job_id).cloned() {
                        self.set_current_page(Page::Job);
                        self.jobs.current = job;
                    }
                }
                JobMessage::NameChanged(name) => {
                    self.jobs.current.set_name(name);
                    self.jobs.current.validate_property("name");
                }
                JobMessage::Create => match self.jobs.current.validate() {
                    Ok(()) => {
                        let job_to_create = self.jobs.current.clone();
                        if let Some(service) = &self.job_service {
                            let service = service.clone();
                            return Task::perform(
                                async move { service.create_job(job_to_create).await },
                                |result| match result {
                                    Ok(job) => Message::Job(JobMessage::CreateSuccess),
                                    Err(e) => Message::Job(JobMessage::LoadError(e.to_string())),
                                },
                            );
                        } else {
                            self.status_message = "Service not initialized".to_string();
                        }
                    }
                    Err(msg) => {
                        self.status_message = format!(
                            "Validation Errors:\n{}",
                            msg.iter()
                                .map(|(key, value)| format!("  • {}: {}", key, value))
                                .collect::<Vec<_>>()
                                .join("\n")
                        );
                    }
                },
                JobMessage::CreateSuccess => {
                    self.jobs.clear_entity_state();
                    return Task::done(Message::Job(JobMessage::GetAll)).chain(Task::done(
                        Message::App(AppMessage::SetStatusMessage("Ready".to_string())),
                    ));
                }
                JobMessage::Load(id) => {
                    if let Some(service) = &self.job_service {
                        let service = service.clone();
                        return Task::perform(
                            async move { service.get_job_by_id(id).await },
                            |result| match result {
                                Ok(Some(job)) => Message::Job(JobMessage::Loaded(job)),
                                Ok(None) => Message::Job(JobMessage::NotFound),
                                Err(e) => Message::Job(JobMessage::LoadError(e.to_string())),
                            },
                        );
                    } else {
                        self.status_message = "Service not initialized".to_string();
                    }
                }
                JobMessage::Loaded(job) => {
                    self.jobs.current = job;
                    self.jobs.is_edit = true;
                    self.status_message = "Job loaded".to_string();
                }
                JobMessage::Update => {}
                JobMessage::Delete(id) => {}
                JobMessage::NotFound => {
                    self.status_message = "Job not found".to_string();
                    self.jobs.current = Job::new();
                }
                JobMessage::LoadError(err) => {
                    self.status_message = format!("Error loading job: {}", err);
                    self.jobs.current = Job::new();
                }
            },
            Message::Organization(org_msg) => match org_msg {
                OrganizationMessage::Clicked(organization_id) => {
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
                OrganizationMessage::Create => {}
            },
            Message::User(user_msg) => match user_msg {
                UserMessage::NameChanged(name) => {
                    self.users.current.set_name(name);
                    self.users.current.validate_property("name");
                }
                UserMessage::JobSelected(job) => {
                    self.users.current.set_job_id(job.id());
                    self.users.current.validate_property("job_id");
                }
                UserMessage::OrganizationSelected(organization) => {
                    self.users.current.set_organization_id(organization.id());
                    self.users.current.validate_property("organization_id");
                }
                UserMessage::Create => match self.users.current.validate() {
                    Ok(()) => {
                        let user_to_create = self.users.current.clone();
                        if let Some(service) = &self.user_service {
                            let service = service.clone();
                            return Task::perform(
                                async move { service.create_user(user_to_create).await },
                                |result| match result {
                                    Ok(user) => Message::User(UserMessage::Loaded(user)),
                                    Err(e) => Message::User(UserMessage::LoadError(e.to_string())),
                                },
                            );
                        } else {
                            self.status_message = "Service not initialized".to_string();
                        }
                    }
                    Err(msg) => self.status_message = "Validation Errors".to_string(),
                },
                UserMessage::Update => {}
                UserMessage::Delete(id) => {}
                UserMessage::Load(id) => {
                    if let Some(service) = &self.user_service {
                        let service = service.clone();
                        return Task::perform(
                            async move { service.get_user_by_id(id).await },
                            |result| match result {
                                Ok(Some(user)) => Message::User(UserMessage::Loaded(user)),
                                Ok(None) => Message::User(UserMessage::NotFound),
                                Err(e) => Message::User(UserMessage::LoadError(e.to_string())),
                            },
                        );
                    } else {
                        self.status_message = "Service not initialized".to_string();
                    }
                }
                UserMessage::Loaded(user) => {
                    self.users.current = user;
                    self.status_message = "User loaded".to_string();
                }
                UserMessage::NotFound => {
                    self.status_message = "User not found".to_string();
                    self.users.current = User::new();
                }
                UserMessage::LoadError(err) => {
                    self.status_message = format!("Error loading user: {}", err);
                    self.users.current = User::new();
                }
            },
        }
        Task::none()
    }

    pub fn set_current_page(&mut self, page: Page) {
        match page {
            Page::User => {
                self.users.clear_entity_state();
                self.current_page = Page::User;
                self.active_entity = DomainEntity::User;
            }
            Page::Job => {
                self.jobs.clear_entity_state();
                self.current_page = Page::Job;
                self.active_entity = DomainEntity::Job;
            }
            Page::Organization => {
                self.organizations.clear_entity_state();
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
