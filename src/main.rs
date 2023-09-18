use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let path_source = String::from(".");

    if args.contains(&"-h".to_string()) {
        println!(" - - DONDETA - - ");
        println!("This script allows two arguments.");
        println!("The first argument is the file you are looking for.");
        println!("The second is optional, is the folder that you want to search from.");
        println!("If you dont add any folder on the second argument, the script will iterate on the current directory.");
        println!("Eg: dondeta [file-name] [folder-search-from]");
    } else if args.len() == 2 {
        let file_to_find = args[1].as_str();
        treeing(&path_source, file_to_find);
    } else if args.len() == 3 {
        let file_to_find = args[1].as_str();
        let path_source = args[2].as_str();
        treeing(&path_source, file_to_find);
    } else {
        println!("Insert a valid argument.")
    }
}

fn treeing(new_dir: &str, file_to_find: &str) {
    match fs::read_dir(new_dir) {
        Ok(data) => {
            for entry in data {
                match entry {
                    Ok(entry) => {
                        if entry.path().is_dir() {
                            treeing(entry.path().to_str().unwrap(), file_to_find);
                        } else if entry.path().is_file() && entry.file_name() == file_to_find {
                            println!("{}", entry.path().to_str().unwrap());
                        }
                    }
                    Err(e) => println!("Error code: {}", e),
                }
            }
        }
        Err(e) => {
            println!("Folder not found");
            println!("Please enter a valid directory");
            println!("Error code: {}", e);
            return;
        }
    }
}
