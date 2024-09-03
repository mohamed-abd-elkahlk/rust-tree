use std::{env, fs, path::PathBuf};

use anyhow::Result;

fn view_content(path: &PathBuf) -> Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if let Some(file_name) = path.file_name() {
            if let Some(file_name_str) = file_name.to_str() {
                println!("{}", file_name_str);
            }
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let currun_dir = env::current_dir()?;
    view_content(&currun_dir)
}
