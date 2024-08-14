mod command;

#[derive(clap::Parser)]
struct Cli {
    #[clap(subcommand)]
    subcommand: Subcommand,
}

#[derive(clap::Subcommand)]
enum Subcommand {
    List,
    New,
}

fn main() -> anyhow::Result<()> {
    let cli = <Cli as clap::Parser>::parse();
    match cli.subcommand {
        Subcommand::List => self::command::list::execute(),
        Subcommand::New => self::command::new::execute(),
    }
}
