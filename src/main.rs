use std::env;
use std::io::Result;
use std::{fs, io};

fn readable_size(bytes: u64) -> String {
    let base: u64 = 1024;

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

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let mut entries = fs::read_dir(file_path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>>>()?;

    entries.sort();

    println!("{:?}", file_path);

    let mut total_size: u64 = 0;

    for file in &entries {
        let metadata = fs::metadata(file)?;

        let size = metadata.len();

        let readable_size = readable_size(size);
        total_size += size;

        println!("{:?} - size = {}", file, readable_size);
    }

    let readable_total_size = readable_size(total_size);

    println!("Total size = {}", readable_total_size);

    Ok(())
}
