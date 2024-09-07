use clap::Parser;
use colored::*;
use rust_tree::print_tree;
use std::io;
use std::path::Path;

// TODO: Common Options
// -a: Show all files, including hidden files.

// bash
// Copy code
// tree -a
// -L: Limit the depth of the tree to a specific number of directory levels.

// bash
// Copy code
// tree -L 2
// This limits the tree display to 2 levels deep.

// -d: List only directories (omit files).

// bash
// Copy code
// tree -d
// -f: Print the full path of each file.

// bash
// Copy code
// tree -f
// -P: Use a wildcard pattern to match files or directories.

// bash
// Copy code
// tree -P "*.rs"
// This shows only files matching a pattern, such as all .rs files.

// -I: Exclude certain files or directories.

// bash
// Copy code
// tree -I "node_modules"
// This excludes node_modules or any other directory you don't want to list.

// --noreport: Suppress the file and directory count at the end of the output.

// bash
// Copy code
// tree --noreport
// --du: Display the size of each file and directory (disk usage).

// bash
// Copy code
// tree --du
// Tree Structure Symbols
// ├──: Represents a file or directory that has more siblings below it.
// └──: Represents the last file or directory in a given level.
// │: Represents a vertical line that connects branches.
// Example of Extended Usage
// bash
// Copy code
// $ tree -L 2 -a -d
// .
// ├── .hidden
// ├── bin
// ├── src
// │   └── tests
// └── Cargo.toml
// The -L 2 option limits the tree depth to 2 levels.
// The -a option includes hidden files or directories (like .hidden).
// The -d option only lists directories.
// This command is particularly useful for system administrators or developers who want to visualize directory structures or locate files easily without opening file explorers. You can also redirect the output of tree to a file for documentation purposes:

// bash
// Copy code
// tree > structure.txt
// This command saves the tree structure to a structure.txt file.

/// Simple tree bin built with rust
#[derive(Parser, Debug)]
#[command(name = "my_tree")]
#[command(version = "1.0")]
#[command(author = "Mohamed <matrixhc00@gmail.com>")]
#[command(about = "Prints the directory tree structure")]
struct Cli {
    /// Show hidden files and directories
    #[arg(short, long)]
    all: bool,
}

fn main() -> io::Result<()> {
    let args = Cli::parse();
    let start_path = Path::new("./");

    println!("{}", ".".blue().bold());

    // Print the tree structure
    let (dir_count, file_count, total_size) = print_tree(start_path, "", args.all)?;

    // Print the summary
    println!(
        "\n{} directories, {} files, {:.2} MB total size",
        dir_count.to_string().yellow(),
        file_count.to_string().yellow(),
        total_size as f64 / (1024.0 * 1024.0)
    );
    println!("{}", args.all);

    Ok(())
}
