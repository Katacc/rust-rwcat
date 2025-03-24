use std::io::Write;
use std::fs:: { File, OpenOptions };
use std::path::Path;

pub fn write_file(string: &String, path: &String) -> std::io::Result<()> {

    if !Path::new(path).exists() {
        let _file = File::create(&path)?;
    }

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(path)
        .expect("Failed to open file.");


    let metadata = Path::new(path).metadata();
    if metadata?.len() == 0 {
        let write_string = String::new() + &string;
        file.write_all(write_string.as_bytes())?;
    } else {
        let write_string = String::new() + "\n" + &string;
        file.write_all(write_string.as_bytes())?;
    }

    Ok(())

}