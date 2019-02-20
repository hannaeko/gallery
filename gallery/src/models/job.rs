use actix_web::actix::Message;
use askama::Template;

use super::schema::jobs;
use crate::error::GalleryError;

pub const STATE_CREATED: &str = "created";
pub const STATE_RUNNING: &str = "running";
pub const STATE_FINISHED: &str = "finished";


#[derive(Insertable, Queryable)]
pub struct Job {
    pub id: String,
    pub name: String,
    pub state: String,
}

#[derive(Template)]
#[template(path = "admin/jobs.html")]
pub struct JobsTemplate {
    pub jobs: Vec<Job>,
}

pub struct CreateJob {
    pub name: String,
}

pub struct ChangeState {
    pub job_id: String,
    pub new_state: String,
}

pub struct GetJobs;

impl Message for CreateJob {
    type Result = Result<String, GalleryError>;
}

impl Message for ChangeState {
    type Result = Result<(), GalleryError>;
}

impl Message for GetJobs {
    type Result = Result<Vec<Job>, GalleryError>;
}
