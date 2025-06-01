use clap::{Parser, Subcommand};
use std::fs;

mod generator;
mod migration;
mod parser;
mod types;

use generator::LocoGenerator;
use migration::{ColumnInfo, MigrationModifier};
use parser::MermaidParser;

#[derive(Parser)]
#[command(
    name = "triton",
    about = "A tool for generating Loco commands from Mermaid diagrams and managing Sea-ORM migrations",
    version = "0.1.0"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate Loco commands from Mermaid ER diagram
    Generate {
        /// Input Mermaid file path
        #[arg(short, long)]
        input: String,

        /// Output file path (optional, prints to stdout if not specified)
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Migration management commands
    Migration {
        #[command(subcommand)]
        migration_cmd: MigrationCommands,
    },
}

#[derive(Subcommand)]
enum MigrationCommands {
    /// Parse and validate a migration file
    Parse {
        /// Migration file path
        #[arg(short, long)]
        file: String,
    },
    /// Add a column to a table in migration
    AddColumn {
        /// Migration file path
        #[arg(short, long)]
        file: String,
        /// Table name
        #[arg(short, long)]
        table: String,
        /// Column name
        #[arg(short, long)]
        column: String,
        /// Column type (e.g., string, integer, boolean)
        #[arg(long, default_value = "string")]
        column_type: String,
        /// Whether the column is nullable
        #[arg(long, default_value = "false")]
        nullable: bool,
        /// Default value for the column
        #[arg(long)]
        default: Option<String>,
    },
    /// Drop a column from a table in migration
    DropColumn {
        /// Migration file path
        #[arg(short, long)]
        file: String,
        /// Table name
        #[arg(short, long)]
        table: String,
        /// Column name to drop
        #[arg(short, long)]
        column: String,
    },
    /// List all columns in a migration file
    List {
        /// Migration file path
        #[arg(short, long)]
        file: String,
        /// Filter by table name (optional)
        #[arg(short, long)]
        table: Option<String>,
    },
    /// Show migration file information
    Info {
        /// Migration file path
        #[arg(short, long)]
        file: String,
    },
}

async fn handle_migration_command(
    migration_cmd: MigrationCommands,
) -> Result<(), Box<dyn std::error::Error>> {
    match migration_cmd {
        MigrationCommands::Parse { file } => {
            println!("🔍 Parsing migration file: {}", file);
            let modifier = MigrationModifier::new(file)?;
            println!("✅ Migration parsed successfully");
        }
        MigrationCommands::AddColumn {
            file,
            table,
            column,
            column_type,
            nullable,
            default,
        } => {
            println!("➕ Adding column '{}' to table '{}'", column, table);
            let mut modifier = MigrationModifier::new(file)?;
            let column_info = migration::parser::ColumnInfo {
                name: column.clone(),
                column_type: column_type.clone(),
                nullable,
                default,
            };
            modifier.add_column(&table, column_info)?;
            println!(
                "✅ Column '{}' ({}) added to table '{}'",
                column, column_type, table
            );
        }
        MigrationCommands::DropColumn {
            file,
            table,
            column,
        } => {
            println!("➖ Dropping column '{}' from table '{}'", column, table);
            let mut modifier = MigrationModifier::new(file)?;
            modifier.drop_column(&table, &column)?;
            println!("✅ Column '{}' dropped from table '{}'", column, table);
        }
        MigrationCommands::List { file, table } => {
            println!("📋 Listing columns in migration file: {}", file);
            let modifier = MigrationModifier::new(file)?;

            match table {
                Some(table_name) => {
                    println!("Columns in table '{}':", table_name);
                    let columns = modifier.list_columns(&table_name);
                    if columns.is_empty() {
                        println!("  No columns found for table '{}'", table_name);
                    } else {
                        for column in columns {
                            let nullable_str = if column.nullable {
                                "nullable"
                            } else {
                                "not null"
                            };
                            let default_str = column
                                .default
                                .as_ref()
                                .map(|d| format!(" (default: {})", d))
                                .unwrap_or_default();
                            println!(
                                "  • {} ({}, {}){}",
                                column.name, column.column_type, nullable_str, default_str
                            );
                        }
                    }
                }
                None => {
                    println!("All columns:");
                    let all_columns = modifier.list_all_columns();
                    if all_columns.is_empty() {
                        println!("  No columns found");
                    } else {
                        for (table_name, columns) in all_columns {
                            println!("  Table '{}':", table_name);
                            for column in columns {
                                let nullable_str = if column.nullable {
                                    "nullable"
                                } else {
                                    "not null"
                                };
                                let default_str = column
                                    .default
                                    .as_ref()
                                    .map(|d| format!(" (default: {})", d))
                                    .unwrap_or_default();
                                println!(
                                    "    • {} ({}, {}){}",
                                    column.name, column.column_type, nullable_str, default_str
                                );
                            }
                        }
                    }
                }
            }
        }
        MigrationCommands::Info { file } => {
            println!("ℹ️  Migration file information: {}", file);
            let modifier = MigrationModifier::new(file.clone())?;
            let info = modifier.get_migration_info();

            println!("📁 File: {}", file);
            println!("📊 Up operations: {}", info.up_operations.len());
            println!("📊 Down operations: {}", info.down_operations.len());

            if !info.up_operations.is_empty() {
                println!("\n⬆️  Up operations:");
                for (i, op) in info.up_operations.iter().enumerate() {
                    println!("  {}. {:?}", i + 1, op);
                }
            }

            if !info.down_operations.is_empty() {
                println!("\n⬇️  Down operations:");
                for (i, op) in info.down_operations.iter().enumerate() {
                    println!("  {}. {:?}", i + 1, op);
                }
            }
        }
    }
    Ok(())
}

fn handle_generate_command(
    input: String,
    output: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔄 Generating Loco commands from: {}", input);

    // Mermaidファイルを読み込み
    let mermaid_content = fs::read_to_string(&input)
        .map_err(|e| format!("Failed to read input file '{}': {}", input, e))?;

    // パース
    let mut parser = MermaidParser::new();
    let schema = parser
        .parse(&mermaid_content)
        .map_err(|e| format!("Failed to parse Mermaid diagram: {}", e))?;

    // Locoコマンド生成
    let generator = LocoGenerator::new();
    let commands = generator.generate_commands(&schema);

    // 出力
    let output_content = commands.join("\n");

    match output {
        Some(path) => {
            fs::write(&path, &output_content)
                .map_err(|e| format!("Failed to write to output file '{}': {}", path, e))?;
            println!("✅ Commands written to: {}", path);
            println!("📝 Generated {} commands", commands.len());
        }
        None => {
            println!("📋 Generated Loco commands:\n");
            println!("{}", output_content);
            println!("\n📝 Total: {} commands", commands.len());
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate { input, output } => {
            handle_generate_command(input, output)?;
        }
        Commands::Migration { migration_cmd } => {
            handle_migration_command(migration_cmd).await?;
        }
    }

    Ok(())
}
