use std::fs;
use std::path::Path;

fn main() {
    let tmp_dir_str = "C:\\Users\\YourUser\\AppData\\Local\\Temp"; // Replace with the desired path
    let tmp_dir = Path::new(tmp_dir_str);

    if tmp_dir.is_dir() {
        if let Ok(entries) = fs::read_dir(tmp_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_file() {
                        if let Err(err) = fs::remove_file(&path) {
                            eprintln!("Error deleting file {:?}: {}", path, err);
                        }
                    } else if path.is_dir() {
                        if let Err(err) = fs::remove_dir_all(&path) {
                            eprintln!("Error deleting folder {:?}: {}", path, err);
                        }
                    }
                }
            }
        } else {
            eprintln!("Error reading temporary folder.");
        }
    } else {
        eprintln!("The specified path is not a valid folder.");
    }
}
