use crate::entity::content::Content;
use crate::service::{file::FileIO, storage::Database};
use sha256::digest;
use std::{io::ErrorKind, io::Error};

#[warn(dead_code)]
pub fn perform_backup(
    file_io: &dyn FileIO, 
    database: &dyn Database, 
    path: String,
    file_name: &String) 
    -> Result<(), std::io::Error> {

        let data = match file_io.read(path) {
            Ok(res) => {
                match String::from_utf8(res) {
                    Ok(c) => c,
                    Err(err) => return Err(Error::new(ErrorKind::InvalidData, err))
                }
            },
            Err(err) => return Err(err)
        };

        let version_check_sum = digest(&data);
        let content: Content = Content::new(data, version_check_sum);
        if !content.is_valid() {
            return Err(Error::new(ErrorKind::Other, "the content_data is invalid"));
        }

        let latest_content = database.latest(file_name)?;
        if latest_content.version != content.version {
            database.put(&content, file_name)?;
        }

        Ok(())
}
