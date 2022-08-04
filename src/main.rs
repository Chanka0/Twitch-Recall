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

#[tokio::main]
async fn main(){

    let domains = vec![
        "https://vod-secure.twitch.tv/",
        "https://vod-metro.twitch.tv/",
        "https://vod-pop-secure.twitch.tv/",
        "https://d2e2de1etea730.cloudfront.net/",
        "https://dqrpb9wgowsf5.cloudfront.net/",
        "https://ds0h3roq6wcgc.cloudfront.net/",
        "https://d2nvs31859zcd8.cloudfront.net/",
        "https://d2aba1wr3818hz.cloudfront.net/",
        "https://d3c27h4odz752x.cloudfront.net/",
        "https://dgeft87wbj63p.cloudfront.net/",
        "https://d1m7jfoe9zdc1j.cloudfront.net/",
        "https://d3vd9lfkzbru3h.cloudfront.net/",
        "https://d2vjef5jvl6bfs.cloudfront.net/",
        "https://d1ymi26ma8va5x.cloudfront.net/",
        "https://d1mhjrowxxagfy.cloudfront.net/",
        "https://ddacn6pr5v0tl.cloudfront.net/",
        "https://d3aqoihi2n8ty8.cloudfront.net/",
        "https://d1xhnb4ptk05mw.cloudfront.net/",
        "https://d6tizftlrpuof.cloudfront.net/",
        "https://d36nr0u3xmc4mm.cloudfront.net/",
        "https://d1oca24q5dwo6d.cloudfront.net/",
        "https://d2um2qdswy1tb0.cloudfront.net/",
        "https://d1w2poirtb3as9.cloudfront.net/",
        "https://d6d4ismr40iw.cloudfront.net/",
        "https://d1g1f25tn8m2e6.cloudfront.net/",
        "https://dykkng5hnh52u.cloudfront.net/",
        "https://d2dylwb3shzel1.cloudfront.net/",
        "https://d2xmjdvx03ij56.cloudfront.net/",
    ];

    let mut url = String::new();
    println!("Enter VOD URL: ");
    std::io::stdin().read_line(&mut url).unwrap();

    let site_type = parse_url(&url);

    if site_type == -1 {
        println!("URL was invalid. Make sure to use the supported sites, and input a URL like this:\nhttps://twitchtracker.com/channel/streams/stream_id");
    } 
    else if site_type == 1 
    {
        let vod = get_stream_tt(&url).await;
        scrape_vods(vod, domains).await;
    }

}

async fn scrape_vods(vod: VOD, domains: Vec<&str>){
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