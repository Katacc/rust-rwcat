use std::fs;

pub fn read_file(path: &String, query_string: &String) -> std::io::Result<()> {

        let mut line_number = 0;
        let lower_query: String = query_string.trim().to_lowercase();


        match fs::read_to_string(&path) {
            Ok(contents) => {
                if query_string.is_empty() {
                    println!("{contents}");
                } else {
                    let contents_lines = contents.lines();
                    for line in contents_lines {
                        let lower_line: String = line.trim().to_lowercase();
                        line_number += 1;
                        if lower_line.contains(&lower_query) {
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