use reqwest::{self, blocking::Response, Error, blocking::get};
use sha256::{self, digest};

use crate::entities;

pub enum SnapshotErr {
    ErrWhenFetchData,
    ErrStatusCodeInvalid,
    ErrWhenParseBodyResultToBytes,
    ErrContentInvalid
}

pub fn make_backup() -> Result<bool,SnapshotErr> {

    let url = "";
    let result: Result<Response, Error> = fetch_data(&url);

    if result.is_err() {
        return Err(SnapshotErr::ErrWhenFetchData);
    }

    let response: Response = result.unwrap();
    if !response.status().is_success() {
        return Err(SnapshotErr::ErrStatusCodeInvalid);
    }

    let resp_bytes = response.text();
    if resp_bytes.is_err() {
        return Err(SnapshotErr::ErrWhenParseBodyResultToBytes);
    }

    let resp_str = resp_bytes.unwrap();
    let data = resp_str.clone();
    let data_version = digest(resp_str.clone());
    let content: entities::content::Content =  entities::content::Content::new(data, data_version);

    if !content.is_valid() {
        return Err(SnapshotErr::ErrContentInvalid);
    }

   Ok(true)

}

pub fn fetch_data(url: &str) -> Result<Response, Error> {
    Ok(get(url)?)
}