use serde_json::Value;
use std::{env, io::Error, io::ErrorKind};

const BASE: &str = "https://youtube.com/results?";

struct Client {
    client: reqwest::Client,
    re_pat: regex::Regex,
    query: Query,
}

// Get results and store each individual one in a struct
struct Video {
    vid_id: String,
    title: String,
    thumb: String,
}
// struct Playlist {
//     ply_id: String,
//     title: String,
//     thumb: String,
// }

// Store all the results in here. Idk why I'm doing this, kinda just assuming it'll make something
// easier in the future
struct Query {
    query: String,
    json: String,
    vid: Vec<Video>,
    // plylst: Vec<Playlist>,
}

#[tokio::main]
async fn main() {
    // there is no arguments provided, IDK how to solve this
    let query: &str = &args[1];
    let query: &str = &args[1];
    let mut local: Client = Client {
        client: reqwest::Client::new(),
        re_pat: regex::Regex::new(r"var ytInitialData =(.*?);</script>").unwrap(),
        query: Query {
            query: "".to_owned(),
            json: "".to_owned(),
            vid: Vec::new(),
            // plylst: Vec::new(),
        },
    };
    let html = local.search(query.to_owned()).await.unwrap();
    let data = local.get_json(html.to_owned()).await.unwrap();
    let json = local.prep_json(data).await;
    local.pop_videos(json).await;
    // let json2: Vec<Value> = serde_json::from_str(&local.prep_json(data).await).unwrap();
    // for data in json2 {
    //     if data["videoRenderer"].is_null() {
    //         continue;
    //     }
    //     println!("{}", data["videoRenderer"]["title"]["runs"][0]["text"]);
    //     println!("{}", data["videoRenderer"]["videoId"]);
    // }
    // let query: &str = "test";
    // let html = local.search(query.to_owned()).await.unwrap();
    // let data = local.get_json(html.to_owned()).await.unwrap();
    // let json2: Vec<Value> = serde_json::from_str(&local.prep_json(data).await).unwrap();
    // println!("{}", data);
    // untyped_example(data.to_owned()).unwrap();
    // let json: Value = serde_json::from_str(&data).unwrap();
    // // println!("{}", json["contents"]["twoColumnSearchResultsRenderer"]["primaryContents"]
    // //         ["sectionListRenderer"]["contents"][0]["itemSectionRenderer"]["contents"].to_owned());
    // let json2: Vec<Value> = serde_json::from_str(&format!(
    //     "{}",
    //     json["contents"]["twoColumnSearchResultsRenderer"]["primaryContents"]
    //         ["sectionListRenderer"]["contents"][0]["itemSectionRenderer"]["contents"]
    //         .to_owned()
    // ))
    // .unwrap();
    // for data in json2.to_owned() {
    //     if data["videoRenderer"].is_null() {
    //         continue;
    //     }
    //     println!("{}", data["videoRenderer"]["title"]["runs"][0]["text"]);
    //     println!("{}", data["videoRenderer"]["videoId"]);
    // }
    local.list_vid().await;
}
// Step by step approach

impl Client {
    /// Query Youtube and get HTML contents
    /// takes the self.query.query and searches it on youtube
    ///
    /// return: Result containing either HTML string or Error
    async fn search(&mut self, query: String) -> Result<String, Error> {
        self.query.query = query.to_owned();
        let res = self
            .client
            .get(format!("{}search_query={}", BASE, query))
            .send()
            .await;
        match res {
            Ok(v) => match v.text().await {
                Ok(w) => Ok(w),
                Err(_) => Err(Error::new(ErrorKind::Other, "Failed to read HTML")),
            },
            Err(_) => Err(Error::new(ErrorKind::Other, "Failed to load page")),
        }
        // res.unwrap().text().await.unwrap()
    }

    async fn get_json(&self, html: String) -> Result<String, Error> {
        let re = self.re_pat.captures(&html);
        match re {
            Some(v) => match v.get(1) {
                Some(w) => Ok(w.as_str().to_owned()),
                None => Err(Error::new(ErrorKind::Other, "Bad HTML")),
            },
            None => Err(Error::new(ErrorKind::Other, "Failed to match")),
        }
    }

    /// Return: String containing new json
    async fn prep_json(&mut self, json_str: String) -> String {
        let parse: Value = serde_json::from_str(&json_str).unwrap();
        let req_parse: Value = serde_json::from_str(&format!(
            "{}",
            parse["contents"]["twoColumnSearchResultsRenderer"]["primaryContents"]
                ["sectionListRenderer"]["contents"][0]["itemSectionRenderer"]["contents"]
                .to_owned()
        ))
        .unwrap();
        self.query.json = format!("{}", req_parse);
        self.query.json.to_owned()
    }

    async fn pop_videos(&mut self, data: String) -> () {
        let reparse: Vec<Value> = serde_json::from_str(&data).unwrap();
        // println!("{}", data["videoRenderer"]["title"]["runs"][0]["text"]);
        // println!("{}", data["videoRenderer"]["videoId"]);
        for data in reparse {
            if data["videoRenderer"].is_null() {
                continue;
            }
            self.query.vid.push(Video {
                vid_id: data["videoRenderer"]["videoId"]
                    .as_str()
                    .unwrap()
                    .to_owned(),
                title: data["videoRenderer"]["title"]["runs"][0]["text"]
                    .as_str()
                    .unwrap()
                    .to_owned(),
                thumb: data["videoRenderer"]["thumbnail"]["thumbnails"][0]["url"]
                    .as_str()
                    .unwrap()
                    .to_owned(),
            });
        }
    }

    async fn list_vid(self) -> () {
        for data in self.query.vid {
            println!(
                r#"Video Title:	{}
Video ID:	{}
Video Thumb:	{}"#,
                data.title, data.vid_id, data.thumb
            );
        }
    }

    // async fn pop_playlists(&mut self, data: String) -> () {}
}
