use std::{
    error::Error, 
    ffi::{OsStr, OsString}, 
    fs::read_dir,
};

fn main() -> Result<(), Box<dyn Error>>{
    let mut args = std::env::args_os();
    let mut target_dir = OsString::from(r".\");

    let _bin_name = args.next().unwrap_or_else(|| {
        eprintln!("Could not get bin name");
        std::process::exit(1);
    });

    let max_depth = 3;

    if let Some(t) = args.next(){
        target_dir = t.clone();
    }
    
    let _ = dir_travel(&target_dir, 0, max_depth)?;

    

    Ok(())
}


fn dir_travel(path: &OsStr, depth: usize, max_depth: usize) -> Result<(), Box<dyn Error>>{

    if max_depth <= 0{
        return Ok(());
    }

    let dir_reader = read_dir(path)?;
    for dir_entry in dir_reader{

        if let Ok(entry) = dir_entry{

            let entry_type = entry.file_type();
            let entry_path = entry.path();
            let entry_name = entry.file_name();
            if let Ok(t) = entry_type{

                if t.is_dir(){
                    println!("D: {}{:?}:", "\t".repeat(depth), entry_name);
                    let _ = dir_travel(entry_path.as_os_str(), depth + 1, max_depth - 1);
                }else{
                    println!("F: {}{:?}", "\t".repeat(depth), entry_name);
                }
            }
        }else{
            eprintln!("[-] Could not operate with entry.")
        }
        
    }
    Ok(())
}