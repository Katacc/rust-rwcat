use std::io::prelude::*;
use std::fs;
use std::fs:: {
    File, OpenOptions
};
use std::env;
use std::path::Path;



fn main() {

    // Command line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {

        if &args[1] == "-w" && args.len() == 4 {
            let string = &args[3];
            let path = &args[2];


            // Use match to catch error or succesfull run message
            // write_file returns Result<()> so Ok, or Err
            // match catches it and then we can parse it in a block
            match write_file(&string, &path) {
                Ok(_) => {
                    println!("Writing succesfull");
                },
                Err(e) => {
                    println!("Writing to file failed {e}");
                },
            }

        } else if &args[1] == "-r" {
            let path = &args[2];

            match read_file(&path) {
                Ok(_) => {
                },
                Err(e) => {
                    println!("Reading failed {e}");
                },
            }

        } else {
            println!("Use -w or -r to write or read");
            println!("-w <path> <string>    -- To write the contents of <string> to a file <path>");
            println!("-r <path>             -- To read the contents of file <path>");
        }

    } else {
        println!("Too few arguments... \n Options: ");

        println!("-w <path> <string>    -- To write the contents of <string> to a file <path>");
        println!("-r <path>             -- To read the contents of file <path>");

    }


}



fn read_file(path: &String) -> std::io::Result<()> {
    println!("Contents of {path}:");
    println!("---");

    let contents = fs::read_to_string(&path)
        .expect("Failed reading file...");

    println!("{contents}");

    Ok(())
}


fn write_file(string: &String, path: &String) -> std::io::Result<()> {

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
