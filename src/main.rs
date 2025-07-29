use clap::{Parser, Subcommand};
use std::fs::{self, create_dir_all, write};
use std::path::PathBuf;

/// My TLDR, your own commands note!
#[derive(Parser, Debug)]
#[command(
    name = "mldr",
    version = "0.1",
    about = "My TLDR, your own commands note!"
)]
struct Cli {
    /// The command name to show the note
    command: Option<String>,

    #[command(subcommand)]
    subcommand: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Add or edit a note
    Edit {
        /// The command name to edit
        command: String,
        /// The note content
        note: String,
    },
}

fn get_note_path(command: &str) -> PathBuf {
    let mut dir = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    dir.push("mldr");
    create_dir_all(&dir).unwrap();
    dir.push(format!("{command}.txt"));
    dir
}

fn main() {
    let cli = Cli::parse();

    match (cli.command, cli.subcommand) {
        (Some(command), None) => {
            let path = get_note_path(&command);
            if path.exists() {
                let content = fs::read_to_string(path).unwrap_or_default();
                println!("{content}");
            } else {
                println!("No note found for `{command}`.");
            }
        }
        (None, Some(Commands::Edit { command, note })) => {
            let path = get_note_path(&command);
            write(path, note).unwrap();
            println!("Note saved for `{command}`.");
        }
        (Some(_), Some(_)) => {
            eprintln!("Error: Cannot specify both a command name and a subcommand.");
            std::process::exit(1);
        }
        (None, None) => {
            eprintln!("Error: Please specify a command name or use a subcommand.");
            std::process::exit(1);
        }
    }
}
