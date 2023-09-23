use std::{io, io::Error, io::ErrorKind};

use r2d2_postgres::{r2d2::Pool, PostgresConnectionManager};
use tokio_postgres::NoTls;

use crate::entity::content::Content;

pub trait SnapshotRepository {
    fn put(&self, content: &Content, file: &String) -> Result<(), Error>;
    fn latest(&self, id: &String) -> Result<Content, Error>;
}

pub struct Repository {
    pool: Pool<PostgresConnectionManager<NoTls>>,
}

impl Repository {
    pub fn new(pool: Pool<PostgresConnectionManager<NoTls>>) -> Self {
        Repository { pool }
    }
}

impl SnapshotRepository for Repository {
    fn put(&self, content: &Content, file: &String) -> Result<(), Error> {
        let query: &str =
            "INSERT INTO snapshots (content_value, version, file_id) VALUES ($1, $2, $3) \
            ON CONFLICT (file_id) DO UPDATE SET content_value = EXCLUDED.content_value, \
            version = EXCLUDED.version";

        if let Err(err) = self
            .pool
            .get()
            .unwrap()
            .execute(query, &[&content.value, &content.version, &file])
        {
            return Err(io::Error::new(ErrorKind::Other, err));
        }

        return Ok(());
    }

    fn latest(&self, id: &String) -> Result<Content, Error> {
        let query = "SELECT * FROM snapshots WHERE file_id = $1";

        match self.pool.get().unwrap().query_one(query, &[&id]) {
            Ok(content) => {
                return Ok(Content {
                    value: content.get("content_value"),
                    version: content.get("version"),
                });
            }
            Err(err) => return Err(Error::new(ErrorKind::NotFound, err)),
        }
    }
}
