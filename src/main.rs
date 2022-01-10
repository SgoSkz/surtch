use serde_json::Value;
use std::{env, io::Error, io::ErrorKind};

// I swear, this will hopefully, probably never change I think, maybe...
const BASE: &str = "https://youtube.com/results?";

// Coz no make client again and no compile regex again
/// Created for the purpose of constants and not recompiling the regex. While this is a script that
/// only runs once, the full purpose is for a TUI that utilizes the client more than once
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

// For the future
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
    // Get command line arguments and store first one as query. I am well aware this breaks when
    // there is no arguments provided, IDK how to solve this
    let args: Vec<String> = env::args().collect();
    let query: &str = &args[1];

    // Create the thing that does the thing
    let mut local: Client = Client {
        client: reqwest::Client::new(),
        // NOTE: Part 1, Important Probably
        // This right here is prob the most important part of scraping the videos. The regex finds
        // the section of the HTML that contains basically all of the page data as a json. See
        // prep_json to see the next part of preparing the json
        re_pat: regex::Regex::new(r"var ytInitialData =(.*?);</script>").unwrap(),
        query: Query {
            query: "".to_owned(),
            json: "".to_owned(),
            vid: Vec::new(),
            // plylst: Vec::new(),
        },
    };

    // Unimportant stuff, just jump to definition
    let html = local.search(query.to_owned()).await.unwrap();
    let data = local.get_json(html.to_owned()).await.unwrap();
    let json = local.prep_json(data).await;
    local.pop_videos(json).await;
    local.list_vid().await;
}

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
        self.query.json = format!("{}", req_parse);
        self.query.json.to_owned()
    }

    /// Populate video vector
    async fn pop_videos(&mut self, data: String) -> () {
        let reparse: Vec<Value> = serde_json::from_str(&data).unwrap();
        for data in reparse {
            if data["videoRenderer"].is_null() {
                continue;
            }
            self.query.vid.push(Video {
                // NOTE: Part 3, Finding the video stuff
                // This is the ID, AKA, the part after https://youtube.com/watch?v=[THIS PART]
                vid_id: data["videoRenderer"]["videoId"]
                    .as_str()
                    .unwrap()
                    .to_owned(),
                // This is the title of the video
                title: data["videoRenderer"]["title"]["runs"][0]["text"]
                    .as_str()
                    .unwrap()
                    .to_owned(),
                // This is the thumbnail link of the video
                thumb: data["videoRenderer"]["thumbnail"]["thumbnails"][0]["url"]
                    .as_str()
                    .unwrap()
                    .to_owned(),
            });
        }
    }

    /// Progressively getting lazier
    /// Lists the videos inside the video vector
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

}
