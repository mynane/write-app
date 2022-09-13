use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct Res<T> {
    pub code: i8,
    pub data: Option<T>,
    pub called: String,
}

pub async fn call_function<T: DeserializeOwned + Default>(
    name: &str,
    body: &HashMap<&str, &str>,
) -> reqwest::Result<Res<T>> {
    let client = reqwest::Client::new();
    client
        .post(format!("http://localhost:3000/callFunction?name={}", name))
        .json(body)
        .send()
        .await?
        .json::<Res<T>>()
        .await
}

pub fn call_function_blocking<T: DeserializeOwned + Default>(
    name: &str,
    body: &HashMap<&str, &str>,
) -> reqwest::Result<Res<T>> {
    let client = reqwest::blocking::Client::new();
    client
        .post(format!("http://localhost:3000/callFunction?name={}", name))
        .json(body)
        .send()?
        .json::<Res<T>>()
}
