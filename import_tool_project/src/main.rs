use std::time::Instant;


fn done(error: bool) {
    if error {
        std::process::exit(-1);
    } else {
        std::process::exit(0);
    }
}


fn main() {
    let test = Instant::now();

}
