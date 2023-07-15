use std::error::Error;

use clap::{Args, Parser, Subcommand};
use common::Dice;

use crate::db::insert_character;
use character::Character;

pub mod character;
pub mod common;
pub mod db;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Character(CharacterArgs),
    Roll(RollArgs),
}

#[derive(Debug, Args)]
struct RollArgs {
    #[arg(short, long)]
    sides: u8,
    #[arg(short, long)]
    times: u8,
}

#[derive(Debug, Args)]
struct CharacterArgs {
    #[command(subcommand)]
    command: CharacterCommands,
}

#[derive(Debug, Subcommand)]
enum CharacterCommands {
    Create(CharacterCreateArgs),
}

#[derive(Debug, Args)]
struct CharacterCreateArgs {
    #[arg(short, long)]
    name: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let mut conn = db::create_connection("data.db".to_string())?;

    db::create_tables(&mut conn)?;

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Character(character) => {
            let character_cmd = &character.command;

            match character_cmd {
                CharacterCommands::Create(character_create_args) => {
                    insert_character(
                        &mut conn,
                        Character {
                            name: character_create_args.name.to_string(),
                        },
                    )?;
                }
            }
        }
        Commands::Roll(roll_args) => {
            let dice = Dice {
                sides: roll_args.sides,
            };

            let roll_result = dice.roll(roll_args.times);

            println!("{}", roll_result.to_string())
        }
    }

    Ok(())
}
