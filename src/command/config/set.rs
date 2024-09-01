use std::fs;

#[derive(serde::Deserialize, serde::Serialize)]
struct ConfigJsonPartial {
    // `"/path/to/data_dir"`
    #[serde(skip_serializing_if = "Option::is_none")]
    data_dir: Option<String>,
    // `"+09:00"`
    #[serde(skip_serializing_if = "Option::is_none")]
    time_zone_offset: Option<String>,
}

pub struct Args {
    pub key: String,
    pub value: String,
}

pub fn execute(Args { key, value }: Args) -> anyhow::Result<()> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("net.bouzuya.rust-sandbox.b")?;
    let config_file_path = xdg_dirs.place_config_file("config.json")?;
    let config_file = fs::read_to_string(&config_file_path)?;
    let mut config = serde_json::from_str::<ConfigJsonPartial>(&config_file)?;

    match key.as_str() {
        "data_dir" => {
            config.data_dir = Some(value);
        }
        "time_zone_offset" => {
            config.time_zone_offset = Some(value);
        }
        _ => anyhow::bail!("unknown config key {}", key),
    }

    fs::write(&config_file_path, serde_json::to_string(&config)?)?;
    Ok(())
}
