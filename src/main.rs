use std::io::Write;
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

use crate::{auth::authenticate, commands::execute_command, util::RemotePath};

mod util;
mod auth;
mod commands;

#[derive(Debug, Parser)]
pub struct Cli {
    /// Filen account email
    #[arg(short, long)]
    email: Option<String>,

    /// Filen account password
    #[arg(short, long)]
    password: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Cd {
        directory: String,
    },
    Ls {
        directory: Option<String>,
    },
    Exit,
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Welcome to Filen CLI!");

    let cli = Cli::parse();
    let client = authenticate(&cli).await?;

    let mut working_path = RemotePath::new("");

    if let Some(command) = cli.command {
        execute_command(&client, &working_path, &command).await?;
    } else {
        loop {
            let line = prompt(&format!("{} >", working_path))?;
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            let mut args = shlex::split(line).with_context(|| "error: Invalid quoting")?;
            args.insert(0, String::from("filen"));
            let cli = match Cli::try_parse_from(args).map_err(|e| e.to_string()) {
                Ok(cli) => cli,
                Err(e) => {
                    eprintln!("{}", e);
                    continue;
                }
            };
            if cli.command.is_none() {
                continue;
            }
            match execute_command(&client, &working_path, &cli.command.unwrap()).await {
                Ok(result) => {
                    if result.exit {
                        break;
                    }
                    working_path = result.working_path.unwrap_or(working_path);
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }
    }

    Ok(())
}

pub struct CommandResult {
    working_path: Option<RemotePath>,
    exit: bool,
}
impl Default for CommandResult {
    fn default() -> CommandResult {
        CommandResult {
            working_path: None,
            exit: false,
        }
    }
}

// util

fn prompt(prompt: &str) -> Result<String> {
    write!(std::io::stdout(), "{} ", prompt.trim())?;
    std::io::stdout().flush()?;
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer)?;
    Ok(buffer)
}