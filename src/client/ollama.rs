use std::time::Duration;

use reqwest::{
    blocking::{Client, Response},
    StatusCode,
};
use serde_derive::{Deserialize, Serialize};

use super::{
    models::{self, InputContent, Message, OutputContent, Role, SYSTEM_PROMPT},
    ApiClient,
};

#[derive(Serialize, Deserialize, Debug)]
struct OllamaRequest {
    model: String,
    messages: Vec<models::Message>,
    stream: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct OllamaResponse {
    message: models::Message,
}

pub struct OllamaClient {
    client: Client,
    url: String,
    request: OllamaRequest,
}

impl ApiClient for OllamaClient {
    fn snake_commands(&mut self, input: InputContent) -> Result<OutputContent, String> {
        self.snake_commands(input)
    }
}

impl OllamaClient {
    pub fn new(url: String, model: String) -> Self {
        Self {
            client: Client::new(),
            url,
            request: OllamaRequest {
                model: model,
                messages: vec![Message {
                    role: Role::System.as_string(),
                    content: SYSTEM_PROMPT.to_string(),
                }],
                stream: false,
            },
        }
    }

    pub fn snake_commands(&mut self, input: InputContent) -> Result<OutputContent, String> {
        if self.request.messages.len() > 1 {
            self.request.messages.pop();
        }
        self.request.messages.push(models::Message {
            role: Role::User.as_string(),
            content: serde_json::to_string(&input).unwrap(),
        });

        let body = match serde_json::to_string(&self.request) {
            Ok(body) => body,
            Err(e) => return Err(format!("Failed to serialize request: {}", e)),
        };

        let resp = self.post(&self.url, body);
        let status = resp.status();

        if status != StatusCode::OK {
            return Err(format!("Request failed with status: {}", status));
        }

        let resp_body = match resp.text() {
            Ok(body) => body,
            Err(e) => return Err(format!("Failed to get response body: {}", e)),
        };

        let resp: Option<OllamaResponse> = match serde_json::from_str(&resp_body) {
            Ok(r) => r,
            Err(e) => {
                return Err(format!("Failed to parse response body {}", e));
            }
        };

        if let Some(resp) = resp {
            let json = extract_json(&resp.message.content);
            let _res: OutputContent = match serde_json::from_str(&json) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    return Err(format!(
                        "Failed to parse messages body: {}, response: {}",
                        e, json
                    ));
                }
            };
        } else {
            return Err("No response form api".to_string());
        };
    }

    fn post(&self, url: &str, body: String) -> Response {
        self.client
            .post(url)
            .timeout(Duration::from_secs(60 * 10))
            .body(body)
            .send()
            .unwrap()
    }
}

fn extract_json(input: &str) -> String {
    let start_idx = input.find('{').unwrap_or(input.len());
    let end_idx = input.rfind('}').unwrap_or(input.len());
    let extracted = input[start_idx..end_idx + 1].to_string();
    extracted
}
