use clap::{Subcommand, Parser};
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// does testing things
    #[command(arg_required_else_help = true)]
    Deduplicate {
        /// Sets a custom config file
        #[arg(short, long, value_name = "FILE")]
        config: Option<PathBuf>,
    },
}


fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Deduplicate { config } => {
            println!("Deduplicate {config:#?}");
            if let Some(dict_file) = config {
                return deduplicate(dict_file);
            }
        }
    }

    Ok(())
}

fn deduplicate(dict_file: PathBuf) -> std::io::Result<()> {
    let file = File::open(dict_file)?;
    let buf_reader = BufReader::new(file);

    let mut lines: HashSet<String> = HashSet::new();
    let mut data_start = false;
    for line in buf_reader.lines() {
        let line = line?;
        if line == "..." {
            data_start = true;
        } 
        if ! data_start {   // keep origin lines before "..."
            println!("{}", line);
        } else if ! lines.contains(&line) {
            println!("{}", line);
            lines.insert(line);
        } // else ignore duplication lines
    }
    Ok(())
}
