use anyhow::Result;
use makefile_parser_rs::Makefile;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "makefile_parser_rs",
    version = "0.1.0",
    author = "Volodymyr Rastiehaiev <volodymyr.rastiehaiev@ukma.edu.ua>",
    about = "Makefile parser",
    long_about = "This program parses makefiles and displays results to user. Program can also substitutes variables in a makefile.",
    disable_help_flag = true,
    disable_help_subcommand = true
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Parse a makefile and show parsed result
    Parse {
        /// Path to makefile file to parse
        file: String,
    },
    /// Show authors
    About,
    /// Show help
    Help,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Parse { file } => {
            let makefile = Makefile::parse_file(file)?;
            println!("{makefile}");
        },
        Commands::About => {
            println!("Makefile parser rs");
            println!("Verison: 0.1.0");
            println!("Made by: Volodymyr Rastiehaiev <volodymyr.rastiehaiev@ukma.edu.ua>");
        },
        Commands::Help => {
            println!("Makefile parser rs");
            println!("\nUSAGE:");
            println!("\tmakefile_parser_rs <COMMAND> <ARGUMENT>");
            println!("\nCOMMANDS:");
            println!("\tparse <FILEPATH> - Parses makefile and displays the result to user.");
            println!("\n\t\tOPTIONS:");
            println!("\t\t\t<FILEPATH> - path to file, relative to executable. (In case of cargo run, to root dir of project)");
            println!("\n\t\tEXAMPLE:");
            println!("\t\t\t makefile_parser_rs parse Makefile");
            println!("\n\tabout - Shows info about program.");
            println!("\n\thelp - Display this help message.\n");
        }
    }
    Ok(())
}
