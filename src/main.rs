mod command;

#[derive(clap::Parser)]
struct Cli {
    #[clap(subcommand)]
    subcommand: Subcommand,
}

#[derive(clap::Subcommand)]
enum Subcommand {
    Config {
        #[arg(long)]
        list: bool,
    },
    Edit {
        id: String,
    },
    List,
    New,
}

fn main() -> anyhow::Result<()> {
    let cli = <Cli as clap::Parser>::parse();
    match cli.subcommand {
        Subcommand::Config { list } => {
            self::command::config::execute(self::command::config::Args { list })
        }
        Subcommand::Edit { id } => self::command::edit::execute(self::command::edit::Args { id }),
        Subcommand::List => self::command::list::execute(),
        Subcommand::New => self::command::new::execute(),
    }
}
