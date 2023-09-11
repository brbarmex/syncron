use std::fs::File;
use std::io::Read;
use serde::Deserialize;

//TODO: To learn a mode to create a singleton instance about this, and how to make global access read-only

#[derive(Debug, Deserialize)]
pub struct EnvConfig {
    pub db_str_connection: String,
    pub artefact_path: String,
    pub cron_time: String,
    pub file_name: String
}

impl EnvConfig {

    #[allow(dead_code)]
    pub fn load() -> Result<EnvConfig, Box<dyn std::error::Error>> {

        let mut file = File::open("{YOUR_ASOLUTE_PATH}/syncron/src/env.yml")?;
        let mut yaml_content = String::new();
        file.read_to_string(&mut yaml_content)?;
    
        let config: EnvConfig = serde_yaml::from_str(&yaml_content)?;    
        Ok(config)
    }
}
