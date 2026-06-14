use std::env;
use std::io::Result;
use std::path::{Path, PathBuf};
use std::{fs, io};

// Struct of an entry path and size
struct Entry {
    size: u64,
    path: PathBuf,
}
//impl fn to entry
impl Entry {
    //size is already in the entry struct no need to give the value to readable_size()
    fn readable_size(&self) -> String {
        readable_size(self.size)
    }
}

// Translates bits sizes to readable (B, KB, MB and GB)
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

// return entries from a path
fn entries_from_path(path: &Path) -> Result<Vec<PathBuf>> {
    fs::read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect()
}

// returns the size of a folder
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

fn main() -> io::Result<()> {
   let config = Config::from_args()?
       ;
    // entries are a collection of entries from the path given, so individual items in this path
    let entries = entries_from_path(config.path.as_path())?;

    // entries_size is a vec of the struct entry, iterates through the items of the struct and collects the size of each entry
    let mut entries_size: Vec<Entry> = entries
        .iter()
        .map(|path| Entry {
            size: entry_size(path),
            path: path.to_path_buf(),
        })
        .collect();


    // sorts the entries by bigger size
    entries_size.sort_by_key(|entry| std::cmp::Reverse(entry.size));

    //total_size is a sum of all sizes in the entry struct
    let total_size: u64 = entries_size.iter().map(|entry| entry.size).sum();


    // displaying the entries, if display_rows is still set to none, meaning there was no --top tag, it sets it to the maximum usize, meaning all items in the directory will be shown
    for entry in entries_size.iter().take(config.number_rows.unwrap_or(usize::MAX)) {
        println!(
            "{} ---- {}",
            entry.path.display(),
            entry.readable_size(),
        );
    }

    // prints total size
    println!("Total size - {}", readable_size(total_size));
    Ok(())
}
