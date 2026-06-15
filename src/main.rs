use std::cmp::Reverse;
use std::env;
use std::fmt;
use std::io::Result;
use std::path::{Path, PathBuf};
use std::{fs, io};
enum EntryType {
    Dir { items: usize },
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
            EntryType::Dir { items } => write!(f, "| Directory  - {} items inside", items),
            EntryType::File => write!(f, "| File"),
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
    depth: Option<usize>,
}
fn top_flag(args: &[String]) -> Option<usize> {
    flag_value("--top", args)
}
fn depth_flag(args: &[String]) -> Option<usize> {
    flag_value("--depth", args)
}
fn flag_value(flag: &str, args: &[String]) -> Option<usize> {
    args.iter()
        .position(|a| a == flag)
        .and_then(|s| args.get(s + 1).and_then(|i| i.parse::<usize>().ok()))
}
impl Config {
    fn from_args() -> Result<Config> {
        let args: Vec<String> = env::args().collect();
        let path = Path::new(match args.get(1) {
            Some(path) => path,
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "no path provided",
                ));
            }
        });
        let config = Config {
            path: path.to_path_buf(),
            number_rows: top_flag(&args),
            depth: depth_flag(&args),
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
                EntryType::Dir {
                    items: entries_from_path(path).map(|v| v.len()).unwrap_or(0),
                }
            } else {
                EntryType::File
            },
        })
        .collect();
    entries_size.sort_by_key(|e| Reverse(e.size));
    Ok(entries_size)
}

fn print_tree(path: &Path, level: usize, depth: usize, top: usize) -> io::Result<()> {
    let prefix = "│  ".repeat(level);
    let entries: Vec<_> = collect_entries(path)?.into_iter().take(top).collect();
    for (i, entry) in entries.iter().enumerate() {
        let connector = if i == entries.len() - 1 {
            "└─"
        } else {
            "├─"
        };
        println!(
            "{prefix}{connector} {} - {} - {}",
            entry
                .path
                .file_name()
                .and_then(|a| a.to_str())
                .unwrap_or("---"),
            entry.readable_size(),
            entry.entry_type
        );
        match &entry.entry_type {
            EntryType::Dir { .. } => {
                if level + 1 < depth {
                    print_tree(&entry.path, level + 1, depth, top)?;
                }
            }
            EntryType::File => {}
        }
    }

    Ok(())
}
fn report(path: &Path, top: usize, depth: usize) -> io::Result<()> {
    let entries = collect_entries(path)?;
    let total_size: u64 = entries.iter().map(|entry| entry.size).sum();
    print_tree(path, 0, depth, top)?;
    println!("Total size - {}", readable_size(total_size));
    Ok(())
}
fn main() -> io::Result<()> {
    let config = Config::from_args()?;
    println!("{}", config.path.display());
    report(
        &config.path,
        config.number_rows.unwrap_or(usize::MAX),
        config.depth.unwrap_or(1),
    )?;

    Ok(())
}
