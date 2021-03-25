use std::env;
use std::fs;
use std::io;
use std::path::{Path};
use std::io::{Error};

fn main() -> io::Result<()> {
    // Collecting command line arguments
    let args: Vec<String> = env::args().collect();

    // Making sure there is a filepath provided
    assert_eq!(args.len(), 2);

    // Getting that filepath provided to us by the user
    let file_path = &args[1];
    let start_path = Path::new(file_path);

    // If the path provided to us is a directory, read its entries
    if start_path.is_dir() {
        match read_files_in_dir(start_path) {
            Ok(file_names) => {
                for file_name in file_names {
                    println!("{}", file_name)
                }
            }
            Err(e) => eprintln!("Error reading files in directory: {}", e)
        }
    } else {
        // If the path provided is a single file, just print its name
        println!("{}", start_path.file_name().unwrap().to_string_lossy());
    }

    Ok(())
}

fn read_files_in_dir(directory: &Path) -> Result<Vec<String>, Error> {
    let mut file_vector = Vec::new();
    // Iterate over entries in the directory
    for entry in fs::read_dir(directory)? {
        let entry = entry?;

        // Get file name
        let os_file_name = entry.file_name();
        file_vector.push(String::from(os_file_name.to_string_lossy()));
    }
    Ok(file_vector)
}
