use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use url::Url;

#[derive(Serialize, Deserialize, Debug)]
struct PublicTariff {
    #[serde(rename = "id")]
    identifier: i32,
    price: f32,
    #[serde(with = "humantime_serde")]
    duration: Duration,
    description: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct PrivateTariff {
    #[serde(rename = "client_price")]
    price: f32,
    #[serde(with = "humantime_serde")]
    duration: Duration,
    description: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Stream {
    user_id: uuid::Uuid,
    is_private: bool,
    settings: i32,
    shard_url: Url,
    public_tariff: PublicTariff,
    private_tariff: Option<PrivateTariff>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Gift {
    #[serde(rename = "id")]
    identifier: i32,
    price: f32,
    description: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Debug {
    #[serde(with = "humantime_serde")]
    duration: Duration,
    at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Request {
    #[serde(rename = "type")]
    req_type: RequestType,
    stream: Stream,
    gifts: Vec<Gift>,
    debug: Debug,
}

#[derive(Serialize, Deserialize, Debug)]
enum RequestType {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "failure")]
    Failure,
}

fn main() {
    let json = std::fs::read_to_string("req.json").unwrap();
    let request: Request = serde_json::from_str(&json).unwrap();
    println!("{:#?}\n", request);
    
    let yaml = serde_yaml::to_string(&request).unwrap();
    println!("Yaml:\n{{\n{}}}\n", yaml);
    let toml = toml::to_string(&request).unwrap();
    println!("Toml:\n{{\n{}}}", toml)
}
