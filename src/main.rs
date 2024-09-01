mod command;

#[derive(clap::Parser)]
struct Cli {
    #[clap(subcommand)]
    subcommand: Subcommand,
}

#[derive(clap::Subcommand)]
enum Subcommand {
    #[clap(subcommand)]
    Config(CommandSubcommand),
    Edit {
        id: String,
    },
    List,
    New,
}

#[derive(clap::Subcommand)]
enum CommandSubcommand {
    /// Get the value for a given key
    Get {
        #[arg()]
        key: String,
    },
    /// List all key-value pairs
    List,
}

fn main() -> anyhow::Result<()> {
    let cli = <Cli as clap::Parser>::parse();
    match cli.subcommand {
        Subcommand::Config(subcommand) => match subcommand {
            CommandSubcommand::Get { key } => {
                self::command::config::get::execute(self::command::config::get::Args { key })
            }
            CommandSubcommand::List => self::command::config::list::execute(),
        },
        Subcommand::Edit { id } => self::command::edit::execute(self::command::edit::Args { id }),
        Subcommand::List => self::command::list::execute(),
        Subcommand::New => self::command::new::execute(),
    }
}
