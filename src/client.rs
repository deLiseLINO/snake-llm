use std::{thread::sleep, time::Duration};

use reqwest::{
    blocking::{Client, Response},
    StatusCode,
};
use serde_derive::{Deserialize, Serialize};

use crate::client::models::{Choice, InputContent, Message, Models, Role, SYSTEM_PROMPT};

use self::models::OutputContent;

pub mod models;

#[derive(Serialize, Deserialize, Debug)]
struct GroqRequest {
    messages: Vec<models::Message>,
    model: String,
    temperature: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct GroqResponse {
    choices: Vec<Choice>,
}

pub trait ApiClient {
    fn snake_commands(&mut self, input: InputContent) -> Result<OutputContent, String>;
}
pub struct GroqClient {
    client: Client,
    url: String,
    token: String,
    request: GroqRequest,
}

impl ApiClient for GroqClient {
    fn snake_commands(&mut self, input: InputContent) -> Result<OutputContent, String> {
        self.snake_commands(input)
    }
}

impl GroqClient {
    pub fn new(url: String, token: String) -> Self {
        Self {
            client: Client::new(),
            url: url,
            token: token,
            request: default_request(Models::Llama3b70),
        }
    }

    pub fn snake_commands(&mut self, input: InputContent) -> Result<OutputContent, String> {
        self.add_massage_to_request(input);

        let body = match serde_json::to_string(&self.request) {
            Ok(body) => body,
            Err(e) => return Err(format!("Failed to serialize request: {}", e)),
        };

        let resp = self.post(&self.url, body);
        let status = resp.status();

        if status != StatusCode::OK {
            if status == StatusCode::TOO_MANY_REQUESTS {
                sleep(Duration::from_secs(5));
            }
            return Err(format!("Request failed with status: {}", status));
        }

        let resp_body = match resp.text() {
            Ok(body) => body,
            Err(e) => return Err(format!("Failed to get response body: {}", e)),
        };

        let resp: Option<GroqResponse> = match serde_json::from_str(&resp_body) {
            Ok(r) => r,
            Err(e) => {
                return Err(format!("Failed to parse response body {}", e));
            }
        };
        if let Some(resp) = resp {
            let _res: OutputContent = match serde_json::from_str(&resp.choices[0].message.content) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    return Err(format!("Failed to parse messages body {}", e));
                }
            };
        } else {
            return Err("No response form api".to_string());
        };
    }

    fn add_massage_to_request(&mut self, input: InputContent) {
        // if self.request.messages.len() > 1 {
        //     self.request.messages.pop();
        // }
        self.request.messages.push(Message {
            role: Role::User.as_string(),
            content: serde_json::to_string(&input).unwrap(),
        });
    }

    fn post(&self, url: &str, body: String) -> Response {
        self.client
            .post(url)
            .bearer_auth(&self.token)
            .body(body)
            .send()
            .unwrap()
    }
}

fn default_request(model: Models) -> GroqRequest {
    GroqRequest {
        temperature: 1.0,
        messages: vec![Message {
            role: Role::System.as_string(),
            content: SYSTEM_PROMPT.to_string(),
        }],
        model: model.as_string(),
    }
}
