use sha256::{self, digest};
use std::path::PathBuf;
use std::u8;

use std::io;
use std::path::Path;

use crate::entity;


pub trait FileIO {
    fn read(&self, path: &Path) -> io::Result<Vec<u8>>;
    fn write(&self, path: &Path, contents: &[u8]) -> io::Result<()>;
}

pub struct ActionReadIO{}
impl FileIO for ActionReadIO {
    fn read(&self, path: &Path) -> io::Result<Vec<u8>> {
        let data = std::fs::read(path)?;
        Ok(data)
    }

    fn write(&self, path: &Path, contents: &[u8]) -> io::Result<()> {
        std::fs::write(path, contents)?;
        Ok(())
    }
}

fn perform_backup(file_io: &dyn FileIO, path_file: String) -> Result<(), std::io::Error> {
   
    let path = PathBuf::from(path_file);
    let data = file_io.read(&path)?;

    let content_data: String = match String::from_utf8(data) {
        Ok(content) => content,
        Err(err) => {
            return Err(io::Error::new(io::ErrorKind::InvalidData, err));
        }
    };

    let content_version = digest(&content_data);
    let content = entity::content::Content::new(content_data, content_version);

    if !content.is_valid() {
        return Err(io::Error::new(io::ErrorKind::Other, "the content_data is invalid"));
    }

    Ok(())

}
