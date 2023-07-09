use clap::{Args, Parser, Subcommand};

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
    #[arg(short, long)]
    x: i32,

    #[arg(short, long)]
    y: i32,
}

fn main() {
    let cli = Cli::parse();

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
