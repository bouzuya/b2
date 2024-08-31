#[test]
fn test() -> anyhow::Result<()> {
    use assert_cmd::Command;
    use std::fs;
    let temp_dir = tempfile::tempdir()?;
    let temp_dir_path = temp_dir.path();
    let config_dir = temp_dir_path.join("net.bouzuya.rust-sandbox.b");
    fs::create_dir_all(&config_dir)?;
    let config_file = config_dir.join("config.json");
    fs::write(
        config_file,
        r#"{"data_dir":"/path/to/data_dir","time_zone_offset":"+09:00"}"#,
    )?;
    temp_env::with_var("XDG_CONFIG_HOME", Some(temp_dir_path), || {
        Command::cargo_bin("b2")?
            .args(["config", "--list"])
            .assert()
            .stdout("data_dir=/path/to/data_dir\ntime_zone_offset=+09:00\n");
        Ok(())
    })
}
