#[derive(PartialEq, Eq)]
pub struct Backup {
    pub content: String,
    pub version: String,
}

impl Backup {
    pub fn is_valid(&self) -> bool {
        self.content.len() > 0 || self.version.len() > 0
    }

    pub fn new(content:String, version:String) -> Backup {
        Backup{
            content,
            version,
        }
    }  
}