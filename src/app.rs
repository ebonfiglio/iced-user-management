use crate::domain::services::JobService;
use crate::domain::services::OrganizationService;
use crate::domain::{Entity, Job, Organization, User, UserService};
use crate::infrastructure::job_repository::JobSqliteRepository;
use crate::infrastructure::organization_repository::OrganizationSqliteRepository;
use crate::infrastructure::user_repository::UserSqliteRepository;
use crate::infrastructure::{get_database_path, Database, EntityState};
use crate::message::{
    app_message::AppMessage, job_message::JobMessage, organization_message::OrganizationMessage,
    user_message::UserMessage, Message,
};
use crate::page::Page;
use iced::{Task, Theme};
use std::collections::HashMap;
use std::sync::Arc;

pub enum AppState {
    Loading,
    Ready(ReadyState),
    Error(String),
}

pub struct ReadyState {
    user_service: UserService,
    job_service: JobService,
    organization_service: OrganizationService,
    current_page: Page,
    users: EntityState<User>,
    organizations: EntityState<Organization>,
    jobs: EntityState<Job>,
    theme: Theme,
    status_message: String,
}
impl AppState {
    pub fn new() -> (Self, Task<Message>) {
        let task = Task::perform(
            async {
                let db_path = get_database_path();
                let database = Database::new(db_path.to_str().unwrap()).await?;

                let pool = database.get_pool();
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

        (AppState::Loading, task)
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match self {
            AppState::Loading => {
                if let Message::App(AppMessage::Initialized(
                    user_service,
                    job_service,
                    organization_service,
                )) = message
                {
                    *self = AppState::Ready(ReadyState {
                        user_service,
                        job_service,
                        organization_service,
                        current_page: Page::User,
                        users: EntityState::new(),
                        organizations: EntityState::new(),
                        jobs: EntityState::new(),
                        theme: Theme::Dark,
                        status_message: "Ready".to_string(),
                    });
                    return Task::batch(vec![
                        Task::done(Message::Job(JobMessage::GetAll)),
                        Task::done(Message::Organization(OrganizationMessage::GetAll)),
                        Task::done(Message::User(UserMessage::GetAll)),
                    ]);
                } else if let Message::App(AppMessage::InitializationError(err)) = message {
                    *self = AppState::Error(err);
                    return Task::none();
                } else {
                    return Task::none();
                }
            }
            AppState::Ready(ready_state) => ready_state.handle_message(message),
            AppState::Error(_) => Task::none(),
        }
    }
    pub fn get_theme(&self) -> &Theme {
        match self {
            AppState::Ready(state) => &state.theme,
            _ => &Theme::Dark,
        }
    }
}

impl ReadyState {
    fn handle_message(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::App(app_msg) => self.handle_app_message(app_msg),
            Message::Job(job_msg) => self.handle_job_message(job_msg),
            Message::Organization(org_msg) => self.handle_organization_message(org_msg),
            Message::User(user_msg) => self.handle_user_message(user_msg),
        }
    }
    fn handle_app_message(&mut self, msg: AppMessage) -> Task<Message> {
        match msg {
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
                self.user_service = user_service;
                self.job_service = job_service;
                self.organization_service = organization_service;
                return Task::batch(vec![
                    Task::done(Message::Job(JobMessage::GetAll)),
                    Task::done(Message::Organization(OrganizationMessage::GetAll)),
                    Task::done(Message::User(UserMessage::GetAll)),
                ])
                .chain(Task::done(Message::App(AppMessage::SetStatusMessage(
                    "Ready".to_string(),
                ))));
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
        }

        Task::none()
    }

    fn handle_job_message(&mut self, msg: JobMessage) -> Task<Message> {
        match msg {
            JobMessage::GetAll => {
                let service = self.job_service.clone();
                return Task::perform(async move { service.get_all_jobs().await }, |result| {
                    match result {
                        Ok(jobs) => Message::Job(JobMessage::SetAll(jobs)),
                        Err(e) => Message::Job(JobMessage::LoadError(e.to_string())),
                    }
                });
            }
            JobMessage::SetAll(jobs) => {
                self.jobs.set_list(jobs);
            }
            JobMessage::Clicked(job_id) => {
                self.set_current_page(Page::Job);
                return Task::done(Message::Job(JobMessage::Load(job_id)));
            }
            JobMessage::NameChanged(name) => {
                self.jobs.current_mut().set_name(name);
                self.jobs.current_mut().validate_property("name");
            }
            JobMessage::Create => match self.jobs.current_mut().validate() {
                Ok(()) => {
                    let job_to_create = self.jobs.current().clone();
                    let service = self.job_service.clone();
                    return Task::perform(
                        async move { service.create_job(job_to_create).await },
                        |result| match result {
                            Ok(_) => Message::Job(JobMessage::CreateSuccess),
                            Err(e) => Message::Job(JobMessage::LoadError(e.to_string())),
                        },
                    );
                }
                Err(errors) => return ReadyState::set_validation_status_message(errors),
            },
            JobMessage::CreateSuccess => {
                self.jobs.clear_entity_state();
                return Task::done(Message::Job(JobMessage::GetAll)).chain(Task::done(
                    Message::App(AppMessage::SetStatusMessage("Ready".to_string())),
                ));
            }
            JobMessage::UpdateSuccess => {
                self.jobs.clear_entity_state();
                return Task::done(Message::Job(JobMessage::GetAll)).chain(Task::done(
                    Message::App(AppMessage::SetStatusMessage("Ready".to_string())),
                ));
            }
            JobMessage::Load(id) => {
                let service = self.job_service.clone();
                return Task::perform(async move { service.get_job_by_id(id).await }, |result| {
                    match result {
                        Ok(Some(job)) => Message::Job(JobMessage::Loaded(job)),
                        Ok(None) => Message::Job(JobMessage::NotFound),
                        Err(e) => Message::Job(JobMessage::LoadError(e.to_string())),
                    }
                });
            }
            JobMessage::Loaded(job) => {
                self.jobs.set_current(job);
                self.jobs.set_is_edit(true);
                return Task::done(Message::Job(JobMessage::GetAll)).chain(Task::done(
                    Message::App(AppMessage::SetStatusMessage("Job loaded".to_string())),
                ));
            }
            JobMessage::Update => match self.jobs.current_mut().validate() {
                Ok(()) => {
                    let service = self.job_service.clone();
                    let job = self.jobs.current().clone();
                    return Task::perform(async move { service.update_job(job).await }, |result| {
                        match result {
                            Ok(()) => Message::Job(JobMessage::UpdateSuccess),
                            Err(e) => Message::Job(JobMessage::LoadError(e.to_string())),
                        }
                    });
                }
                Err(errors) => return ReadyState::set_validation_status_message(errors),
            },
            JobMessage::Delete(id) => {
                let service = self.job_service.clone();
                return Task::perform(async move { service.delete_job(id).await }, |result| {
                    match result {
                        Ok(()) => Message::Job(JobMessage::DeleteSuccess),
                        Err(e) => Message::App(AppMessage::SetStatusMessage(e.to_string())),
                    }
                });
            }
            JobMessage::DeleteSuccess => {
                self.jobs.clear_entity_state();
                return Task::done(Message::Job(JobMessage::GetAll)).chain(Task::done(
                    Message::App(AppMessage::SetStatusMessage("Ready".to_string())),
                ));
            }
            JobMessage::NotFound => {
                self.jobs.set_current(Job::new());
                return Task::done(Message::App(AppMessage::SetStatusMessage(
                    "Job not found".to_string(),
                )));
            }
            JobMessage::LoadError(err) => {
                self.jobs.set_current(Job::new());
                return Task::done(Message::App(AppMessage::SetStatusMessage(format!(
                    "Error loading job: {}",
                    err
                ))));
            }
        }

        Task::none()
    }

    fn handle_organization_message(&mut self, msg: OrganizationMessage) -> Task<Message> {
        match msg {
            OrganizationMessage::GetAll => {
                let service = self.organization_service.clone();
                return Task::perform(
                    async move { service.get_all_organizations().await },
                    |result| match result {
                        Ok(organizations) => {
                            Message::Organization(OrganizationMessage::SetAll(organizations))
                        }
                        Err(e) => {
                            Message::Organization(OrganizationMessage::LoadError(e.to_string()))
                        }
                    },
                );
            }
            OrganizationMessage::SetAll(organizations) => {
                self.organizations.set_list(organizations);
            }
            OrganizationMessage::Clicked(organization_id) => {
                self.set_current_page(Page::Organization);
                return Task::done(Message::Organization(OrganizationMessage::Load(
                    organization_id,
                )));
            }
            OrganizationMessage::NameChanged(name) => {
                self.organizations.current_mut().set_name(name);
                self.organizations.current_mut().validate_property("name");
            }
            OrganizationMessage::Create => match self.organizations.current_mut().validate() {
                Ok(()) => {
                    let organization_to_create = self.organizations.current().clone();
                    let service = self.organization_service.clone();
                    return Task::perform(
                        async move { service.create_organization(organization_to_create).await },
                        |result| match result {
                            Ok(_) => Message::Organization(OrganizationMessage::CreateSuccess),
                            Err(e) => {
                                Message::Organization(OrganizationMessage::LoadError(e.to_string()))
                            }
                        },
                    );
                }
                Err(errors) => return ReadyState::set_validation_status_message(errors),
            },
            OrganizationMessage::CreateSuccess => {
                self.organizations.clear_entity_state();
                return Task::done(Message::Organization(OrganizationMessage::GetAll)).chain(
                    Task::done(Message::App(AppMessage::SetStatusMessage(
                        "Ready".to_string(),
                    ))),
                );
            }
            OrganizationMessage::Load(id) => {
                let service = self.organization_service.clone();
                return Task::perform(
                    async move { service.get_organization_by_id(id).await },
                    |result| match result {
                        Ok(Some(organization)) => {
                            Message::Organization(OrganizationMessage::Loaded(organization))
                        }
                        Ok(None) => Message::Organization(OrganizationMessage::NotFound),
                        Err(e) => {
                            Message::Organization(OrganizationMessage::LoadError(e.to_string()))
                        }
                    },
                );
            }
            OrganizationMessage::Loaded(organization) => {
                self.organizations.set_current(organization);
                self.organizations.set_is_edit(true);
                return Task::done(Message::Organization(OrganizationMessage::GetAll)).chain(
                    Task::done(Message::App(AppMessage::SetStatusMessage(
                        "Organization loaded".to_string(),
                    ))),
                );
            }
            OrganizationMessage::Update => match self.organizations.current_mut().validate() {
                Ok(()) => {
                    let service = self.organization_service.clone();
                    let organization_to_update = self.organizations.current().clone();
                    return Task::perform(
                        async move { service.update_organization(organization_to_update).await },
                        |result| match result {
                            Ok(_) => Message::Organization(OrganizationMessage::UpdateSuccess),
                            Err(e) => {
                                Message::Organization(OrganizationMessage::LoadError(e.to_string()))
                            }
                        },
                    );
                }
                Err(errors) => return ReadyState::set_validation_status_message(errors),
            },
            OrganizationMessage::UpdateSuccess => {
                self.organizations.clear_entity_state();
                return Task::done(Message::Organization(OrganizationMessage::GetAll)).chain(
                    Task::done(Message::App(AppMessage::SetStatusMessage(
                        "Ready".to_string(),
                    ))),
                );
            }
            OrganizationMessage::Delete(id) => {
                let service = self.organization_service.clone();
                return Task::perform(
                    async move { service.delete_organization(id).await },
                    |result| match result {
                        Ok(()) => Message::Organization(OrganizationMessage::DeleteSuccess),
                        Err(e) => Message::App(AppMessage::SetStatusMessage(e.to_string())),
                    },
                );
            }
            OrganizationMessage::DeleteSuccess => {
                self.organizations.clear_entity_state();
                return Task::done(Message::Organization(OrganizationMessage::GetAll)).chain(
                    Task::done(Message::App(AppMessage::SetStatusMessage(
                        "Ready".to_string(),
                    ))),
                );
            }
            OrganizationMessage::NotFound => {
                self.organizations.set_current(Organization::new());
                return Task::done(Message::App(AppMessage::SetStatusMessage(
                    "Organization not found".to_string(),
                )));
            }
            OrganizationMessage::LoadError(err) => {
                self.organizations.set_current(Organization::new());
                return Task::done(Message::App(AppMessage::SetStatusMessage(format!(
                    "Error loading organization: {}",
                    err
                ))));
            }
        }
        Task::none()
    }

    fn handle_user_message(&mut self, msg: UserMessage) -> Task<Message> {
        match msg {
            UserMessage::NameChanged(name) => {
                self.users.current_mut().set_name(name);
                self.users.current_mut().validate_property("name");
            }
            UserMessage::JobSelected(job) => {
                self.users.current_mut().set_job_id(job.id());
                self.users.current_mut().validate_property("job_id");
            }
            UserMessage::OrganizationSelected(organization) => {
                self.users
                    .current_mut()
                    .set_organization_id(organization.id());
                self.users
                    .current_mut()
                    .validate_property("organization_id");
            }
            UserMessage::Create => match self.users.current_mut().validate() {
                Ok(()) => {
                    let user_to_create = self.users.current().clone();
                    let service = self.user_service.clone();
                    return Task::perform(
                        async move { service.create_user(user_to_create).await },
                        |result| match result {
                            Ok(_) => Message::User(UserMessage::CreateSuccess),
                            Err(e) => Message::User(UserMessage::LoadError(e.to_string())),
                        },
                    );
                }
                Err(errors) => return ReadyState::set_validation_status_message(errors),
            },
            UserMessage::CreateSuccess => {
                self.users.clear_entity_state();
                return Task::done(Message::User(UserMessage::GetAll)).chain(Task::done(
                    Message::App(AppMessage::SetStatusMessage("Ready".to_string())),
                ));
            }
            UserMessage::Update => match self.users.current_mut().validate() {
                Ok(()) => {
                    let service = self.user_service.clone();
                    let user = self.users.current().clone();
                    return Task::perform(
                        async move { service.update_user(user).await },
                        |result| match result {
                            Ok(()) => Message::User(UserMessage::UpdateSuccess),
                            Err(e) => Message::User(UserMessage::LoadError(e.to_string())),
                        },
                    );
                }
                Err(errors) => return ReadyState::set_validation_status_message(errors),
            },
            UserMessage::UpdateSuccess => {
                self.users.clear_entity_state();
                return Task::done(Message::User(UserMessage::GetAll)).chain(Task::done(
                    Message::App(AppMessage::SetStatusMessage("Ready".to_string())),
                ));
            }
            UserMessage::Delete(id) => {
                let service = self.user_service.clone();
                return Task::perform(async move { service.delete_user(id).await }, |result| {
                    match result {
                        Ok(()) => Message::User(UserMessage::DeleteSuccess),
                        Err(e) => Message::App(AppMessage::SetStatusMessage(e.to_string())),
                    }
                });
            }
            UserMessage::DeleteSuccess => {
                self.users.clear_entity_state();
                return Task::done(Message::User(UserMessage::GetAll)).chain(Task::done(
                    Message::App(AppMessage::SetStatusMessage("Ready".to_string())),
                ));
            }
            UserMessage::Load(id) => {
                let service = self.user_service.clone();
                return Task::perform(async move { service.get_user_by_id(id).await }, |result| {
                    match result {
                        Ok(Some(user)) => Message::User(UserMessage::Loaded(user)),
                        Ok(None) => Message::User(UserMessage::NotFound),
                        Err(e) => Message::User(UserMessage::LoadError(e.to_string())),
                    }
                });
            }
            UserMessage::Loaded(user) => {
                self.users.set_current(user);
                self.users.set_is_edit(true);
                return Task::done(Message::User(UserMessage::GetAll)).chain(Task::done(
                    Message::App(AppMessage::SetStatusMessage("User loaded".to_string())),
                ));
            }
            UserMessage::NotFound => {
                self.users.set_current(User::new());
                return Task::done(Message::App(AppMessage::SetStatusMessage(
                    "User not found".to_string(),
                )));
            }
            UserMessage::LoadError(err) => {
                self.users.set_current(User::new());
                return Task::done(Message::App(AppMessage::SetStatusMessage(format!(
                    "Error loading user: {}",
                    err
                ))));
            }
            UserMessage::GetAll => {
                let service = self.user_service.clone();
                return Task::perform(async move { service.get_all_users().await }, |result| {
                    match result {
                        Ok(users) => Message::User(UserMessage::SetAll(users)),
                        Err(e) => Message::User(UserMessage::LoadError(e.to_string())),
                    }
                });
            }
            UserMessage::SetAll(users) => {
                self.users.set_list(users);
            }
        }
        Task::none()
    }
}

impl ReadyState {
    pub fn get_job_name(&self, job_id: i64) -> String {
        self.jobs
            .list()
            .iter()
            .find(|j| j.id() == job_id)
            .map(|j| j.name().to_string())
            .unwrap_or_else(|| "None".to_string())
    }

