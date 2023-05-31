fn main() {
    println!("{}", get_folder_contents());
}







fn get_current_directory() -> std::path::PathBuf {
    let current_directory = std::env::current_dir().unwrap();
    return current_directory
}


fn get_folder_contents() -> String {
    let contents = std::fs::read_dir(get_current_directory()).unwrap();

    let mut files = Vec::new();
    let mut folders = Vec::new();
    let mut symlinks = Vec::new();
    let mut read_only = Vec::new();

    for content in contents {
        let content = content.unwrap();
        let path = content.path();
        
        let file_or_folder_name = String::from((&path).file_name().unwrap().to_str().unwrap());

        if content.file_type().unwrap().is_dir() {
            folders.push(file_or_folder_name);
        } else if content.file_type().unwrap().is_file() && content.metadata().unwrap().permissions().readonly() {
            read_only.push(file_or_folder_name);
        } else if content.file_type().unwrap().is_file() {
            files.push(file_or_folder_name);
        } else if content.file_type().unwrap().is_symlink() {
            symlinks.push(file_or_folder_name);
        }
    }
    



    let mut output:Vec<String> = Vec::new();

    for folder in folders {
        output.push(format!("\x1b[34m{}\x1b[0m ", folder));
    }
    for file in files {
        output.push(format!("\x1b[32m{}\x1b[0m ", file));
    }
    for read_only_file in read_only {
        output.push(format!("\x1b[31m{}\x1b[0m ", read_only_file));
    }
    for symlink in symlinks {
        output.push(format!("\x1b[35m{}\x1b[0m ", symlink));
    }




    //will reverse the order of the output if uncommented 
    //output.sort();
    return output.join(" ");






}