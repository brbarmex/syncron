use crate::daemon;
use crate::entity::content::Content;
use crate::repository::snapshot_repository;
use sha256::digest;
use std::{io::Error, io::ErrorKind};
use crate::iofile::iofile;

pub struct Backup{
    file: Box<dyn iofile::FileIO>,
    repository: Box<dyn snapshot_repository::SnapshotRepository>,
}

impl daemon::DaemonJob for Backup {
    fn execute(&self) {
        todo!()
    }
}

impl Backup{


    pub fn new(file: Box<dyn iofile::FileIO>, db: Box<dyn snapshot_repository::SnapshotRepository>) -> Self {
        Backup { file, repository: db }
    }

    pub fn perform_copy(&self, path: String, file_name: &String) -> Result<(), std::io::Error> {
        let data = self.file.read(path).unwrap_or(Vec::default());
        let data = String::from_utf8(data).unwrap_or(String::default());
        if data.is_empty() {
           return Err(Error::new(ErrorKind::InvalidData, "the file not contain data"));
        }

        let checksum = digest(&data);
        let content = Content::new(data, checksum);
        if !content.is_valid() {
            return Err(Error::new(ErrorKind::Other, "the content_data is invalid"));
        }

        let latest_content = self.repository.latest(file_name)?;
        if latest_content.version != content.version {
            self.repository.put(&content, file_name)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use crate::{entity::content::Content, usecase::backup::mock};
    use crate::usecase;
    use std::io;

    #[test]
    fn backup_failure_when_file_read_failed() {

        // Arrange
        let file_io_mock = Box::<mock::FileMock>::new(mock::FileMock{
            read_mock: || Err(io::Error::new(io::ErrorKind::InvalidData, "failed")),
            write_mock: || Ok(()),
        });

        let database_mock = Box::<mock::DatabaseMock>::new(mock::DatabaseMock {
            put_mock: || Ok(()),
            latest_mock: || {
                Ok(Content {
                    value: String::from("mock"),
                    version: String::from("mock"),
                })
            },
        });

        let path = String::from("fake");
        let file = String::from("fake");

        // Act
        let service = usecase::backup::Backup::new(file_io_mock, database_mock);
        let result = service.perform_copy(path, &file);

        // Assert
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::InvalidData);
    }

    #[test]
    fn backup_successful_when_everything_succeeds() {
        // Arrange
        let file_io_mock = Box::<mock::FileMock>::new(mock::FileMock{
            read_mock: || Ok(vec![1, 2, 3]),
            write_mock: || Ok(()),
        });

        let database_mock = Box::<mock::DatabaseMock>::new(mock::DatabaseMock {
            put_mock: || Ok(()),
            latest_mock: || {
                Ok(Content {
                    value: String::from("mock"),
                    version: String::from("mock"),
                })
            },
        });

        let path = String::from("fake");
        let file = String::from("fake");

        // Act
        let service = usecase::backup::Backup::new(file_io_mock, database_mock);
        let result = service.perform_copy(path, &file);

        // Assert
        assert!(result.is_ok());
    }
}

pub(crate) mod mock {

    use crate::{entity::content::Content, repository};
    use crate::usecase;
    use crate::iofile::iofile;
    use std::{io, path::Path};

    #[derive(Debug)]
    pub(crate) struct DatabaseMock {
        pub(crate) put_mock: fn() -> Result<(), io::Error>,
        pub(crate) latest_mock: fn() -> Result<Content, io::Error>,
    }

    impl repository::snapshot_repository::SnapshotRepository for DatabaseMock {
        fn put(
            &self,
            _content: &crate::entity::content::Content,
            _file: &String,
        ) -> Result<(), io::Error> {
            (self.put_mock)()
        }

        fn latest(&self, _id: &String) -> Result<Content, io::Error> {
            (self.latest_mock)()
        }
    }

    #[derive(Debug)]
    pub(crate) struct FileMock {
        pub(crate) read_mock: fn() -> io::Result<Vec<u8>>,
        pub(crate) write_mock: fn() -> std::io::Result<()>,
    }

    impl iofile::FileIO for FileMock {
        fn read(&self, _path: String) -> std::io::Result<Vec<u8>> {
            (self.read_mock)()
        }

        fn write(&self, _path: &Path, _contents: &[u8]) -> std::io::Result<()> {
            (self.write_mock)()
        }
    }
}
