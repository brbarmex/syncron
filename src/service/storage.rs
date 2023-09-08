use std::{io,io::Error, io::ErrorKind};
use postgres::{Client, NoTls};

use crate::entity::content::Content;

pub trait Database {
    fn put(&self, content: &Content, file: &String) -> Result<(), Error>;
    fn latest(&self, id: &String) -> Result<Content, Error>;
}

#[derive(Debug)]
pub struct Postgres {}

impl Postgres {
    pub fn new() -> Postgres {
        Postgres {  }
    }
}

impl Database for Postgres {
    fn put(&self, content: &Content, file: &String) -> Result<(), Error> {
       
        let mut client: Client = match Client::connect(
            "postgresql://rustdb:rustpwd@localhost/db_snapshot",
            NoTls,
        ) {
            Ok(c) => c,
            Err(err) => return Err(Error::new(ErrorKind::NotConnected, err)),
        };

        let query: &str = "INSERT INTO snapshots (content_value, version, file_id) VALUES ($1, $2, $3) \
        ON CONFLICT (file_id) DO UPDATE SET content_value = EXCLUDED.content_value, \
        version = EXCLUDED.version";

        match client.execute(
            query,
            &[&content.value, &content.version, &file],
        ) {
            Ok(_) => return Ok(()),
            Err(err) => return Err(io::Error::new(ErrorKind::Other, err)),
        };
    }

    fn latest(&self, id: &String) -> Result<Content, Error> {
       
        let mut client: Client = match Client::connect(
            "postgresql://rustdb:rustpwd@localhost/db_snapshot",
            NoTls,
        ) {
            Ok(c) => c,
            Err(err) => return Err(Error::new(ErrorKind::NotConnected, err)),
        };

        let row = match client.query_one("SELECT * FROM snapshots WHERE file_id = $1", &[&id]) {
            Ok(row) => row,
            Err(err) => return Err(Error::new(ErrorKind::NotConnected, err))
        };

        let content= Content{
            value: row.get("content_value"),
            version: row.get("version"),
        };
    
        Ok(content)
    }
}
