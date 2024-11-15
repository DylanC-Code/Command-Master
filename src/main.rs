use anyhow::Result;
use std::env::consts::OS;
use std::ffi::OsString;
use std::fs::{self, DirEntry};
use std::{env::args, process};

fn main() {
    let command = get_command_arg();

    match &command[..] {
        "new" => new_command(),
        _ => println!("Unknown command '{command}', try help, -h or --help!"),
    };
}

fn get_command_arg() -> String {
    let args: Vec<String> = args().collect();

    match args.get(1) {
        Some(v) => v.to_owned(),
        None => {
            println!(
                "Command parameter is missing!\r\nTry help, -h or --help to see the available commands."
            );
            process::exit(1)
        }
    }
}

fn new_command() {
    let commands = get_commands().unwrap();

    println!("There are {} available :  \r\n\r\n{commands:?}",commands.len());
}

fn get_commands() -> Result<Vec<String>> {
    let mut commands: Vec<String> = Vec::new();

    match OS {
        "linux" => {
            let bin_commands = fs::read_dir("/bin").unwrap();

            for dir_entry_result in bin_commands {
                let file_name = get_file_name(dir_entry_result?);
                if let Some(file_name) = file_name {
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
    let file_name = file.file_name();

    match file_name.to_str() {
        Some(v) => Some(String::from(v)),
        None => None,
    }
}