    pub fn get_organization_name(&self, organization_id: i64) -> String {
        self.organizations
            .list()
            .iter()
            .find(|o| o.id() == organization_id)
            .map(|o| o.name().to_string())
            .unwrap_or_else(|| "None".to_string())
    }

    pub fn set_validation_status_message(
        errors: &HashMap<&'static str, &'static str>,
    ) -> Task<Message> {
        return Task::done(Message::App(AppMessage::SetStatusMessage(format!(
            "Validation Errors:\n{}",
            errors
                .iter()
                .map(|(key, value)| format!("  • {}: {}", key, value))
                .collect::<Vec<_>>()
                .join("\n")
        ))));
    }

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
            }
            Page::Job => {
                self.jobs.clear_entity_state();
                self.current_page = Page::Job;
            }
            Page::Organization => {
                self.organizations.clear_entity_state();
                self.current_page = Page::Organization;
            }
            Page::Settings => {
                self.current_page = Page::Settings;
            }
        }
    }

    pub fn get_job_entity_state(&self) -> &EntityState<Job> {
        &self.jobs
    }

    pub fn get_user_entity_state(&self) -> &EntityState<User> {
        &self.users
    }

    pub fn get_organization_entity_state(&self) -> &EntityState<Organization> {
        &self.organizations
    }

    pub fn get_theme(&self) -> &Theme {
        &self.theme
    }
}
