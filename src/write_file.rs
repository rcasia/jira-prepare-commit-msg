use std::fs::File;
use std::io::Write;

pub fn write_to_file(filename: &str, content: &str) -> () {
    let mut file = File::create(filename).expect("Unable to create file");
    file.write_all(content.as_bytes()).expect("Unable to write data");
}
