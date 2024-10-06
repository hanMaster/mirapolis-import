use std::{fs, io};
use std::io::BufRead;

use serde::Serialize;
use serde_json::json;

use crate::config::config;
use crate::error::Result;

mod config;
mod error;

#[derive(Debug, Serialize)]
struct Payload {
    title: String,
    body: String,
    #[serde(rename(serialize = "userId"))]
    user_id: u32,
}

#[tokio::main]
async fn main() -> Result<()> {
    let file = fs::File::open(&config().PAYLOAD_PATH)?;
    let mut lines = io::BufReader::new(file).lines();

    let mut payload: Vec<Payload> = vec![];

    // Пропуск заголовков
    lines.next();

    while let Some(data) = lines.next() {
        let str = data?;
        let arr: Vec<&str> = str.split(';').filter(|field| field.len() > 0).collect();
        let row = Payload {
            title: String::from(arr[0]),
            body: String::from(arr[1]),
            user_id: arr[2].parse::<u32>()?,
        };
        payload.push(row);
    }
    // println!("{:#?}", payload);

    let client = reqwest::Client::new();
    let res = client
        .post(&config().API_URL)
        .json(&json!(payload[0]))
        .send()
        .await?;

    println!("Request status: {:?}", res.status());

    Ok(())
}
