use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use lingua::{ LanguageDetector, LanguageDetectorBuilder};
use lingua::Language as Lang;
use Lang::{English, Hungarian, Russian};
use reqwest::Client;
use serde_json;
use clap::Parser;

async fn telnet_client(host: &str, port: u16) -> std::io::Result<()> {
    let mut stream = match TcpStream::connect((host, port)).await {
        Ok(stream) => stream,
        Err(err) => {
            eprintln!("Failed to connect: {}", err);
            return Err(err.into());
        }
    };
    println!("connected to ({}, {})", host, port);

    let mut buffer = [0; 100000];
    let mut players: Vec<String> = Vec::new();
    stream.write_all(b"status\r\n").await?;
    loop {
        let bytes_read = stream.read(&mut buffer).await?;
        let data = String::from_utf8_lossy(&buffer[..bytes_read]);
        let lines: Vec<String> = data.lines().map(|line| line.to_string()).collect();
        for line in lines {
            //IN PROGRESS new way to detect user connect, disconnect
            if line.ends_with(" connected") {
                println!("{}", line);

            }
            if line.ends_with(" disconnected"){
                println!("{}", line);
            }
            //after asking status parse users
            if line.contains("active") {
                let slices: Vec<String> = line.split("'").map(|s| s.to_string()).collect();
                if slices.len() > 1 {
                    let player = slices[1].to_string();
                    if !players.contains(&player) {
                        players.push(player);
                        //println!("{:?}",players);
                    }
                }

            }
            if  players.iter().any(|player| line.contains(player)) && (line.contains("@") || line.contains("[DEAD]") || line.contains("[ALL]") || line.contains("Terrorist")) {
                let n_translate = need_translate(&line);
                if n_translate != true  {
                println!("{}", line);
                }
                else {

                    let line_slice: Vec<String> = line.split(":").map(|s| s.to_string()).collect();
                    let message: String = line_slice.iter().skip(1).map(|s| s.as_str()).collect::<Vec<&str>>().join("");
                    if message.len() == 0 {
                        println!("{}", line);
                        continue;
                    }
                    let t_message = trans(&message).await;
                    match t_message {
                        Ok(translation) => {
                            println!("{} : {}\n  [{}]", line_slice[0], message, translation.unwrap());
                        }
                        Err(err) => {
                            println!("{} : {}\n  [{:?}]", line_slice[0], message, err);
                        }
                    };
                }
                }
            }
        }
    }


async fn trans(text: &str) -> Result<Option<String>, reqwest::Error> {
    let client = Client::new();
    let response = client
        .get("https://translate.googleapis.com/translate_a/single")
        .query(&[("client", "dict-chrome-ex"), ("sl", "auto"), ("tl", "en"), ("dt", "t"), ("q", text)])
        .send()
        .await?;

    let response_text = response.text().await?;
    let translation: Option<String> = serde_json::from_str(&response_text)
        .ok()
        .and_then(|value: serde_json::Value| value.get(0).and_then(|v| v.get(0)).and_then(|v| v.get(0)).cloned())
        .and_then(|s| Some(s.to_string()))
        .map(|s| s.to_string())
        .map(|s| s.trim_matches('"').to_string());
    Ok(translation)
}

fn need_translate(text: &String) -> bool {
    //language detection is somewhat ok with these settings
    let l: Vec<Lang> = vec![English, Russian, Hungarian];
    //let detector: LanguageDetector = LanguageDetectorBuilder::from_all_spoken_languages().build();
    let detector: LanguageDetector = LanguageDetectorBuilder::from_languages(&l).with_minimum_relative_distance(0.9).build();
    let detected_language: Option<Lang> = detector.detect_language_of(text);
    //println!("{:?}", detected_language);
    match detected_language {
        Some(lang) => {
            if lang == English {
                return false;
            }
            else {
                return true;
            }
        }
        None => {
            return false;
        }
    }
}

#[derive(Parser)]
struct Args {
    #[clap(short, long, default_value = "127.0.0.1")]
    server: String,

    #[clap(short, long, default_value = "1337")]
    port: u16,
}
#[tokio::main]
async fn main() {
    let args: Args = Args::parse();
    telnet_client(&args.server, args.port).await.unwrap();
}
