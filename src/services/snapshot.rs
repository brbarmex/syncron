extern crate reqwest;

use reqwest::Error;
use entities::Backup;

pub struct SnapshotSvc {
    client: reqwest::client
}

impl SnapshotSvc {
    fn new(client: reqwest::client) -> SnapshotSvc {
        SnapshotSvc{
            client
        }
    }

    fn execute() {

        let content_dowloaded: String = String::from("dummy");
        let content_version: String = String::from("sha:x723-456");

        let backup = Backup::new(content_dowloaded, content_version);
        let r = backup.is_valid();
        println!(r);

    }
}