use std::{
    fs::OpenOptions,
    io::{stdin, Read, Write},
};

pub mod chat;

pub fn get_api_key() -> String {
    let mut file_content = String::new();

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("ia_credentials")
        .unwrap_or_else(|e| panic!("Error opening/creating ia_credentials file: {e}"));

    let _ = file.read_to_string(&mut file_content);

    for line in file_content.lines() {
        if let Some((_, value)) = line.split_once("OPENAI_API_KEY=") {
            return value.trim().to_string();
        }
    }

    let api_key = get_stdin_string("Provide an OPENAI_API_KEY :");
    let line = format!("OPENAI_API_KEY={api_key}");

    if let Err(e) = file.write_all(line.as_bytes()) {
        panic!("Error persisting OPENAI_API_KEY in file ia_credentials : {e}");
    }

    return api_key;
}

fn get_stdin_string(message: &str) -> String {
    println!("{message}");

    let mut api_key = String::new();
    let _ = stdin().read_line(&mut api_key);

    api_key = api_key.trim().to_string();

    if !api_key.is_empty() {
        return api_key;
    }

    return get_stdin_string(message);
}
