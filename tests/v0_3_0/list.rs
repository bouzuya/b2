#[test]
fn test() -> anyhow::Result<()> {
    use anyhow::Context as _;
    use assert_cmd::Command;
    use std::fs;
    let temp_dir = tempfile::tempdir()?;
    let temp_dir_path = temp_dir.path();

    let data_dir = temp_dir_path.join("data_dir");
    let data_file_dir = data_dir.join("flow").join("1970").join("01").join("01");
    fs::create_dir_all(&data_file_dir)?;
    let data_file = data_file_dir.join("19700101T090000Z.md");
    fs::write(&data_file, r#"note"#)?;
    let meta_file = data_file_dir.join("19700101T090000Z.json");
    fs::write(&meta_file, r#"{"created_at":"1970-01-01T00:00:00+09:00"}"#)?;

    let xdg_config_home = temp_dir_path.join("config");
    let config_dir = xdg_config_home.join("net.bouzuya.rust-sandbox.b");
    fs::create_dir_all(&config_dir)?;
    let config_file = config_dir.join("config.json");
    fs::write(
        config_file,
        &format!(
            r#"{{"data_dir":"{}","time_zone_offset":"+09:00"}}"#,
            data_dir.display()
        ),
    )?;

    temp_env::with_var(
        "XDG_CONFIG_HOME",
        Some(xdg_config_home.to_str().context("not UTF-8")?),
        || {
            Command::cargo_bin("b2")?
                .args(["list", "1970-01-01"])
                .assert()
                .stdout("19700101T090000Z note\n");
            Ok(())
        },
    )
}
