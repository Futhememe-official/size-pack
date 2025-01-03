use clap::Parser;
use colored::Colorize;

/// Search for a pattern in a file and display the lines that contain it
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    println!("\n\n\n pattern: {:?}, path: {:?}", args.pattern, args.path);
    println!("{}", "Finalizado !".green().bold());
}
