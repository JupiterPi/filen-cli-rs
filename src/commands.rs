use anyhow::Result;
use filen_sdk_rs::{auth::Client, fs::HasName};

use crate::{util::RemotePath, CommandResult, Commands};

pub async fn execute_command(client: &Client, working_path: &RemotePath, command: &Commands) -> Result<CommandResult> {
    let result: Option<CommandResult> = match command {
        Commands::Cd { directory } => {
            let working_path = working_path.navigate(directory);
            Some(CommandResult { working_path: Some(working_path), ..Default::default() })
        }
        Commands::Ls { directory: _ } => {
            let items = client.list_dir(client.root()).await
                .expect("Failed to list root directory");
            let mut directories = items.0.iter()
                .map(|f| f.name().expect("Failed to get directory name"))
                .collect::<Vec<&str>>();
            directories.sort();
            let mut files = items.1.iter()
                .map(|f| f.name().expect("Failed to get file name"))
                .collect::<Vec<&str>>();
            files.sort();
            println!("{}", [directories, files].concat().join("  "));
            None
        }
        Commands::Exit => {
            Some(CommandResult { exit: true, ..Default::default() })
        }
    };
    Ok(result.unwrap_or(CommandResult::default()))
}