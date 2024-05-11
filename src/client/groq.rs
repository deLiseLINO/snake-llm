use reqwest::{
    blocking::{Client, Response},
    StatusCode,
};
use serde_derive::{Deserialize, Serialize};

use super::{
    models::{self, Choice, InputContent, Message, OutputContent, Role, SYSTEM_PROMPT},
    ApiClient,
};

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

pub struct GroqClient {
    client: Client,
    url: String,
    token: String,
    request: GroqRequest,
}

#[allow(dead_code)]
pub enum GroqModels {
    Llama3b70,
    LLama3b8,
    Mixtrel8b,
}

impl GroqModels {
    pub fn as_string(&self) -> String {
        match self {
            GroqModels::LLama3b8 => "llama3-8b-8192".to_owned(),
            GroqModels::Llama3b70 => "llama3-70b-8192".to_owned(),
            GroqModels::Mixtrel8b => "Mixtral-8x7b-32768".to_owned(),
        }
    }
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
            url,
            token,
            request: default_request(GroqModels::Llama3b70),
        }
    }

    pub fn snake_commands(&mut self, input: InputContent) -> Result<OutputContent, String> {
        self.add_message_to_request(input);

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
                    return Err(format!(
                        "Failed to parse messages body: {}, response: {:?}",
                        e, resp.choices[0].message
                    ));
                }
            };
        } else {
            return Err("No response form api".to_string());
        };
    }

    fn add_message_to_request(&mut self, input: InputContent) {
        if self.request.messages.len() > 1 {
            self.request.messages.pop();
        }
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

fn default_request(model: GroqModels) -> GroqRequest {
    GroqRequest {
        temperature: 1.0,
        messages: vec![Message {
            role: Role::System.as_string(),
            content: SYSTEM_PROMPT.to_string(),
        }],
        model: model.as_string(),
    }
}

