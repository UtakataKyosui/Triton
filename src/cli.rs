// src/cli.rs
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Generate {
        #[arg(short, long)]
        config: String,
    },
    Migration {
        #[command(subcommand)]
        migration_cmd: MigrationCommands,
    },
}

#[derive(Subcommand, Debug)]
pub enum MigrationCommands {
    Parse {
        #[arg(short, long)]
        file: String,
    },
    AddColumn {
        #[arg(short, long)]
        file: String,
        #[arg(short, long)]
        table: String,
        #[arg(short, long)]
        column: String,
        #[arg(short, long)]
        column_type: String,
        #[arg(long)]
        nullable: bool,
    },
    DropColumn {
        #[arg(short, long)]
        file: String,
        #[arg(short, long)]
        table: String,
        #[arg(short, long)]
        column: String,
    },
    List {
        #[arg(short, long)]
        file: String,
    },
}
