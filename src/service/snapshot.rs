use crate::entity;
use sha256::{self, digest};
use std::{path::Path,path::PathBuf,io};

pub trait ContentIO {
    fn read(&self, path: &Path) -> io::Result<Vec<u8>>;
    fn write(&self, path: &Path, contents: &[u8]) -> io::Result<()>;
}

pub trait ContentStorage {
    fn put_content(&self, content: &entity::content::Content) -> Result<(), std::io::Error>;
    fn read_latest(&self) -> Result<entity::content::Content, std::io::Error>;
}

fn perform_backup(
    content_io: &dyn ContentIO, 
    content_storage: &dyn ContentStorage, 
    content_path_file: String) 
    -> Result<(), std::io::Error> {
   
    let path: PathBuf = PathBuf::from(content_path_file);
    let data_from_file_u8: Vec<u8> = content_io.read(&path)?;

    let data_from_file_as_string: String = match String::from_utf8(data_from_file_u8) {
        Ok(content) => content,
        Err(err) => return Err(io::Error::new(io::ErrorKind::InvalidData, err))
    };

    let content_version_checksum: String = digest(&data_from_file_as_string);    
    let content_new: entity::content::Content = entity::content::Content::new(data_from_file_as_string, content_version_checksum);

    if !content_new.is_valid() {
        return Err(io::Error::new(io::ErrorKind::Other, "the content_data is invalid"));
    }

    let content_latest: entity::content::Content = content_storage.read_latest()?;
    if content_latest.version != content_new.version {
        content_storage.put_content(&content_new)?;
    }

    Ok(())

}
