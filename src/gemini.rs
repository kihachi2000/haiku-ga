use chrono::{Datelike, Local};
use dotenv::dotenv;
use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{Read, Write};

use crate::error::{Error, Result};

type Parts = Vec<HashMap<String, String>>;
type Contents = Vec<HashMap<String, Parts>>;
type Data = HashMap<String, Contents>;

pub struct GeminiClient {
    daily_request_count: HashMap<String, u64>,
    todays_request_count: u64,
    client: Client,
    url: String,
    api_key: String,
}

impl GeminiClient {
    pub fn new() -> Result<Self> {
        let daily_request_count = Self::read_daily_request_count().unwrap_or_default();
        let today = Self::get_today_as_string();

        let client = GeminiClient {
            todays_request_count: *daily_request_count.get(&today).unwrap_or(&0),
            daily_request_count: daily_request_count,
            client: Client::new(),
            url: Self::url(),
            api_key: Self::api_key("GEMINI_API_KEY")?,
        };

        Ok(client)
    }

    pub async fn request(&mut self) -> Result<String> {
        let data = Self::build_data();
        let response = self.client
            .post(self.url.clone())
            .query(&[("key", self.api_key.clone())])
            .json(&data)
            .send()
            .await
            .expect("通信に失敗しました。")
            .json::<serde_json::Value>()
            .await
            .expect("JSONデータの解析に失敗しました。");

        self.todays_request_count += 1;
        if let Some(count) = self.daily_request_count.get_mut(&Self::get_today_as_string()) {
            *count = self.todays_request_count;
        };

        let _ = Self::write_todays_request_count(&self.daily_request_count)?;

        match response["candidates"][0]["content"]["parts"][0]["text"].as_str() {
            Some(reply) => Ok(reply.clone()),
            _ => Err(Error::ParseError),
        }
    }

    fn build_data() -> Data {
        let mut parts = Parts::new();
        let mut map = HashMap::<String, String>::new();
        map.insert("text".to_string(), "自己紹介をしてください。".to_string());
        parts.push(map);

        let mut contents = Contents::new();
        let mut map = HashMap::<String, Parts>::new();
        map.insert("parts".to_string(), parts);
        contents.push(map);

        let mut data = Data::new();
        data.insert("contents".to_string(), contents);

        data
    }

    fn get_today_as_string() -> String {
        let now = Local::now();
        format!("{}-{}-{}", now.year(), now.month(), now.day())
    }

    fn read_daily_request_count() -> Option<HashMap<String, u64>> {
        let mut file = File::open("./data/daily_request_count.json").ok()?;
        let mut text = String::new();

        file.read_to_string(&mut text).ok()?;
        let value: Value = serde_json::from_str(&text).ok()?;
        let obj = value.as_object()?;

        let mut map = HashMap::<String, u64>::new();
        for (key, value) in obj.iter() {
            map.insert(key.clone(), value.as_u64().unwrap());
        }

        Some(map)
    }

    fn write_todays_request_count(data: &HashMap<String, u64>) -> Result<()> {
        let file_name = "./data/daily_request_count.json";
        let text = serde_json::to_string(&data).unwrap();

        File::create(&file_name)
            .and_then(|mut file| file.write_all(text.as_bytes()))
            .map_err(|_| Error::FileWriteError(file_name.to_string()))?;

        Ok(())
    }

    fn url() -> String {
        [
            "https://generativelanguage.googleapis.com",
            "v1beta",
            "models",
            "gemini-2.0-flash:generateContent",
        ].join("/")
    }

    fn api_key(key: &str) -> Result<String> {
        match dotenv() {
            Ok(_) => env::var(key).map_err(|_| Error::VariableNotFound(key.to_string())),
            _ => Err(Error::DotEnvNotFound),
        }
    }
}
