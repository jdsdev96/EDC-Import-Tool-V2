use std::time::Instant;
mod dir_file;
mod js_pause;


fn done(error: bool) {
    println!("\x1b[0mAttempt");
    js_pause::pause();
    if error {
        std::process::exit(-1);
    } else {
        std::process::exit(0);
    }
}


fn main() {
    //start the stopwatch for execution time
    let start = Instant::now();

    let version = String::from("2.0.0");

    // print the title of the program as magenta and underlined
    println!("\x1b[95m\x1b[4mImport Events Tool v2.0");
    
    //reset the text color
    print!("\x1b[0m");

    //print the version number
    println!("v{version}");


    let debug_file_exist = dir_file::does_file_exist(String::from("debug.txt"));

    if debug_file_exist {
        println!("Debug Mode.");
        let wrk_dir = dir_file::get_cur_dir();
        println!("{:?}", wrk_dir);
    } else {
        println!("Production Mode.");
    }

    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);
    done(false);
}
