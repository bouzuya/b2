use crate::Config;

pub struct Args {
    pub key: String,
}

pub fn execute(Args { key }: Args) -> anyhow::Result<()> {
    let config = Config::load()?;

    println!(
        "{}",
        match key.as_str() {
            "data_dir" => config.data_dir(),
            "time_zone_offset" => config.time_zone_offset(),
            _ => anyhow::bail!("unknown config key {}", key),
        }
    );

    Ok(())
}
