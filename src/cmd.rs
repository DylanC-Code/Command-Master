use anyhow::Result;
use rand::Rng;
use std::env::consts::OS;
use std::fs::{self, DirEntry, OpenOptions};
use std::io::{stdin, Read};
use std::process::{self, Command};

use super::ia;

pub async fn new_command() {
    let commands = get_commands().unwrap();
    let command = get_command(&commands);
    let command_man = get_command_manual(&command);

    let api_key = ia::get_api_key();
    let chat = ia::chat::ChatAPI::new(api_key);

    println!("Generating for command '{command}' :\r\n...");

    let message = chat
        .new_exercise_for_cmd(command, &command_man)
        .await
        .unwrap();

    println!("{message}");
//    save_the_command(command);
}

fn get_commands() -> Result<Vec<String>> {
    let mut commands: Vec<String> = Vec::new();

    match OS {
        "linux" => {
            let bin_commands = fs::read_dir("/bin").unwrap();

            for dir_entry_result in bin_commands {
                if let Some(file_name) = get_file_name(dir_entry_result?) {
                    commands.push(file_name);
                }
            }

            let sbin_commands = fs::read_dir("/sbin").unwrap();

            for dir_entry_result in sbin_commands {
                let file_name = match get_file_name(dir_entry_result?) {
                    Some(v) => v,
                    None => continue,
                };

                if !commands.contains(&file_name) {
                    commands.push(file_name);
                };
            }
        }
        _ => {
            println!("OS '{OS}' not supported");
            process::exit(1)
        }
    };

    Ok(commands)
}

fn get_file_name(file: DirEntry) -> Option<String> {
    file.file_name()
        .to_str()
        .and_then(|v| Some(String::from(v)))
}

fn get_command<'c>(commands: &'c Vec<String>) -> &'c String {
    let command_index = rand::thread_rng().gen_range(0..commands.len());
    commands.get(command_index).unwrap()
}

fn get_command_manual(command: &str) -> String {
    let manual = match Command::new("man").arg(command).output() {
        Err(e) => {
            eprintln!("Error reading manual of command {command}: {e}");
            let result = format!("No manual found for command {command}.");
            return String::from(result);
        }
        Ok(output) => String::from_utf8(output.stdout)
            .expect("Expect utf-8 but manual of command {command} it isn't!"),
    };

    let mut condensed = String::new();

    if let Some(name) = extract_section(&manual, "NAME") {
        condensed.push_str(&name)
    };
    if let Some(synopsys) = extract_section(&manual, "SYNOPSIS") {
        condensed.push_str(&synopsys)
    };
    if let Some(description) = extract_section(&manual, "DESCRIPTION") {
        if description.len() > 300 {
            condensed.push_str(&description[..300])
        } else {
            condensed.push_str(&description)
        }
    };

    return condensed;
}

fn extract_section(content: &str, section: &str) -> Option<String> {
    let pattern = format!("\n{section}");
    let value = match content.split_once(&pattern) {
        None => return None,
        Some((_, value)) => value,
    };

    let value = match value.split_once("\r\n\r\n") {
        Some((_, v)) => format!("{section}: {v}"),
        None => format!("{section}: {value}"),
    };

    Some(value)
}

fn save_the_command(command: &str) {
    let mut user_response = String::new();
    if let Err(e) = stdin().read_line(&mut user_response) {
        panic!("Error reading user input : {e}");
    };

    if user_response.trim().to_lowercase() != "y" {
        return;
    }

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("saved_commands")
        .unwrap();

    let mut file_content = String::new();
    let _ = file.read_to_string(&mut file_content);

    for line in file_content.lines() {
        let item = line.split(",");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn save_the_command_should_pass() {
        //save_the_command("");
    }
}
