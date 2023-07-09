use clap::{Args, Parser, Subcommand};
use common::Dice;

pub mod character;
pub mod common;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Adds files to myapp
    Math(MathArgs),
}

#[derive(Debug, Args)]
struct MathArgs {
    #[command(subcommand)]
    command: MathCommands,
}

#[derive(Debug, Subcommand)]
enum MathCommands {
    Add(AddArgs),
}

#[derive(Debug, Args)]
struct AddArgs {
    #[arg(short, long, allow_hyphen_values = true)]
    x: i32,

    #[arg(short, long, allow_hyphen_values = true)]
    y: i32,
}

fn main() {
    let cli = Cli::parse();

    let dice = Dice { sides: 6 };

    let roll_result = dice.roll(1);

    println!("{}", roll_result.to_string());

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Math(math) => {
            let math_cmd = &math.command;

            match math_cmd {
                MathCommands::Add(args) => {
                    println!("{} + {} = {}", args.x, args.y, args.x + args.y);
                }
            }
        }
    }
}
