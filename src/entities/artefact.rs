extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Data {
    pub length: i32,
    pub text: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Obj {
    pub class: String,
    pub age: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Item {
    pub min_position: i32,
    pub has_more_items: bool,
    pub items_html: String,
    pub new_latent_count: i32,
    pub data: Data,
    #[serde(rename = "numericalArray")]
    pub numerical_array: Vec<i32>,
    #[serde(rename = "StringArray")]
    pub string_array: Vec<String>,
    #[serde(rename = "multipleTypesArray")]
    pub multiple_types_array: String,
    #[serde(rename = "objArray")]
    pub obj_array: Vec<Obj>,
}

