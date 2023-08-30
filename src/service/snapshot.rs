use reqwest::{self, blocking::Response, Error, blocking::get};


pub fn download_snapshot(url: &str) -> Result<Response, Error> {
    let result: Response = get(url)?;
    Ok(result)
}