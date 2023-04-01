use std::env;
use std::fs::{self, DirEntry};
use std::path::Path;
use std::collections::HashSet;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: directorycleanup <path>");
        return;
    }
    let path = &args[1];
    cleanup_directory(&path);
}

fn cleanup_directory<P: AsRef<Path>>(path: P) {
    let entries = match fs::read_dir(&path) {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("Error reading directory: {:?}", e);
            return;
        }
    };

    let mut extensions = HashSet::new();
    for entry_result in entries {
        let entry = match entry_result {
            Ok(entry) => entry,
            Err(e) => {
                eprintln!("Error reading entry: {:?}", e);
                continue;
            }
        };
        if let Some(ext) = entry.path().extension() {
            let ext_str = ext.to_string_lossy().to_lowercase();
            if !extensions.contains(&ext_str) {
                extensions.insert(ext_str.clone());
                let new_dir = path.as_ref().join(&ext_str);
                if !new_dir.exists() {
                    if let Err(e) = fs::create_dir(&new_dir) {
                        eprintln!("Error creating directory {}: {:?}", ext_str, e);
                        continue;
                    }
                }
            }
            let path_str = path.as_ref().to_str().expect("Path is not valid UTF-8");
            move_file(&entry, &path_str, &ext_str);
        }
    }
}


fn move_file(entry: &DirEntry, path: &str, ext: &str) {
    let new_path = Path::new(path).join(ext).join(entry.file_name());
    if let Err(e) = fs::rename(&entry.path(), &new_path) {
        eprintln!("Error moving file {:?} to {:?}: {:?}", entry.path(), new_path, e);
    }
}
