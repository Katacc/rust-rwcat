use std::fs;

pub fn read_file(path: &String, query_string: &String) -> std::io::Result<()> {

        let mut line_number = 0;

        match fs::read_to_string(&path) {
            Ok(contents) => {
                if query_string.is_empty() {
                    println!("{contents}");
                } else {
                    let contents_lines = contents.lines();
                    for line in contents_lines {
                        line_number += 1;
                        if line.contains(query_string) {
                            println!("{line_number}: {line}");
                        }
                    }
                }
            },
            Err(_) => {
                println!("Error accessing file (file missing?)");
            }
        }

Ok(())
}