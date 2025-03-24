use clap::{Parser, Subcommand};

mod operations;
use crate::operations:: {
    write_file::*,
    read_file::*,
};

#[derive(Parser)]
#[command(author, version, about, long_about = None,
override_usage = "\
rwcat <COMMAND> [OPTIONS]")]
struct Cli {

    #[command(subcommand)]
    command: Commands,

}

#[derive(Subcommand)]
enum Commands {
    /// <PATH> [QUERY]      -- Read from file
    Read {

        /// Path tot the file to read from
        #[arg(help = "The path to the file to read from")]
        path: String,

        /// The query string to search for, returns all lines of occurence
        #[arg(help = "The query string to search for, returns all lines of occurence", default_value = "")]
        query: String,
    },
    /// <PATH> <CONTENT>    -- Write to file
    Write {

        /// The path to the file to write to
        #[arg(help = "The path to the file to write to")]
        path: String,

        /// The content to write to a new line of the file
        #[arg(help = "The content to write to a new line of the file")]
        content: String,
    },

}

fn main() {

    let cli = Cli::parse();

    match cli.command {
        Commands::Write { path, content } => {
            match write_file(&content, &path) {

                Ok(_) => println!("Writing to file succesfull"),
                Err(e) => println!("Writing to file failed: {e}"),

            }
        }

        Commands::Read { path, query} => {
            match read_file(&path, &query) {

                Ok(_) => {},
                Err(e) => println!("Error reading file: {e}"),

            }
        }
    }


}

