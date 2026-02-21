pub mod app_message;
pub mod job_message;
pub mod organization_message;
pub mod user_message;

#[derive(Debug, Clone)]
pub enum Message {
    App(app_message::AppMessage),
    User(user_message::UserMessage),
    Job(job_message::JobMessage),
    Organization(organization_message::OrganizationMessage),
}
