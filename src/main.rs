use std::env;
use std::io::Result;
use std::path::{Path, PathBuf};
use std::{fs, io};
use std::cmp::Reverse;
use std::fmt;
enum EntryType {
    Dir,
    File,
}
struct Entry {
    size: u64,
    path: PathBuf,
    entry_type: EntryType,
}
impl fmt::Display for EntryType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EntryType::Dir => write!(f, "dir"),
            EntryType::File => write!(f, "file"),
        }
    }
}
impl Entry {
    //size is already in the entry struct no need to give the value to readable_size()
    fn readable_size(&self) -> String {
        readable_size(self.size)
    }
}
fn readable_size(bytes: u64) -> String {
    let base = 1024;
    let float_value = bytes as f64;
    if bytes < base {
        format!("{} B", bytes)
    } else if bytes < base.pow(2) {
        format!("{:.1} KB", float_value / base as f64)
    } else if bytes < base.pow(3) {
        format!("{:.1} MB", float_value / base.pow(2) as f64)
    } else {
        format!("{:.1} GB", float_value / base.pow(3) as f64)
    }
}
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
fn entries_from_path(path: &Path) -> Result<Vec<PathBuf>> {
    fs::read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect()
}
fn dir_size(dir_path: &Path) -> io::Result<u64> {
    let entries = entries_from_path(dir_path)?;
    let mut total_folder_size: u64 = 0;
    for entry in &entries {
        total_folder_size += entry_size(entry);
    }
    Ok(total_folder_size)
}
struct Config {
    path: PathBuf,
    number_rows: Option<usize>,
}
impl Config {
    fn from_args() -> Result<Config> {
        let args: Vec<String> = env::args().collect();
        let path = Path::new( match args.get(1){
            Some(path) => path,
            None => return Err(io::Error::new(io::ErrorKind::InvalidInput, "no path provided")),
        });
        let mut display_rows: Option<usize> = None;
        if let Some(arg) = args.get(2)
            && arg == "--top"
        {
            display_rows = args.get(3).and_then(|s| s.parse::<usize>().ok());
        };
        let config = Config {
            path: path.to_path_buf(),
            number_rows: display_rows,
        };
        Ok(config)
    }
}
fn collect_entries(path: &Path) -> Result<Vec<Entry>> {
    let entries = entries_from_path(path)?;
    let mut entries_size: Vec<Entry> = entries
        .iter()
        .map(|path| Entry {
            size: entry_size(path),
            path: path.to_path_buf(),
            entry_type: if path.is_dir() {
                EntryType::Dir
            } else {
                EntryType::File
            }
        })
        .collect();
    entries_size.sort_by_key(|e|Reverse(e.size));
    Ok(entries_size)
}
fn display_dir(entries: &[Entry],size: Option<usize>){
   let total_size: u64 = entries.iter().map(|entry| entry.size).sum();
    for entry in entries.iter().take(size.unwrap_or(usize::MAX)) {
        println!("{} - {} - {}", entry.path.display(), entry.readable_size(), entry.entry_type);
    };
    println!("Total size - {}", readable_size(total_size));
}
fn main() -> io::Result<()> {
    let config = Config::from_args()?;
    println!("{}", config.path.display());
    let  entries_size = collect_entries(&config.path)?;
    display_dir(entries_size.as_ref(), config.number_rows);
    Ok(())
}
