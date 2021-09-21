use std::fs;
use std::fs::metadata;

fn is_folder(file: &String) -> bool {
    let file_md = metadata(file);
    match file_md {
        Ok(file_md) => return file_md.is_dir(),
        Err(_) => {
            eprintln!("error: no such directory: \'{}\'", &file);
            std::process::exit(1);
        }
    }
}

fn is_validfile(file: &String) -> bool {
    let file_md = metadata(file);
    match file_md {
        Ok(file_md) => return file_md.is_file() ,
        Err(_) => {
            eprintln!("error: no such file: \'{}\'", &file);
            std::process::exit(1);
        }
    }
}

fn check_file(file: &String) -> bool {
    let file_md = metadata(file);
    match file_md {
        Ok(file_md) => return file_md.is_file()|| file_md.is_dir(),
        Err(_) => {
            eprintln!("error: no such file or directory: \'{}\'", &file);
            std::process::exit(1);
        }
    }
}

fn get_folder_content(folder: &String) -> Vec<String> {
    let paths = fs::read_dir(folder).unwrap();
    let mut path_vec: Vec<String> = Vec::new();
    for path in paths {
        path_vec.push(path.unwrap().path().display().to_string());
    }
    return path_vec;
}

pub fn get_files(raw_files: Vec<String>, path: String) -> Vec<String> {
    let mut complete_files: Vec<String> = Vec::new();
    for file in raw_files {
        if is_validfile(&file) {
            complete_files.push(file.clone());
        } else if is_folder(&file) {
            complete_files.append(&mut get_files(
                get_folder_content(&file),
                path.clone() + &file + "/",
            ));
        }
    }
    return complete_files;
}

pub fn check_only_files(files: &Vec<String>) {
    for file in files {
        if !check_file(&file) {
            eprintln!(
                "error: This is a directory: \'{}\'\nuse \'encrypt -[]f\' for folder support.",
                &file
            );
            std::process::exit(1);
        }
    }
}
