use std::{fs::read_to_string, path::PathBuf};

use clap::{Parser, Subcommand};

use crate::parser::configuration;

#[derive(Parser)]
#[command(name = "habits", version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    List {
        #[arg(short, long)]
        configured: bool,
    },
}

pub fn run_cli() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::List { configured }) => {
            if *configured {
                let input = read_to_string("sample-config.habits").unwrap();
                let parsed = configuration(&input).unwrap().1;
                for habit in parsed {
                    println!("{}", habit);
                }
            } else {
                println!("Not printing testing lists...");
            }
        }
        None => {}
    }

    // Continued program logic goes here...
}
