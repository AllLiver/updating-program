use anyhow::{anyhow, Result};

#[tokio::main()]
async fn main() -> Result<()> {
    let current_version_local = env!("CARGO_PKG_VERSION");

    let remote_cargo_toml = reqwest::get("https://raw.githubusercontent.com/AllLiver/updating-program/refs/heads/main/Cargo.toml").await?.text().await?;

    let remote_cargo_toml = remote_cargo_toml.split("\n").collect::<Vec<&str>>();

    if let Some(current_version_remote) = remote_cargo_toml.iter().find(|x| x.starts_with("version = ")) {
        let current_version_remote = current_version_remote.split("=").collect::<Vec<&str>>()[1].trim().trim_start_matches('"').trim_end_matches('"');

        let current_version_local_vec: Vec<u8> = current_version_local.split(".").map(|x| x.parse::<u8>().expect("Failed to parse remote version")).collect();
        let current_version_remote_vec: Vec<u8> = current_version_remote.split(".").map(|x| x.parse::<u8>().expect("Failed to parse remote version")).collect();

        let mut out_of_date = false;

        for i in 0..current_version_local_vec.len() {
            if current_version_remote_vec[i] > current_version_local_vec[i] {
                out_of_date = true;
                break;
            } else if current_version_remote_vec[i] < current_version_local_vec[i] {
                break;
            }
        }

        if out_of_date {
            println!("Out of date, updating from {} -> {}", current_version_local, current_version_remote);
        } else {
            println!("Up to date!");
        }
    } else {
        return Err(anyhow!("Failed to retrieve current version on main remote branch"));
    }
    Ok(())
}
