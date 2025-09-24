    use std::fs::{File, OpenOptions};
    use std::io::{Read, Write};

    fn main() {
        match File::create("test-write.txt") {
            Ok(file) => {
                let mut file = file;
                match file.write_all(b"Hello world from Rust \n") {
                    Ok(_) => println!("File written successfully."),
                    Err(e) => println!("Unable to write to file: {}", e),
                }
            },
            Err(e) => {
                println!("Failed to create file: {}", e);
                return;
            }
        }

        match File::open("test-write.txt") {
            Ok(file) => {
                let mut file = file;
                let mut buf = String::new();
                match file.read_to_string(&mut buf) {
                    Ok(_) => {
                        println!("\nSuccessfully read file contents:");
                        print!("{}", buf); // Since we added a newline to the string that is being written to file.
                    }
                    Err(e) => println!("Unable to read to file: {}", e)
                }
            }
            Err(e) => {
                print!("Failed to open file: {}", e);
                return;
            }
        }

        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("test-write.txt");

        match file {
            Ok(mut file) => {
                let content = "Appending more content to text file \n";
                match file.write_all(content.as_bytes()) {
                    Ok(_) => println!("File appended successfully."),
                    Err(e) => println!("Failed to append to file: {}", e),
                }
            }
            Err(e) => {
                println!("Failed to open file: {}", e);
                return;
            }
        }

        match File::open("test-write.txt") {
            Ok(file) => {
                let mut file = file;
                let mut buf = String::new();
                match file.read_to_string(&mut buf) {
                    Ok(_) => {
                        println!("\nSuccessfully read file contents:");
                        print!("{}", buf); // Since we added a newline to the string that is being written to file.
                    }
                    Err(e) => println!("Unable to read to file: {}", e)
                }
            }
            Err(e) => {
                print!("Failed to open file: {}", e);
                return;
            }
        }

}
