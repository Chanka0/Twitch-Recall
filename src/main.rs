use std::str::SplitTerminator;

use reqwest;
use scraper::{self, ElementRef, Html, Selector};
use regex::{self, Regex};
use chrono::{TimeZone, Utc};
use sha1_smol;

struct VOD {
    username: String,
    stream_id: i64,
    timestamp: i64
}

//TODO Fix the horrible str/String intermingling. Standardization is needed so the inefficient conversions stop happening.

#[tokio::main]
async fn main(){

    let mut url = String::new();
    println!("Enter VOD URL: ");
    std::io::stdin().read_line(&mut url).unwrap();

    let site_type = parse_url(&url);
    let list = req_workaround(&"https://raw.githubusercontent.com/Chanka0/Twitch-Recall/master/domains.txt".to_string()).await.unwrap().text().await.unwrap();
    let domains = list.split_terminator("\n");

    if site_type == -1 {
        println!("URL was invalid. Make sure to use the supported sites, and input a URL like this:\nhttps://twitchtracker.com/channel/streams/stream_id");
    } 
    else if site_type == 1 
    {
        let vod = get_stream_tt(&url).await;
        scrape_vods(vod, domains).await;
    }

}

async fn scrape_vods(vod: VOD, domains: SplitTerminator<'_, &str>){
    println!("Stream Timestamp: {}", vod.timestamp);
    println!("Streamer: {}", vod.username);
    println!("Stream ID: {}", vod.stream_id);
    println!("Searching for the VOD...");

    let base_string = vod.username + "_" + &vod.stream_id.to_string() + "_" + &vod.timestamp.to_string();
    let hashed = sha1_smol::Sha1::from(&base_string).digest().to_string()[..20].to_string();
    let final_string: String= hashed + "_" + &base_string + "/chunked/index-dvr.m3u8";

    let mut found = false;

    for domain in domains {
        let potential = domain.to_string() + &final_string;
        let res = req_workaround(&potential).await.unwrap();
        if verify_alive(res.status(), false) {
            println!("{}", potential);
            found = true;
        }
    }
    
    if !found {
        println!("No VOD was found on known domains.")
    }
}

/// Parses TwitchTracker for data.
/// Returns a VOD struct.
async fn get_stream_tt(url: &String) -> VOD {
    let res = req_workaround(&url.to_string()).await.unwrap();
    verify_alive(res.status(), true);

    println!("Connected to {}", url);

    let document = Html::parse_document(&res.text().await.unwrap());

    let timestamp = convert_date(get_element(r#"div[class="stream-timestamp-dt to-dowdatetime"]"#, &document).inner_html().as_str());
    let re = Regex::new(r"([a-zA-Z0-9_-]*)(/streams/)(\d+)").unwrap();
    let cap = re.captures(url).unwrap();

    let vod = VOD {timestamp: timestamp, username: cap[1].to_string(), stream_id: cap[3].parse::<i64>().unwrap()};

    return vod;
}

/// Dumb and quick workaround for the Result being annoying. Remove this dumbshit once the program works.
async fn req_workaround(url: &String) -> Result<reqwest::Response, reqwest::Error> {
    let res = reqwest::get(url).await?;
    Ok(res)
} 

fn get_element<'a>(element: &'static str, document: &'a Html) -> ElementRef<'a> {
    let selector = Selector::parse(element).unwrap();
    return document.select(&selector).next().unwrap();
}

fn convert_date(date: &str) -> i64 {
    let naive = Utc.datetime_from_str(&date, "%Y-%m-%d %H:%M:%S").unwrap();
    return naive.timestamp();
}

//Currently redundant given that Twitch Tracker is the only site supported, keeping it in case others are supported.
fn parse_url(url: &String) -> i32 {
    if url.contains("twitchtracker.com/") && url.contains("/streams/")
    {
        return 1;
    } else {
        return -1;
    }
}

pub fn verify_alive(status: reqwest::StatusCode, panic: bool) -> bool {
    match status {
        reqwest::StatusCode::OK => {
            return true;
        },
        reqwest::StatusCode::UNAUTHORIZED => {
            if panic
            {
                panic!("Unauthorized connection.\nResponse Code: {}", status);
            } else {
                return false;
            }
        },
        _ => {
            if panic
            {
                panic!("An error occured while connecting.\nResponse Code: {}", status);
            } else {
                return false;
            }
        },
    };
}