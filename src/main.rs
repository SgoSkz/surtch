use serde_json::Value;
use std::{env, io::Error, io::ErrorKind};

// I swear, this will hopefully, probably never change I think, maybe...
const BASE: &str = "https://youtube.com/results?";

enum QResults {
    Video(Vec<Video>),
    Plylst(Vec<Playlist>),
}

// Get results and store each individual one in a struct
struct Video {
    vid_id: String,
    title: String,
    thumb: String,
}

// For the future
struct Playlist {
    ply_id: String,
    title: String,
    thumb: String,
}

// Coz no make client again and no compile regex again
/// Created for the purpose of constants and not recompiling the regex. While this is a script that
/// only runs once, the full purpose is for a TUI that utilizes the client more than once
struct Client {
    client: reqwest::Client,
    re_pat: regex::Regex,
    // query: QueryResults,
}

impl Client {
    // async fn search(query_results: QResults) -> QResults {
    //     QResults { Video: }
    // }

    /// Query Youtube and get HTML contents
    /// takes the self.query.query and searches it on youtube
    ///
    /// return: Result containing either HTML string or Error
    async fn get_html(&mut self, query: String) -> Result<String, Error> {
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
    }

    /// Get the json data from the HTML provided
    ///
    /// return: Result containing either the json string or Error
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

    /// Prep the json for iteration and whatnot
    /// 
    /// Return: String containing new json string
    async fn prep_json(&mut self, json_str: String) -> String {
        // NOTE: Part 2, Finding the good stuff
        // I forget why I did it like this, but basically, convert string into json
        let parse: Value = serde_json::from_str(&json_str).unwrap();

        // Cut down the data to this section of the json data.
        // ["contents"]["twoColumnSearchResultsRenderer"]["primaryContents"]["sectionListRenderer"]["contents"][0]["itemSectionRenderer"]["contents"]
        // This will give a list of "videoRenderer" objects which contains data on the list of
        // things that come up from a youtube search. See pop_videos for the next part
        let req_parse: Value = serde_json::from_str(&format!(
            "{}",
            parse["contents"]["twoColumnSearchResultsRenderer"]["primaryContents"]
                ["sectionListRenderer"]["contents"][0]["itemSectionRenderer"]["contents"]
                .to_owned()
        ))
        .unwrap();
        format!("{}", req_parse)
    }

    async fn get_results(&self, json: String, query_results: QResults) -> QResults {
        let reparse: Vec<Value> = serde_json::from_str(&json).unwrap();
        match query_results {
            QResults::Video(mut v) => { 
                for data in reparse {
                    if data["videoRenderer"].is_null() {
                        continue;
                    }
                    let uniform = &data["videoRenderer"];
                    v.push(Video {
                        // NOTE: Part 3, Finding the video stuff
                        // This is the ID, AKA, the part after https://youtube.com/watch?v=[THIS PART]
                        vid_id: uniform["videoId"]
                            .as_str()
                            .unwrap()
                            .to_owned(),
                        // This is the title of the video
                        title: uniform["title"]["runs"][0]["text"]
                            .as_str()
                            .unwrap()
                            .to_owned(),
                        // This is the thumbnail link of the video
                        thumb: uniform["thumbnail"]["thumbnails"][0]["url"]
                            .as_str()
                            .unwrap()
                            .to_owned(),
                    });
                }
                QResults::Video(v)
            },
            QResults::Plylst(mut p) => { 
                for data in reparse {
                    if data["playlistRenderer"].is_null() {
                        continue;
                    }
                    let uniform = &data["playlistRenderer"];
                    p.push(Playlist {
                        // NOTE: Part 3, Finding the video stuff
                        // This is the ID, AKA, the part after https://youtube.com/watch?v=[THIS PART]
                        ply_id: uniform["playlistId"]
                            .as_str()
                            .unwrap()
                            .to_owned(),
                        // This is the title of the video
                        title: uniform["title"]["simpleText"]
                            .as_str()
                            .unwrap()
                            .to_owned(),
                        // This is the thumbnail link of the video
                        // thumb: uniform["thumbnail"]["thumbnails"][0]["url"]
                        //     .as_str()
                        //     .unwrap()
                        //     .to_owned(),
                        thumb: "".to_owned(),
                    });
                }
                QResults::Plylst(p)
            },
        }
    }

    /// Progressively getting lazier
    /// Lists the videos inside the video vector
    async fn list(&self, query_results: QResults) -> () {
        match query_results {
            QResults::Video(v) => {
                for data in v {
                    println!(r#"
Video Title:    {}
Video ID:       {}
Video Thumb:    {}"#, data.title, data.vid_id, data.thumb);
                }
            },
            QResults::Plylst(p) => {
                for data in p {
                    println!(r#"
Playlist Title: {}
Playlist ID:    {}
Playlist Thumb: {}"#, data.title, data.ply_id, data.thumb);
                }
            },
        }
    }
}

#[tokio::main]
async fn main() {
    // Will use clap in the future
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        return ()
    }
    let query: &str = &args[1];

    // Create the thing that does the thing
    let mut local: Client = Client {
        client: reqwest::Client::new(),
        // NOTE: Part 1, Important Probably
        // This right here is prob the most important part of scraping the videos. The regex finds
        // the section of the HTML that contains basically all of the page data as a json. See
        // prep_json to see the next part of preparing the json
        re_pat: regex::Regex::new(r"var ytInitialData =(.*?);</script>").unwrap(),
        // query: QueryResults {
        //     // json: "".to_owned(),
        //     vid: Vec::new(),
        //     // plylst: Vec::new(),
        // },
    };

    // Unimportant stuff, just jump to definition
    let html = local.get_html(query.to_owned()).await.unwrap();
    let data = local.get_json(html.to_owned()).await.unwrap();
    let json = local.prep_json(data).await;
    // local.pop_videos(json.to_owned()).await;
    let videos = local.get_results(json.to_owned(), QResults::Video(Vec::new())).await;
    local.list(videos).await;
    let playlists = local.get_results(json.to_owned(), QResults::Plylst(Vec::new())).await;
    local.list(playlists).await;
}

