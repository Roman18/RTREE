use std::{
    error::Error, 
    ffi::OsStr, 
    fs::read_dir,
};
use clap::Parser;

mod argparse;

use argparse::Args;


fn main() -> Result<(), Box<dyn Error>>{

    let args = Args::parse();
 
    let target_dir = args.name.unwrap();
    let print_all = args.all;

    let max_depth = args.depth;

    let _ = dir_travel(&target_dir, 0, max_depth, print_all)?;

    Ok(())
}


fn dir_travel(path: &OsStr, depth: usize, max_depth: usize, print_all: bool) -> Result<(), Box<dyn Error>>{

    if max_depth <= 0{
        return Ok(());
    }

    let dir_reader = read_dir(path)?;
    for dir_entry in dir_reader{

        if let Ok(entry) = dir_entry{

            let entry_type = entry.file_type();
            let entry_path = entry.path();
            let entry_name = entry.file_name();
            // Check if the file name starts with a dot symbol (according to the unix it is hidden file)
            if entry_name.as_encoded_bytes()[0] == b'.' && !print_all{
                continue;
            }

            if let Ok(t) = entry_type{

                if t.is_dir(){
                    println!("D: {}{:?}:", "\t".repeat(depth), entry_name);
                    let _ = dir_travel(
                        entry_path.as_os_str(), 
                        depth + 1, 
                        max_depth - 1,
                        print_all
                    );

                }else{
                    println!("F: {}{:?}", "\t".repeat(depth), entry_name);
                }

            }
        }else{
            eprintln!("[-] Could not operate on entry.")
        }
        
    }
    Ok(())
}