use std::fs;

#[derive(serde::Deserialize)]
struct ConfigJson {
    // `"/path/to/data_dir"`
    data_dir: String,
    // `"+09:00"`
    time_zone_offset: String,
}

pub struct Args {
    pub list: bool,
}

pub fn execute(Args { list }: Args) -> anyhow::Result<()> {
    if list {
        let xdg_dirs = xdg::BaseDirectories::with_prefix("net.bouzuya.rust-sandbox.b")?;
        let config_file_path = xdg_dirs.place_config_file("config.json")?;
        let config_file = fs::read_to_string(&config_file_path)?;
        let config = serde_json::from_str::<ConfigJson>(&config_file)?;

        println!("data_dir={}", config.data_dir);
        println!("time_zone_offset={}", config.time_zone_offset);
    }
    Ok(())
}
