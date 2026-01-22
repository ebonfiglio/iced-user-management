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
    current_page: Page,
    pub active_entity: DomainEntity,
    pub users: EntityState<User>,
    pub organizations: EntityState<Organization>,
    pub jobs: EntityState<Job>,
    pub theme: Theme,
    status_message: String,
    pub user_service: Option<UserService>,
    pub job_service: Option<JobService>,
    pub organization_service: Option<OrganizationService>,
}

impl AppState {
    pub fn status_message(&self) -> &str {
        &self.status_message
    }
    pub fn set_status_message(&mut self, message: String) {
        self.status_message = message;
    }

    pub fn current_page(&self) -> Page {
        self.current_page
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

                let user_service = UserService::new(user_repo, job_repo.clone(), org_repo.clone());
                let job_service = JobService::new(job_repo.clone());
                let organization_service = OrganizationService::new(org_repo);

                Ok::<(UserService, JobService, OrganizationService), sqlx::Error>((
                    user_service,
                    job_service,
                    organization_service,
                ))
            },
            |result| match result {
                Ok((user_service, job_service, organization_service)) => Message::App(
                    AppMessage::Initialized(user_service, job_service, organization_service),
                ),
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
            organization_service: None,
        };

        (state, task)
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::App(app_msg) => match app_msg {
                AppMessage::Navigate(page) => {
                    self.set_current_page(page);
                    return Task::done(Message::App(AppMessage::SetStatusMessage(
                        "Ready".to_string(),
                    )));
                }
                AppMessage::ThemeChanged(theme) => {
                    self.theme = theme;
                }
                AppMessage::Initialized(user_service, job_service, organization_service) => {
                    self.user_service = Some(user_service);
                    self.job_service = Some(job_service);
                    self.organization_service = Some(organization_service);
                    return Task::batch(vec![
                        Task::done(Message::Job(JobMessage::GetAll)),
                        Task::done(Message::Organization(OrganizationMessage::GetAll)),
                    ])
                    .chain(Task::done(Message::App(
                        AppMessage::SetStatusMessage("Ready".to_string()),
                    )));
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
                AppMessage::SetStatusMessage(msg) => self.set_status_message(msg),
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
                            return Task::done(Message::App(AppMessage::SetStatusMessage(
                                "Service not initialized".to_string(),
                            )));
                        }
                    }
                    Err(msg) => {
                        return Task::done(Message::App(AppMessage::SetStatusMessage(format!(
                            "Validation Errors:\n{}",
                            msg.iter()
                                .map(|(key, value)| format!("  • {}: {}", key, value))
                                .collect::<Vec<_>>()
                                .join("\n")
                        ))));
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
                        return Task::done(Message::Job(JobMessage::GetAll)).chain(Task::done(
                            Message::App(AppMessage::SetStatusMessage(
                                "Service not initialized".to_string(),
                            )),
                        ));
                    }
                }
                JobMessage::Loaded(job) => {
                    self.jobs.current = job;
                    self.jobs.is_edit = true;
                    return Task::done(Message::Job(JobMessage::GetAll)).chain(Task::done(
                        Message::App(AppMessage::SetStatusMessage("Job loaded".to_string())),
                    ));
                }
                JobMessage::Update => {}
                JobMessage::Delete(id) => {}
                JobMessage::NotFound => {
                    self.jobs.current = Job::new();
                    return Task::done(Message::App(AppMessage::SetStatusMessage(
                        "Job not found".to_string(),
                    )));
                }
                JobMessage::LoadError(err) => {
                    self.jobs.current = Job::new();
                    return Task::done(Message::App(AppMessage::SetStatusMessage(format!(
                        "Error loading job: {}",
                        err
                    ))));
                }
            },
            Message::Organization(org_msg) => match org_msg {
                OrganizationMessage::GetAll => {
                    if let Some(service) = &self.organization_service {
                        let service = service.clone();
                        return Task::perform(
                            async move { service.get_all_organizations().await },
                            |result| match result {
                                Ok(organizations) => Message::Organization(
                                    OrganizationMessage::SetAll(organizations),
                                ),
                                Err(e) => Message::Organization(OrganizationMessage::LoadError(
                                    e.to_string(),
                                )),
                            },
                        );
                    }
                }
                OrganizationMessage::SetAll(organizations) => {
                    self.organizations.list = organizations;
                }
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
                OrganizationMessage::Create => match self.organizations.current.validate() {
                    Ok(()) => {
                        let organization_to_create = self.organizations.current.clone();
                        if let Some(service) = &self.organization_service {
                            let service = service.clone();
                            return Task::perform(
                                async move { service.create_organization(organization_to_create).await },
                                |result| match result {
                                    Ok(organization) => {
                                        Message::Organization(OrganizationMessage::CreateSuccess)
                                    }
                                    Err(e) => Message::Organization(
                                        OrganizationMessage::LoadError(e.to_string()),
                                    ),
                                },
                            );
                        } else {
                            return Task::done(Message::App(AppMessage::SetStatusMessage(
                                "Service not initialized".to_string(),
                            )));
                        }
                    }
                    Err(msg) => {
                        return Task::done(Message::App(AppMessage::SetStatusMessage(format!(
                            "Validation Errors:\n{}",
                            msg.iter()
                                .map(|(key, value)| format!("  • {}: {}", key, value))
                                .collect::<Vec<_>>()
                                .join("\n")
                        ))));
                    }
                },
                OrganizationMessage::Load(id) => {
                    if let Some(service) = &self.organization_service {
                        let service = service.clone();
                        return Task::perform(
                            async move { service.get_organization_by_id(id).await },
                            |result| match result {
                                Ok(Some(organization)) => {
                                    Message::Organization(OrganizationMessage::Loaded(organization))
                                }
                                Ok(None) => Message::Organization(OrganizationMessage::NotFound),
                                Err(e) => Message::Organization(OrganizationMessage::LoadError(
                                    e.to_string(),
                                )),
                            },
                        );
                    } else {
                        return Task::done(Message::Organization(OrganizationMessage::GetAll))
                            .chain(Task::done(Message::App(AppMessage::SetStatusMessage(
                                "Service not initialized".to_string(),
                            ))));
                    }
                }
                OrganizationMessage::Update => {}
                OrganizationMessage::Delete(id) => {}
                OrganizationMessage::NotFound => {
                    self.organizations.current = Organization::new();
                    return Task::done(Message::App(AppMessage::SetStatusMessage(
                        "Organization not found".to_string(),
                    )));
                }
                OrganizationMessage::LoadError(err) => {
                    self.organizations.current = Organization::new();
                    return Task::done(Message::App(AppMessage::SetStatusMessage(format!(
                        "Error loading organization: {}",
                        err
                    ))));
                }
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
                            return Task::done(Message::App(AppMessage::SetStatusMessage(
                                "Service not initialized".to_string(),
                            )));
                        }
                    }
                    Err(msg) => {
                        return Task::done(Message::App(AppMessage::SetStatusMessage(
                            "Validation Errors".to_string(),
                        )));
                    }
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
                        return Task::done(Message::App(AppMessage::SetStatusMessage(
                            "UserService not initialized".to_string(),
                        )));
                    }
                }
                UserMessage::Loaded(user) => {
                    self.users.current = user;
                    return Task::done(Message::App(AppMessage::SetStatusMessage(
                        "User loaded".to_string(),
                    )));
                }
                UserMessage::NotFound => {
                    self.users.current = User::new();
                    return Task::done(Message::App(AppMessage::SetStatusMessage(
                        "User not found".to_string(),
                    )));
                }
                UserMessage::LoadError(err) => {
                    self.users.current = User::new();
                    return Task::done(Message::App(AppMessage::SetStatusMessage(format!(
                        "Error loading user: {}",
                        err
                    ))));
                }
            },
        }
        Task::none()
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
