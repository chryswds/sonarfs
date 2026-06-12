use std::env;
use std::io::Result;
use std::path::{Path, PathBuf};
use std::{fs, io};

// Translates bits sizes to readable (B, KB, MB and GB)
fn readable_size(bytes: u64) -> String {
    let base = 1024;
    let float_value = bytes as f64;
    if bytes < base {
        format!("{bytes} B")
    } else if bytes < base.pow(2) {
        format!("{:.1} KB", float_value / base as f64)
    } else if bytes < base.pow(3) {
        format!("{:.1} MB", float_value / base.pow(2) as f64)
    } else {
        format!("{:.1} GB", float_value / base.pow(3) as f64)
    }
}

// Returns entry size, if a folder dir_size() if a file metdata.len()
fn entry_size(entry: &Path) -> u64 {
    let metadata = match fs::metadata(entry) {
        Ok(m) => m,
        Err(_) => return 0,
    };
    if metadata.is_dir() {
        dir_size(entry).unwrap_or(0)
    } else {
        metadata.len()
    }
}

// returs the size of a folder
fn dir_size(dir_path: &Path) -> io::Result<u64> {
    let entries = fs::read_dir(dir_path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>>>()?;
    let mut total_folder_size: u64 = 0;
    for entry in &entries {
        total_folder_size += entry_size(entry);
    }
    Ok(total_folder_size)
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);
    println!("{}", path.display());

    let entries = fs::read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>>>()?;

    let mut entries_by_size: Vec<(u64, &PathBuf)> = Vec::new();

    for entry in &entries {
        entries_by_size.push((entry_size(entry), entry));
    }

    entries_by_size.sort_by_key(|(s, _)| std::cmp::Reverse(*s));

    let mut total_size: u64 = 0;
    for (size, path) in &entries_by_size {
        println!("{} ---- {}", path.display(), readable_size(*size));
        total_size += size;
    }

    println!("{}", readable_size(total_size));
    Ok(())
}
