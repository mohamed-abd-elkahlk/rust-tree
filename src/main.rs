use clap::Parser;
use colored::*;
use rust_tree::print_tree;
use std::io;
use std::path::Path;

/// Simple program to greet a person
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() -> io::Result<()> {
    // let args = Cli::parse();
    let start_path = Path::new("./");

    println!("{}", ".".blue().bold());

    // Print the tree structure
    let (dir_count, file_count, total_size) = print_tree(start_path, "")?;

    // Print the summary
    println!(
        "\n{} directories, {} files, {:.2} MB total size",
        dir_count.to_string().yellow(),
        file_count.to_string().yellow(),
        total_size as f64 / (1024.0 * 1024.0)
    );
    Ok(())
}
