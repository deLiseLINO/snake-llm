use figment::{
    providers::{Format, Yaml},
    Figment,
};
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub groq_client: TokenClient,
    pub ollama_client: Client,
}

#[derive(Deserialize)]
pub struct TokenClient {
    pub url: String,
    pub token: String,
}

#[derive(Deserialize)]
pub struct Client {
    pub url: String,
}

pub fn parse() -> Config {
    let config: Config = Figment::new()
        .join(Yaml::file("config.yaml"))
        .extract()
        .unwrap();

    config
}
