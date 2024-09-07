use colored::*;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

fn format_size(size: u64) -> String {
    const MB: u64 = 1024 * 1024;
    const KB: u64 = 1024;

    if size >= MB {
        format!("{:.2} MB", size as f64 / MB as f64)
    } else {
        format!("{:.2} KB", size as f64 / KB as f64)
    }
}

pub fn print_tree(dir: &Path, prefix: &str) -> io::Result<(usize, usize, u64)> {
    let mut file_count = 0;
    let mut dir_count = 0;
    let mut total_size = 0;

    if dir.is_dir() {
        let entries = fs::read_dir(dir)?
            .map(|entry| entry.map(|e| e.path()))
            .collect::<Result<Vec<PathBuf>, io::Error>>()?;

        let mut files = vec![];
        let mut directories = vec![];

        for path in entries {
            if let Some(name) = path.file_name() {
                if name.to_string_lossy().starts_with('.') {
                    continue;
                }
            }
            if path.is_dir() {
                directories.push(path);
            } else if path.is_file() {
                files.push(path);
            } else {
                eprintln!("Skipping unexpected entry: {:?}", path);
            }
        }

        for (i, file) in files.iter().enumerate() {
            let is_last_entry = directories.is_empty() && i == files.len() - 1;
            let file_size = fs::metadata(file)?.len();
            total_size += file_size;
            println!(
                "{}{} {} ({})",
                prefix,
                if is_last_entry {
                    "└──"
                } else {
                    "├──"
                },
                file.file_name().unwrap().to_string_lossy().green(),
                format_size(file_size)
            );
            file_count += 1;
        }

        for (i, directory) in directories.iter().enumerate() {
            let is_last_entry = i == directories.len() - 1;
            let (d_count, f_count, dir_size) = print_tree(
                directory,
                &format!("{}{}", prefix, if is_last_entry { "    " } else { "│   " }),
            )?;
            total_size += dir_size;
            dir_count += d_count + 1;
            file_count += f_count;
            println!(
                "{}{} {} ({})",
                prefix,
                if is_last_entry {
                    "└──"
                } else {
                    "├──"
                },
                directory
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .blue()
                    .bold(),
                format_size(dir_size)
            );
        }
    }

    Ok((dir_count, file_count, total_size))
}
