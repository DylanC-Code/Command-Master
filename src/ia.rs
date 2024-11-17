use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::{fs::OpenOptions, io::Read, process};

pub struct IAClient {
    client: Client,
    token: String,
    file: File
}

#[derive(Serialize, Deserialize, Debug)]
struct RetrieveResponse {
    error: Option<ServerError>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ServerError {
    message: String,
    code: String,
    param: Option<String>,
    #[serde(rename(deserialize = "type"))]
    error_type: String,
}

impl IAClient {
    pub async fn new(api_secret_key: String) -> IAClient {
        let token = format!("Bearer {}", api_secret_key);
        let ia_client = IAClient {
            client: reqwest::Client::new(),
            token,
        };
        let _ = ia_client.retrieve_project().await;

        return ia_client;
    }

    async fn retrieve_project(&self) -> Result<String> {
        let resp = self
            .client
            .get("https://api.openai.com/v1/organization/projects/command_master")
            .header("Content-Type", "application/json")
            .header("Authorization", &self.token)
            .send()
            .await?
            .json::<RetrieveResponse>()
            .await?;

        println!("RESP: {resp:?}");
        if let Some(error) = resp.error {
            println!("ERROR {error:?}");
            match error.code.as_str() {
                "object_not_found" => {
                    println!("PROJECT NOT FOUND!");
                    self.create_project().await?
                }
                _ => {
                    eprintln!(
                        "Unknown error finding project 'command_master': {}",
                        error.code
                    );
                    process::exit(1)
                }
            };
        };

        Ok(String::new())
    }

    async fn create_project(&self) -> Result<()> {
        let body = HashMap::from([("name", "command_master")]);
        let resp = self
            .client
            .post("https://api.openai.com/v1/organization/projects")
            .header("Content-Type", "application/json")
            .header("Authorization", &self.token)
            .json(&body)
            .send()
            .await?
            .json::<CreateProjectResponse>()
            .await?;

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct CreateProjectResponse {
    id: String,
    object: String,
    name: String,
    created_at: u64,
    archived_at: Option<u64>,
    status: String,
}

pub fn get_ia_secret_key_api() -> String {
    let mut file = get_ia_credentials_file();
    let mut ia_credentials = String::new();
    let _ = file.read_to_string(&mut ia_credentials);

    let mut admin_secret_key = get_env_value(&ia_credentials, "ADMIN_SECRET_KEY");

    if !admin_secret_key.is_empty() {
        return admin_secret_key;
    };

    println!("Please provide an admin access key from your account :");

    let _ = io::stdin()
        .read_line(&mut admin_secret_key)
        .expect("Error reading line");

    if admin_secret_key.trim().is_empty() {
        eprintln!("Error: admin key can't be an empty string!");
        return get_ia_secret_key_api();
    };

    add_key_value_to_file("ADMIN_SECRET_KEY", &admin_secret_key, &mut file);

  //  let key_value = format!("ADMIN_SECRET_KEY={}", admin_secret_key);

  //  file.write_all(key_value.as_bytes())
  //      .expect("Error during trying to persist admin secret key...");

    admin_secret_key
}

fn add_key_value_to_file(key: &str, value: &str, file: &mut File) {
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)
        .unwrap_or_else(|err| panic!("Error reading file content : {err}"));

    let new_line = format!("{}={}", key, value);

    for mut line in file_content.lines() {
        if line.contains(key) {
            line = &new_line;
        }
    }

    if let Err(e) = file.write_all(new_line.as_bytes()) {
        eprintln!("Error trying to persist {key} key : {e}");
        process::exit(1)
    };
}

fn get_ia_credentials_file() -> File {
    let file_result = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open("ia_credentials");

    file_result.unwrap_or_else(|err| {
        eprintln!("Error during opening file containing ia api secret key : {err}");
        process::exit(1);
    })
}

fn get_env_value(content: &str, key: &str) -> String {
    if content.is_empty() {
        return String::new();
    };

    let mut pattern = key.to_string();
    pattern.push('=');

    for line in content.lines() {
        if let Some((_, value)) = line.split_once(&pattern) {
            if value.trim().is_empty() {
                break;
            };

            return value.trim().to_string();
        }
    }

    String::new()
}
