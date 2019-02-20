use actix_web::actix::Handler;
use uuid;
use diesel;
use diesel::prelude::*;

use crate::models::db::DbExecutor;
use crate::models::job::{Job, CreateJob, ChangeState, GetJobs, self};
use crate::error::GalleryError;

impl Handler<CreateJob> for DbExecutor {
    type Result = Result<String, GalleryError>;

    fn handle(&mut self, msg: CreateJob, _ctx: &mut Self::Context) -> Self::Result {
        use crate::models::schema::jobs;

        let uuid = uuid::Uuid::new_v4().to_string();

        let new_job = Job {
            id: uuid,
            name: msg.name,
            state: job::STATE_CREATED.to_string(),
        };

        diesel::insert_into(jobs::table)
            .values(&new_job)
            .execute(&self.conn.get().unwrap())?;

        debug!("New job {} created.", new_job.id);

        Ok(new_job.id)
    }
}

impl Handler<ChangeState> for DbExecutor {
    type Result = Result<(), GalleryError>;

    fn handle(&mut self, msg: ChangeState, _ctx: &mut Self::Context) -> Self::Result {
        use crate::models::schema::jobs::dsl::*;

        diesel::update(jobs.find(&msg.job_id))
            .set(state.eq(&msg.new_state))
            .execute(&self.conn.get().unwrap())?;

        debug!("Changed state of job {:?} to {:?}.", msg.job_id, msg.new_state);
        Ok(())
    }
}

impl Handler<GetJobs> for DbExecutor {
    type Result = Result<Vec<Job>, GalleryError>;

    fn handle(&mut self, _msg: GetJobs, _ctx: &mut Self::Context) -> Self::Result {
        use crate::models::schema::jobs::dsl::*;

        Ok(jobs.load::<Job>(&self.conn.get().unwrap())?)
    }
}
