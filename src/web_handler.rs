use reqwest::{Client, Response};

// TODO: Implement API for playlists and channels

const BODY: &str = r#"{"context": {"client": {"clientName": "ANDROID",
                 "clientVersion": "16.49", "hl": "en", "timeZone": "UTC",
                 "utcOffsetMinutes": 0}}, "maxResults": "50", "#;

/// Handles building a request so the user doesn't have to
#[derive(Clone)]
pub struct WebHandler {
    client: Client,
    // cookies:Cookies,
    // data:Data,
    url: String,
    body: String,
    // headers:Headers,
}

impl WebHandler {

    /// Create a WebHandler EZPZ
    pub fn new() -> WebHandler {
        return WebHandler {
            client: Client::new(),
            // This is actually specific to videos
            url: "https://www.youtube.com/youtubei/v1/search".to_string(),
            body: "".to_string(),
        };
    }

    /// Send a post request form created by the WebHandler
    pub async fn send(self) -> Result<Response, reqwest::Error> {
        self.client
            .post(self.url)
            .query(&[("key", "AIzaSyAO_FJ2SlqU8Q4STEHLGCilw_Y9_11qcW8"), ("maxResults", "50")])
            .header("X-YouTube-Client-Name", "3")
            .header("X-YouTube-Client-Version", "16.49")
            .header("Origin", "https://www.youtube.com")
            .header("content-type", "application/json")
            .body(self.body)
            .send()
            .await
    }

    /// Help create the body using query param
    pub fn search(&mut self, query: &str) -> WebHandler {
        self.body = BODY.to_string() + &format!(r#""query": "{}"}}"#, query);
        return self.clone();
    }

    /// Help create the body using token param
    pub fn continuation(&mut self, token: &str) -> WebHandler {
        self.body = BODY.to_string() + &format!(r#""continuation": {}}}"#, token);
        // println!("{}", BODY.to_string() + &format!(r#""continuation": {}}}"#, token));
        return self.clone();
    }

    // pub async create_body(
}

