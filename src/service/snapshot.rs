use reqwest::{self, blocking::Response, Error, blocking::get};
use sha256::{self, digest};


pub fn fetch_data(url: &str) -> Result<Response, Error> {
    Ok(get(url)?)
}