// Change help docs in v0.3.0
//
// #[test]
// fn test() -> anyhow::Result<()> {
//     use assert_cmd::Command;
//     Command::cargo_bin("b2")?
//         .arg("config")
//         .assert()
//         .stdout("")
//         .stderr(concat!(
//             "Usage: b2 config <COMMAND>\n",
//             "\n",
//             "Commands:\n",
//             "  get   Get the value for a given key\n",
//             "  list  List all key-value pairs\n",
//             "  set   Set the value for a given key\n",
//             "  help  Print this message or the help of the given subcommand(s)\n",
//             "\n",
//             "Options:\n",
//             "  -h, --help  Print help\n",
//         ));
//     Ok(())
// }
