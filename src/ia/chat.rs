use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, ClientBuilder,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

type ResultChat = Result<ChatResponse, ChatErrorResponse>;

pub struct ChatAPI {
    client: Client,
}

impl ChatAPI {
    const BASE_URL: &'static str = "https://api.openai.com/v1/chat/completions";

    pub fn new(api_secret_key: String) -> ChatAPI {
        let token = format!("Bearer {}", api_secret_key);
        let mut headers = HeaderMap::new();

        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Authorization", HeaderValue::from_str(&token).unwrap());

        let client = ClientBuilder::new()
            .default_headers(headers)
            .build()
            .unwrap();
        ChatAPI { client }
    }

    pub async fn new_exercise_for_cmd(&self, cmd: &str, cmd_man: &str) -> anyhow::Result<String> {
        let url = &ChatAPI::BASE_URL.to_string();
        let explanation = format!("You are a tech teacher an precisely a operating system master and you will provide a consice explication of the following command '{cmd}'.
              In addition, you will give a litle exercise to help learning the provided command followed by an example of this one.");
        let doc = format!("If you need, this is a part of the cmd manual : \r\n{cmd_man}");
        let body = json!(
            {
                "model": "gpt-4o-mini",
                "messages": [
                    {"role": "system", "content": explanation},
                    {"role": "system", "content":doc}
                ]
            }
        )
        .to_string();

        let text_resp = self
            .client
            .post(url)
            .body(body)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        let chat_response = ChatAPI::parse_response(&text_resp).unwrap();
        let message = &chat_response.choices[0].message.content;

        Ok(message.to_string())
    }

    fn parse_response(response: &String) -> ResultChat {
        serde_json::from_str::<ChatResponse>(response).or_else(|_| {
            let error: ChatErrorResponse = serde_json::from_str(response)
                .unwrap_or_else(|e| panic!("Error parsing Project object!\n{e}"));

            Err(error)
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct ChatResponse {
    id: String,
    model: String,
    created: u64,
    choices: Vec<ChatChoice>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChatChoice {
    index: u32,
    message: ChatMessage,
    finish_reason: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatErrorResponse {
    #[serde(rename(deserialize = "type"))]
    error_type: String,
    code: Option<String>,
    message: String,
    param: Option<String>,
}
