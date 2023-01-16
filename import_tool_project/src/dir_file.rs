use std::path::{PathBuf};
use std::fs;
use std::env;

pub fn does_file_exist(file_name: String) -> bool {
    let files = files_in_wrk_dir();
    match files {
        Ok(files) => {
            for file in files.iter() {
                if &file_name == file {
                    return true;
                } else {
                    continue;
                } 
            }
            return false;
        }
        Err(e) => {
            println!("An error occurred while getting the vector of file names. {}", e);
        }      
    }
    

    
    return false;
}

pub fn files_in_wrk_dir() -> Result<Vec<String>, std::io::Error> {
    let cur_dir = get_cur_dir()?;
    let paths = fs::read_dir(cur_dir)?;
    let mut file_names = vec![];
    for path in paths {
        let path = path?.path();
        if path.is_file() {
            file_names.push(path.file_name().unwrap().to_str().unwrap().to_string());
        }
    }
    Ok(file_names)
}

pub fn get_cur_dir() -> std::io::Result<PathBuf> {
    env::current_dir()
}