// use reqwest::Client;
use clap::{App, Arg, SubCommand};
// use surtch::gui;
use surtch::results;
use surtch::web_handler;

#[tokio::main]
async fn main() {
    let mut verbose: bool = false;
    let matches = App::new("surtch utility")
        .version("1.0")
        .author("Kevin K. <kbknapp@gmail.com>")
        .about("Does awesome things")
        .arg(
            Arg::with_name("search")
                .short("s")
                .long("search")
                .value_name("query")
                .help("Sets a query for surtch")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .subcommand(
            SubCommand::with_name("test")
                .about("controls testing features")
                .version("1.3")
                .author("Someone E. <someone_else@other.com>")
                .arg(
                    Arg::with_name("debug")
                        .short("d")
                        .help("print debug information verbosely"),
                ),
        )
        .get_matches();
    let config = matches.value_of("search").unwrap_or("");
    if config.is_empty() {
        println!("-s was not used. Stopping.");
        return;
    }
    if matches.is_present("v") {
        verbose = true;
    }

    // Initial Search
    let mut handler = web_handler::WebHandler::new();
    if verbose {
        println!("Creating webhandler");
    }
    let mut ret_json = handler.search(config).send();
    if verbose {
        println!("Searching");
    }

    let mut json1:String;
    let mut res: results::JSONResult;

    // Results and continuation
    json1 = ret_json
        .await
        .unwrap()
        .text_with_charset("utf8")
        .await
        .unwrap();
    res = results::JSONResult::new(&json1);
    if verbose {
        println!("Parsing data");
    }
    for i in res.videos() {
        println!(
            "Title: {}\tVidID: {}\tThmb:  {}\x0f",
            i.title, i.id, i.thumbnail
        );
    }
    // println!("{}", res.cont_token());
    for _ in 0..2 {
        ret_json = handler.continuation(&res.cont_token()).send();
        json1 = ret_json
            .await
            .unwrap()
            .text_with_charset("utf8")
            .await
            .unwrap();
        res = results::JSONResult::new(&json1);
        for i in res.videos() {
            println!(
                "Title: {}\tVidID: {}\tThmb:  {}\x0f",
                i.title, i.id, i.thumbnail
            );
        }
    }
    // // println!("{}", res.cont_token());
    // ret_json = handler.continuation(&res.cont_token()).send();
    // let json3 = &(ret_json
    //     .await
    //     .unwrap()
    //     .text_with_charset("utf8")
    //     .await
    //     .unwrap());
    // let res: results::JSONResult = results::JSONResult::new(json3);
    // for i in res.videos() {
    //     println!(
    //         "Title: {}\tVidID: {}\tThmb:  {}\x0f",
    //         i.title, i.id, i.thumbnail
    //     );
    // }
    // // println!("{}", res.cont_token());
    // // println!("{}", g.await.unwrap().text_with_charset("utf8").await.unwrap());
}
