use serde_json::{from_str, Value};

#[derive(Clone)]
pub struct JSONResult {
    pub json: String,
}

impl JSONResult {
    pub fn new(json: &str) -> JSONResult {
        JSONResult {
            json: format!("{}", json),
        }
    }

    /// Grab the continuation token
    pub fn cont_token(&self) -> String {
        // This determines which script to used because the API for regular data and continuation
        // data is not the same
        let sift1: Value = from_str(&self.json).unwrap();
        let token: Value;
        if sift1["contents"].is_null() {
            token = from_str(&format!(
                "{}",
                sift1["continuationContents"]["sectionListContinuation"]["continuations"][0]
                    ["nextContinuationData"]["continuation"]
                    .to_owned()
            ))
            .unwrap();
        } else {
            token = from_str(&format!(
                "{}",
                sift1["contents"]["sectionListRenderer"]["continuations"][0]
                    ["nextContinuationData"]["continuation"]
                    .to_owned()
            ))
            .unwrap();
        }
        return format!("{}", token);
    }

    /// Grab the videos
    pub fn videos(&self) -> Vec<Video> {
        let sift1: Value = from_str(&self.json).unwrap();
        let parse: Vec<Value>;
        if sift1["contents"].is_null() {
            parse = from_str(&format!(
                "{}",
                sift1["continuationContents"]//["twoColumnSearchResultsRenderer"]["primaryContents"]
                    ["sectionListContinuation"]["contents"][0]["itemSectionRenderer"]["contents"]
                    .to_owned()
            ))
            .unwrap();
        } else {
            parse = from_str(&format!(
            "{}",
            sift1["contents"]//["twoColumnSearchResultsRenderer"]["primaryContents"]
                ["sectionListRenderer"]["contents"][0]["itemSectionRenderer"]["contents"]
                .to_owned()
            ))
            .unwrap();
        }

        let mut list: Vec<Video> = vec![];
        for data in parse {
            // println!("{}", data);
            if data["compactVideoRenderer"].is_null() {
                continue;
            }
            let uniform = &data["compactVideoRenderer"];
            list.push(Video {
                id: uniform["videoId"].as_str().unwrap().to_owned(),
                title: uniform["title"]["runs"][0]["text"]
                    .as_str()
                    .unwrap()
                    .to_owned(),
                thumbnail: uniform["thumbnail"]["thumbnails"][2]["url"]
                    .as_str()
                    .unwrap()
                    .to_owned(),
            });
        }
        return list;
    }
}

pub struct Video {
    pub id: String,
    pub title: String,
    pub thumbnail: String,
}

pub struct Playlist {
    pub id: String,
    pub thumbnail: String,
}

pub struct Channel {
    pub id: String,
    pub name: String,
    pub thumbnail: String,
}
