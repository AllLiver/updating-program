use anyhow::{anyhow, Result};

#[tokio::main()]
async fn main() -> Result<()> {
    let current_version_local = env!("CARGO_PKG_VERSION");

    let remote_cargo_toml = reqwest::get("https://raw.githubusercontent.com/AllLiver/updating-program/refs/heads/main/Cargo.toml").await?.text().await?;

    let remote_cargo_toml = remote_cargo_toml.split("\n").collect::<Vec<&str>>();

    if let Some(current_version_remote) = remote_cargo_toml.iter().find(|x| x.starts_with("version = ")) {
        let current_version_remote = current_version_remote.split("=").collect::<Vec<&str>>()[1].trim().trim_start_matches('"').trim_end_matches('"');

        println!("{}", current_version_remote);
    } else {
        return Err(anyhow!("Failed to retrieve current version on main remote branch"));
    }
    Ok(())
}
