use clap::Parser;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,
}


fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    if let Some(path) = cli.config.as_deref() {
        let file = File::open(path)?;
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
    }

    Ok(())
}
