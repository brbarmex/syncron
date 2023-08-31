use reqwest::{self, blocking::Response, Error, blocking::get};
use sha256::{self, digest};


fn execute() {

    let data = String::from("");
    let _r = digest(data);

    let _result = match fetch_data(&String::from("")) {
        Ok(_) => {},
        Err(_) => {},
    };
}


pub fn fetch_data(url: &str) -> Result<Response, Error> {
    Ok(get(url)?)
}