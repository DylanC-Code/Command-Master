use anyhow::Result;
use rand::Rng;
use std::env::args;
use std::env::consts::OS;
use std::fs::{self, DirEntry, OpenOptions};
use std::io::{self, Read, Write};
use std::process;

//use ia::{get_ia_secret_key_api, IAClient};
use super::ia;

 pub async fn new_command() {
    let commands = get_commands().unwrap();
    let command = get_command(&commands);

    let ADMIN_API_KEY = ia::get_ia_secret_key_api();
    println!("ADMIN KEY retrieved!");
    let ia_client = ia::IAClient::new(ADMIN_API_KEY).await;
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
