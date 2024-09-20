use std::{
    error::Error, 
    ffi::{OsStr, OsString}, 
    fs::read_dir,
    process,
};

fn main() -> Result<(), Box<dyn Error>>{
    let mut args = std::env::args_os();
    let mut target_dir = OsString::from(r".\");

    let bin_name = args.next().unwrap_or_else(|| {
        eprintln!("Could not get bin name");
        std::process::exit(1);
    });

    println!("{bin_name:?}");

    if let Some(t) = args.next(){
        target_dir = t.clone();
    }
    
    let _ = dir_travel(&target_dir)?;

    

    Ok(())
}


fn dir_travel(path: &OsStr) -> Result<(), Box<dyn Error>>{

    let dir_reader = read_dir(path)?;
    for dir_entry in dir_reader{

        let dir_entry = dir_entry.unwrap_or_else(|err| {
            eprintln!("{:?}", err.kind());
            process::exit(1);
        });

        match dir_entry.file_type(){
            Ok(i) => {
                
                if i.is_dir(){
                    println!("Dirname: {:?}", dir_entry.file_name());
                    dir_travel(dir_entry.path().as_os_str())?
                }else{
                    println!("Filename: {:?}", dir_entry.file_name());
                }
            },
            Err(e) => return Err(Box::new(e)),
        }
    }
    Ok(())
}