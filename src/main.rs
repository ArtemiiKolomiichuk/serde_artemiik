use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum RequestType {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "failure")]
    Failure,
}

#[derive(Serialize, Deserialize, Debug)]
struct Event{
    name: String,
    #[serde(serialize_with = "serealize_date", deserialize_with = "deserialize_date")]
    date: String,
}

fn serealize_date<S:Serializer>(date: &str, serealizer: S) -> Result<S::Ok, S::Error>{
    serealizer.serialize_str(&format!("Date: {}", date))
}

fn deserialize_date<'de, D:Deserializer<'de>>(deserializer: D) -> Result<String, D::Error>{
    let s: &str = Deserialize::deserialize(deserializer)?;
    Ok(s.replace("Date: ", ""))
}


fn main() {
    let event = Event{
        name: "EventName".to_string(),
        date: "2024-11-14".to_string(),
    };
    let json = serde_json::to_string(&event).unwrap();
    println!("{}\n", json);

    let de_event: Event = serde_json::from_str(&json).unwrap();
    println!("{:#?}\n", de_event);

    /* 
    let json = std::fs::read_to_string("req.json").unwrap();
    let request: Request = serde_json::from_str(&json).unwrap();
    println!("{:#?}\n", request);
    
    let yaml = serde_yaml::to_string(&request).unwrap();
    println!("Yaml:\n{{\n{}}}\n", yaml);
    let toml = toml::to_string(&request).unwrap();
    println!("Toml:\n{{\n{}}}", toml)
    */
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_1(){
        let json = std::fs::read_to_string("req.json").unwrap();
        let request: Request = serde_json::from_str(&json).unwrap();

        assert_eq!(request.req_type, RequestType::Success);
        assert_eq!(request.stream.user_id, uuid::Uuid::parse_str("8d234120-0bda-49b2-b7e0-fbd3912f6cbf").unwrap());
        assert_eq!(request.stream.public_tariff.price, 100.0);
        assert_eq!(request.gifts[0].price, 2.0);
        assert_eq!(request.gifts.len(), 2);
        assert_eq!(request.debug.duration, Duration::from_millis(234));
        assert_eq!(request.stream.public_tariff.duration, Duration::from_secs(3600));
        assert_eq!(request.stream.private_tariff.unwrap().duration, Duration::from_secs(60));
    }
}
