use std::time::Instant;
mod dir_file;

fn done(error: bool) {
    if error {
        std::process::exit(-1);
    } else {
        std::process::exit(0);
    }
}


fn main() {
    let start = Instant::now();
    
    let debug_file_exist = dir_file::does_file_exist(String::from("debug.txt"));

    if debug_file_exist {
        println!("Debug Mode.")
    } else {
        println!("Production Mode.")
    }
    


    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);
    done(false);
}
