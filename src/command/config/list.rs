use crate::Config;

pub fn execute() -> anyhow::Result<()> {
    let config = Config::load()?;

    println!("data_dir={}", config.data_dir());
    println!("time_zone_offset={}", config.time_zone_offset());
    Ok(())
}
