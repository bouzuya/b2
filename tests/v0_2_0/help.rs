#[test]
fn test() -> anyhow::Result<()> {
    use assert_cmd::Command;
    Command::cargo_bin("b2")?
        .args(["help"])
        .assert()
        .stdout(concat!(
            "Usage: b2 <COMMAND>\n",
            "\n",
            "Commands:\n",
            "  config  \n",
            "  edit    \n",
            "  list    \n",
            "  new     \n",
            "  help    Print this message or the help of the given subcommand(s)\n",
            "\n",
            "Options:\n",
            "  -h, --help  Print help\n"
        ));

    Ok(())
}
