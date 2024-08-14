use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr as _;

use anyhow::Context as _;
use chrono::FixedOffset;
use chrono::Utc;

#[derive(serde::Deserialize)]
struct ConfigJson {
    // `"/path/to/data_dir"`
    data_dir: String,
    // `"+09:00"`
    time_zone_offset: String,
}

#[derive(serde::Serialize)]
struct MetadataJson {
    created_at: String,
    tags: Vec<String>,
}

pub fn execute() -> anyhow::Result<()> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("net.bouzuya.rust-sandbox.b")?;
    let config_file_path = xdg_dirs.place_config_file("config.json")?;
    let config_file = fs::read_to_string(&config_file_path)?;
    let config = serde_json::from_str::<ConfigJson>(&config_file)?;
    let dir = PathBuf::from(config.data_dir);
    let now = Utc::now();

    let flow_dir = dir.join("flow");
    let today_dir = flow_dir
        .join(&now.format("%Y").to_string())
        .join(&now.format("%m").to_string())
        .join(&now.format("%d").to_string());
    fs::create_dir_all(&today_dir)?;

    let tz = FixedOffset::from_str(&config.time_zone_offset).context("+09:00 is valid offset")?;
    let now_in_jst = now.with_timezone(&tz);

    let content = "";
    let metadata = MetadataJson {
        created_at: now_in_jst.format("%Y-%m-%dT%H:%M:%Si%:z").to_string(),
        tags: vec![],
    };
    let mut file_path = today_dir.join(&now.format("%Y%m%dT%H%M%SZ").to_string());
    file_path.set_extension("json");
    fs::write(&file_path, serde_json::to_string_pretty(&metadata)?)?;
    file_path.set_extension("md");
    fs::write(&file_path, content)?;

    let editor = env::var("EDITOR").context("EDITOR environment variable is invalid")?;
    let status = Command::new("sh")
        .arg("-c")
        .arg(format!("{} {}", editor, file_path.display()))
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(anyhow::anyhow!("EDITOR is not success"))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr as _;

    use super::*;

    #[test]
    fn test_fixed_offset_from_str() -> anyhow::Result<()> {
        assert_eq!(
            FixedOffset::from_str("+09:00")?,
            FixedOffset::east_opt(9 * 60 * 60).context("+09:00 is valid fixed offset")?
        );
        Ok(())
    }
}
