use std::time::Instant;
use std::fs;
use std::path::Path;


fn done(error: bool) {
    if error {
        std::process::exit(-1);
    } else {
        std::process::exit(0);
    }
}


fn main() {
    let start = Instant::now();
    
    let mut debug_file_exist = false;

    //gets all file names in working dir
    let paths = fs::read_dir("./").unwrap();

    //loops for dispplay
    for path in paths {
        println!("{}", path.unwrap().path().display());
    }

    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);
    done(false);
}
