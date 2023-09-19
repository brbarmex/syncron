use crate::entity::content::Content;
use crate::service::{file::FileIO, storage::Database};
use sha256::digest;
use std::sync::Arc;
use std::{io::ErrorKind, io::Error};

#[warn(dead_code)]
pub fn perform_backup(
    file_io:  Arc<dyn FileIO>, 
    database: Arc<dyn Database>, 
    path: String,
    file_name: &String) 
    -> Result<(), std::io::Error> {

        let data = file_io
        .read(path)
        .unwrap_or(Vec::default());

        let data = String::from_utf8(data)
        .unwrap_or(String::default());

        if data.is_empty() {
            Error::new(ErrorKind::InvalidData, "the file not contain data");
        }

        let content = Content::new(data, digest(&data));
        if !content.is_valid() {
            Error::new(ErrorKind::Other, "the content_data is invalid");
        }

        let latest_content = database.latest(file_name)?;
        if latest_content.version != content.version {
            database.put(&content, file_name)?;
        }

        Ok(())
}


#[cfg(test)]
mod tests {
    
    use std::{io, path::Path};
    use crate::service;
    use crate::entity::content::Content;

    #[test]
    fn backup_successful_when_everything_succeeds() {

        // Arrange
        let file_io_mock = create_successful_file_io_mock();
        let data_mock = create_successful_database_mock();
        let path = "dummy".to_string();
        let file = "dummy.txt".to_string();

        // Act
        let result = service::snapshot::perform_backup(&file_io_mock, &data_mock, path, &file);

        // Assert
        assert!(result.is_ok());
    }

    #[test]
    fn backup_failure_when_file_read_failed() {

        // Arrange
        let file_io_mock = create_unsuccessful_file_io_mock();
        let data_mock = create_successful_database_mock();
        let path = "dummy".to_string();
        let file = "dummy.txt".to_string();

        // Act
        let result = service::snapshot::perform_backup(&file_io_mock, &data_mock, path, &file);

        // Assert
        assert!(result.is_err());
    }

    fn create_unsuccessful_file_io_mock() -> FileMock {
        FileMock {
            read_mock: || Err(io::Error::new(io::ErrorKind::Other, "failed")),
            write_mock: || Ok(()),
        }
    }

    fn create_successful_file_io_mock() -> FileMock {
        FileMock {
            read_mock: || Ok(vec![1, 2, 3]),
            write_mock: || Ok(()),
        }
    }

    fn create_successful_database_mock() -> DatabaseMock {
        DatabaseMock {
            put_mock: || Ok(()),
            latest_mock: || Ok(Content {
                value: "dummy".to_string(),
                version: "dummy".to_string(),
            }),
        }
    }

    #[derive(Debug)]
    struct FileMock {
        read_mock: fn() -> io::Result<Vec<u8>>,
        write_mock: fn() -> std::io::Result<()>
    }

    impl service::file::FileIO for FileMock  {
        fn read(&self, _path: String) -> std::io::Result<Vec<u8>> {
            (self.read_mock)()
        }

        fn write(&self, _path: &Path, _contents: &[u8]) -> std::io::Result<()> {
            (self.write_mock)()
        }
    }
    
    #[derive(Debug)]
    struct DatabaseMock {
        put_mock: fn()-> Result<(), io::Error>,
        latest_mock: fn()-> Result<Content, io::Error>
    }

    impl service::storage::Database for DatabaseMock {
        fn put(&self, _content: &crate::entity::content::Content, _file: &String) -> Result<(), io::Error> {
            (self.put_mock)()
        }

        fn latest(&self, _id: &String) -> Result<Content, io::Error> {
            (self.latest_mock)()
        }
    }
}
