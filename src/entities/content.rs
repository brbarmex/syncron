#[derive(PartialEq, Eq)]
pub struct Content {
    pub value: String,
    pub version: String,
    hash_checksum: String,
}

impl Content {
    pub fn is_valid(&self) -> bool {
        self.value.len() > 0 || self.version.len() > 0 || self.hash_checksum.len() > 0
    }

    pub fn new(content:String, version:String, hash_checksum: String) -> Content {
        Content{
            value: content,
            version,
            hash_checksum,
        }
    }  
}