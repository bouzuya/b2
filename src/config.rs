use std::fs;

#[derive(serde::Deserialize)]
struct ConfigJson {
    // `"/path/to/data_dir"`
    data_dir: String,
    // `"+09:00"`
    time_zone_offset: String,
}

pub struct Config(ConfigJson);

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        let xdg_dirs = xdg::BaseDirectories::with_prefix("net.bouzuya.rust-sandbox.b")?;
        let config_file_path = xdg_dirs.place_config_file("config.json")?;
        let config_file_content = fs::read_to_string(config_file_path)?;
        let config_json = serde_json::from_str::<ConfigJson>(&config_file_content)?;
        Ok(Self(config_json))
    }

    pub fn data_dir(&self) -> &str {
        &self.0.data_dir
    }

    pub fn time_zone_offset(&self) -> &str {
        &self.0.time_zone_offset
    }
}
