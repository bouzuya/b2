mod command;
mod config;

pub use self::config::Config;

#[derive(clap::Parser)]
struct Cli {
    #[clap(subcommand)]
    subcommand: Subcommand,
}

#[derive(clap::Subcommand)]
enum Subcommand {
    /// Manage configs
    #[clap(subcommand)]
    Config(CommandSubcommand),
    /// Edit the b with the given id
    Edit { id: String },
    /// List bs
    List {
        /// YYYY-MM-DD
        date: Option<String>,
    },
    /// Create a new b
    New,
    /// Show the b
    Show { id: String },
}

#[derive(clap::Subcommand)]
enum CommandSubcommand {
    /// Get the value for a given key
    Get { key: String },
    /// List all key-value pairs
    List,
    /// Set the value for a given key
    Set {
        #[arg()]
        key: String,
        #[arg()]
        value: String,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = <Cli as clap::Parser>::parse();
    match cli.subcommand {
        Subcommand::Config(subcommand) => match subcommand {
            CommandSubcommand::Get { key } => {
                self::command::config::get::execute(self::command::config::get::Args { key })
            }
            CommandSubcommand::List => self::command::config::list::execute(),
            CommandSubcommand::Set { key, value } => {
                self::command::config::set::execute(self::command::config::set::Args { key, value })
            }
        },
        Subcommand::Edit { id } => self::command::edit::execute(self::command::edit::Args { id }),
        Subcommand::List { date } => {
            self::command::list::execute(self::command::list::Args { date })
        }
        Subcommand::New => self::command::new::execute(),
        Subcommand::Show { id } => self::command::show::execute(self::command::show::Args { id }),
    }
}
