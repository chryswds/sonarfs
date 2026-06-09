use std::env;
use std::io::Result;
use std::{fs, io};





fn main() -> io::Result<()> {




    let args: Vec<String> = env::args().collect();
    
    let file_path = &args[1];


    let mut entries = fs::read_dir(file_path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>>>()?;

    entries.sort();
    
    let metadata = fs::metadata(&entries[0])?;
    
    let size = metadata.len();

    println!("{}", size);


    Ok(())



}
