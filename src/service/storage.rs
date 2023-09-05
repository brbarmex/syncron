use std::{io, path::Path};
use postgres::{Client, NoTls};

use crate::service::snapshot::ContentIO;
use super::snapshot::ContentStorage;


pub struct FileIO {}
impl ContentIO for FileIO {
    fn read(&self, path: &Path) -> io::Result<Vec<u8>> {
        let data = std::fs::read(path)?;
        Ok(data)
    }

    fn write(&self, path: &Path, contents: &[u8]) -> io::Result<()> {
        std::fs::write(path, contents)?;
        Ok(())
    }
}

pub struct PgService {}
impl ContentStorage for PgService {
    fn put_content(&self, content: &crate::entity::content::Content) -> Result<(), std::io::Error> {
       
        let mut client: Client = match Client::connect(
            "postgresql://your_username:your_password@localhost/your_database",
            NoTls,
        ) {
            Ok(c) => c,
            Err(err) => return Err(io::Error::new(io::ErrorKind::NotConnected, err)),
        };

        match client.execute(
            "INSERT INTO content (content_value, version) VALUES ($1, $2)",
            &[&content.value, &content.version],
        ) {
            Ok(_) => return Ok(()),
            Err(err) => return Err(io::Error::new(io::ErrorKind::Other, err)),
        };
    }

    fn read_latest(&self) -> Result<crate::entity::content::Content, std::io::Error> {
       
        let mut client: Client = match Client::connect(
            "postgresql://your_username:your_password@localhost/your_database",
            NoTls,
        ) {
            Ok(c) => c,
            Err(err) => return Err(io::Error::new(io::ErrorKind::NotConnected, err)),
        };

        let row = match client.query_one("SELECT * FROM content ORDER BY id DESC LIMIT 1", &[]) {
            Ok(row) => row,
            Err(err) => return Err(io::Error::new(io::ErrorKind::NotConnected, err)),
        };

        // Parse the retrieved data into a Content struct
        let content = crate::entity::content::Content{
            value: row.get("content_value"),
            version: row.get("version"),
        };
    
        Ok(content)
    }
}
