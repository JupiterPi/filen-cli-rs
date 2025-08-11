use anyhow::{Context, Result};
use filen_sdk_rs::auth::Client;

use crate::{prompt, Cli};

pub async fn authenticate(cli: &Cli) -> Result<Client> {
    let mut client = authenticate_from_cli_args(cli).await;
    if client.is_err() {
        client = authenticate_from_prompt().await;
    }
    client
}

async fn authenticate_from_cli_args(cli: &Cli) -> Result<Client> {
    let email = cli.email.clone().context("Email is required")?;
    let password = cli.password.clone().context("Password is required")?;
    let client = Client::login(email, &password, "XXXXXX").await
        .with_context(|| "Failed to log in")?;
    Ok(client)
}

async fn authenticate_from_prompt() -> Result<Client> {
    let email = prompt("Email: ")?;
    let password = prompt("Password: ")?;
    let client = Client::login(email.trim().to_string(), &password.trim(), "XXXXXX").await
        .with_context(|| "Failed to log in")?;
    Ok(client)
}