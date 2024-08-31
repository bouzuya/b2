#[test]
fn test() -> anyhow::Result<()> {
    use assert_cmd::Command;
    Command::cargo_bin("b2")?.arg("config").assert().stdout("");
    Ok(())
}
