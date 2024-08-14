use std::collections::BTreeSet;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use anyhow::Context as _;
use chrono::Utc;

#[derive(serde::Deserialize)]
struct ConfigJson {
    // `"/path/to/data_dir"`
    data_dir: String,
    // `"+09:00"`
    // time_zone_offset: String,
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

    let mut lines = BTreeSet::new();
    for dir_entry in fs::read_dir(&today_dir)? {
        let dir_entry = dir_entry?;
        let path = dir_entry.path();
        if let Some(extension) = path.extension() {
            if extension == "md" {
                let id = path
                    .file_stem()
                    .context("no file_stem")?
                    .to_str()
                    .context("not UTF-8")?
                    .to_string();

                let mut file = File::open(&path)?;
                let mut buf = [0; 1024];
                let _ = Read::read(&mut file, &mut buf)?;
                let s = String::from_utf8_lossy(&buf);
                let s = s.trim_end_matches(char::REPLACEMENT_CHARACTER);
                let s = s.replace("\n", " ");
                let s = s.chars().take(30).collect::<String>();
                // YYYYMMDDTHHMMSSZ
                // 12345678901234567...
                // 78 - 17 = 61
                // 61 / 2 ~= 30
                lines.insert(format!("{} {}", id, s));
            }
        }
    }

    for line in lines {
        println!("{}", line);
    }
    Ok(())
}
