use mockito::{Server, ServerGuard};
use super::*;

#[cfg(test)]
mod tests {
    use crate::service;


    #[test]
    fn it_must_fetch_data() {
        
        let json_str = r#"{
            "min_position": 3,
            "has_more_items": true,
            "items_html": "Car",
            "new_latent_count": 9,
            "data": {
                "length": 23,
                "text": "Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."
            },
            "numericalArray": [24, 33, 24, 27, 32],
            "StringArray": ["Oxygen", "Oxygen", "Nitrogen", "Carbon"],
            "multipleTypesArray": "Hello",
            "objArray": [
                { "class": "lower", "age": 6 },
                { "class": "lower", "age": 2 },
                { "class": "upper", "age": 5 },
                { "class": "upper", "age": 0 },
                { "class": "lower", "age": 9 }
            ]
        }"#;

        //mock server
        let mut server: mockito::ServerGuard = mockito::Server::new();
        let mut url: String = server.url();
        url.push_str("/content");

        server
            .mock("GET", "/content")
            .with_status(200)
            .with_body(json_str)
            .create();

        let result = service::snapshot::fetch_data(&url);
        assert!(result.is_ok(), "Was expected Ok result");

        let response = result.unwrap();
        assert_eq!(response.status().is_success(), true, "Expected status code 200")

    }
}
