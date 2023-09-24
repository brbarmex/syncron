use std::{io, path::Path, path::PathBuf};

pub trait FileIO {
    fn read(&self, path: String) -> io::Result<Vec<u8>>;
    fn write(&self, path: &Path, contents: &[u8]) -> io::Result<()>;
}

#[derive(Debug)]
pub struct IoFile();

impl IoFile {

    #[warn(dead_code)]
    pub fn new() -> Self{
        IoFile()
    }
}

impl Default for IoFile {
    fn default() -> Self {
        Self::new()
    }
}

impl FileIO for IoFile {
    fn read(&self, path: String) -> io::Result<Vec<u8>> {
        let path: PathBuf = PathBuf::from(path);
        let data: Vec<u8> = std::fs::read(path)?;
        Ok(data)
    }

    fn write(&self, path: &Path, contents: &[u8]) -> io::Result<()> {
        std::fs::write(path, contents)?;
        Ok(())
    }
}

pub fn new() -> IoFile{
    IoFile()
}

