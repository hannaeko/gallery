use actix_web::actix::{Actor, Addr, SyncContext, SyncArbiter, Handler};
use uuid;

use diesel;
use diesel::result::Error as DieselError;
use diesel::prelude::*;
use diesel::connection::Connection;

use super::album::{NewAlbum, CreateAlbum, GetAlbumId, GetRootAlbumId};

pub struct DbExecutor(SqliteConnection);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

pub fn init(db_url: String) -> Addr<DbExecutor> {
    SyncArbiter::start(3, move || {
        DbExecutor(SqliteConnection::establish(&db_url)
            .expect("Failed to establish connection to the database"))
    })
}

impl Handler<CreateAlbum> for DbExecutor {
    type Result = Result<String, DieselError>;

    fn handle(&mut self, msg: CreateAlbum, _ctx: &mut Self::Context) -> Self::Result {
        use super::schema::albums;

        let uuid = uuid::Uuid::new_v4().to_string();

        let new_album = NewAlbum {
            id: uuid,
            name: msg.name,
            parent_album_id: msg.parent_album_id,
        };

        diesel::insert_into(albums::table)
            .values(&new_album)
            .execute(&self.0)?;

        debug!("Inserting new album in database, \"{}\" -> {}", new_album.name, new_album.id);

        Ok(new_album.id)
    }
}

impl Handler<GetAlbumId> for DbExecutor {
    type Result = Result<Option<String>, DieselError>;

    fn handle(&mut self, msg: GetAlbumId, _ctx: &mut Self::Context) -> Self::Result {
        use super::schema::albums::dsl::*;

        let album_id = albums
            .filter(parent_album_id.eq(&msg.parent_album_id))
            .filter(name.eq(&msg.name))
            .select(id)
            .limit(1)
            .load::<String>(&self.0)?
            .pop();

        Ok(album_id)
    }
}

impl Handler<GetRootAlbumId> for DbExecutor {
    type Result = Result<Option<String>, DieselError>;

    fn handle(&mut self, _msg: GetRootAlbumId, _ctx: &mut Self::Context) -> Self::Result {
        use super::schema::albums::dsl::*;

        let album_id = albums
            .filter(parent_album_id.is_null())
            .select(id)
            .load::<String>(&self.0)?
            .pop();

        Ok(album_id)
    }
}
