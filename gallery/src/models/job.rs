use actix_web::actix::Message;

use super::schema::jobs;
use crate::error::GalleryError;

#[derive(Insertable)]
pub struct Job {
    pub id: String,
    pub name: String,
    pub state: String,
}

pub struct CreateJob {
    pub name: String,
    pub state: String,
}

pub struct ChangeState {
    pub job_id: String,
    pub new_state: String,
}

impl Message for CreateJob {
    type Result = Result<String, GalleryError>;
}

impl Message for ChangeState {
    type Result = Result<(), GalleryError>;
}
