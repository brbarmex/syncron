#[derive(PartialEq, Eq, Debug)]
pub struct Content {
    pub value: String,
    pub version: String,
}

impl Content {

    #[warn(dead_code)]
    pub fn is_valid(&self) -> bool {
        self.value.len() > 0 || self.version.len() > 0
    }

    #[warn(dead_code)]
    pub fn new(content:String, version:String) -> Content {
        Content{
            value: content,
            version,
        }
    }  
}