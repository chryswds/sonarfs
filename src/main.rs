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

// returns the size of a folder
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

struct Entry {
    size: u64,
    path: PathBuf,
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);
    println!("{}", path.display());
    let mut display_rows: Option<usize> = None;
    if let Some(arg) = args.get(2)
        && arg == "--top"
    {
        display_rows = args.get(3).and_then(|s| s.parse::<usize>().ok());
    }
    let entries = fs::read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>>>()?;
    let mut entries_size: Vec<Entry> = entries
        .iter()
        .map(|path| Entry {
            size: entry_size(path),
            path: path.to_path_buf(),
        })
        .collect();
    entries_size.sort_by_key(|entry| std::cmp::Reverse(entry.size));
    let total_size: u64 = entries_size.iter().map(|entry| entry.size).sum();
    for entry in entries_size
        .iter()
        .take(display_rows.unwrap_or(usize::MAX))
    {
        println!(
            "{} ---- {}",
            entry.path.display(),
            readable_size(entry.size)
        );
    }
    println!("Total size - {}", readable_size(total_size));
    Ok(())
}
