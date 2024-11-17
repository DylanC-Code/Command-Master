use std::{env::args, process};
use anyhow::Result;

pub mod cmd;
pub mod ia;

#[tokio::main]
async fn main() -> Result<()> {
    let command = get_command_arg();

    match &command[..] {
        "new" => cmd::new_command().await,
        _ => println!("Unknown command '{command}', try help, -h or --help!")
    };

    Ok(())
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
