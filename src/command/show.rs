use std::fs;
use std::path::PathBuf;

use chrono::DateTime;

use crate::Config;

pub struct Args {
    pub id: String,
}

pub fn execute(Args { id }: Args) -> anyhow::Result<()> {
    let config = Config::load()?;
    let dir = PathBuf::from(config.data_dir());

    let date_time = {
        anyhow::ensure!(id.len() == 16);
        let chars = id.chars().collect::<Vec<char>>();
        anyhow::ensure!(chars.len() == 16);
        let s = format!(
            "{}-{}-{}T{}:{}:{}Z",
            chars[0..4].iter().collect::<String>(),
            chars[4..6].iter().collect::<String>(),
            chars[6..8].iter().collect::<String>(),
            chars[9..11].iter().collect::<String>(),
            chars[11..13].iter().collect::<String>(),
            chars[13..15].iter().collect::<String>(),
        );
        DateTime::parse_from_rfc3339(&s)?
    };

    let flow_dir = dir.join("flow");
    let mut file_path = flow_dir
        .join(date_time.format("%Y").to_string())
        .join(date_time.format("%m").to_string())
        .join(date_time.format("%d").to_string())
        .join(date_time.format("%Y%m%dT%H%M%SZ").to_string());
    file_path.set_extension("md");

    let file_content = fs::read_to_string(file_path)?;
    print!("{}", file_content);
    Ok(())
}
