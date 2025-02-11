use std::fs;
use std::path::Path;

/// Gets the names of all subdirectories in the given directory.
pub fn subdir_names(dir: &Path) -> Result<Vec<String>, String> {
    let mut names = Vec::new();
    let dir_iter = fs::read_dir(dir).map_err(|e| e.to_string())?;

    for entry in dir_iter {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.is_dir() {
            names.push(entry.file_name().to_string_lossy().into_owned());
        }
    }
    Ok(names)
}

/// Recursively copies all contents of the source directory to the destination directory.
/// Creates the destination directory if it does not exist.
/// Does not overwrite existing files.
pub fn copy_dir(src: &Path, dest: &Path) {
    fs::create_dir_all(dest).unwrap();
    let dir_iter = fs::read_dir(src).unwrap();

    for entry in dir_iter {
        let entry = entry.unwrap();
        let src_path = entry.path();
        let dest_path = dest.join(entry.file_name());

        if src_path.is_dir() {
            copy_dir(&src_path, &dest_path);
        } else if dest_path.exists() {
            println!("File already exists: {:?}", dest_path);
        } else {
            fs::copy(&src_path, &dest_path).unwrap();
        }
    }
}

/// Copies all subdirectories of the source directory to the destination directory.
/// Creates the destination directory if it does not exist.
/// Each subdirectory is copied recursively including all files and subdirectories.
/// Does not overwrite existing files.
pub fn copy_subdirs(src: &Path, dest: &Path) {
    let dir_iter = match fs::read_dir(src) {
        Ok(iter) => iter,
        Err(e) => {
            eprintln!("Failed to read directory {:?}: {:?}", src, e);
            return;
        }
    };

    println!("Copying subdirectories:");
    println!("  source: {:?}", src);
    println!("  destination: {:?}", dest);

    for entry in dir_iter {
        let entry = entry.unwrap();
        let src_path = entry.path();
        let dest_path = dest.join(entry.file_name());

        if src_path.is_dir() {
            copy_dir(&src_path, &dest_path);
        }
    }
}

/// Copies all files from the given list of filenames
/// from the source directory to the destination directory.
/// Creates the destination directory if it does not exist.
/// Does not overwrite existing files.
pub fn copy_files(files: &[String], src: &Path, dest: &Path) {
    fs::create_dir_all(dest).unwrap();

    for file in files {
        let src_path = src.join(file);
        let dest_path = dest.join(file);

        if dest_path.exists() {
            println!("File already exists: {:?}", dest_path);
        } else {
            fs::copy(&src_path, &dest_path).unwrap();
        }
    }
}

/// Appends the given content as a new line to the file with the given name in the given directory.
/// Only appends the content if the file exists and doesn't already contain the line.
/// Ensures that the new line is preceded with a newline and the file ends with a newline character.
/// Doesn't create the file if it doesn't exist.
pub fn append_to_file(file: &Path, content: &str) {
    if !file.exists() {
        return;
    }

    let content = content.trim_end();
    let file_lines = fs::read_to_string(file).unwrap_or_default();
    let mut file_lines: Vec<&str> = file_lines.lines().collect();
    if file_lines.iter().any(|line| line == &content) {
        return;
    }
    if file_lines.last().map_or(true, |line| !line.is_empty()) {
        file_lines.push("");
    }
    file_lines.push(content);

    let file_content = format!("{}\n", file_lines.join("\n"));

    fs::write(file, file_content).unwrap();
}

/// Returns a list of files in the given directory that have the given suffix.
/// Note that the suffix is just a string suffix, not a file extension.
/// I.e. to catch a file extension, the dot must be included in the suffix.
pub fn files_with_suffix(dir: &Path, suffix: &str) -> Result<Vec<String>, String> {
    let mut names = Vec::new();
    let dir_iter = fs::read_dir(dir).map_err(|e| e.to_string())?;

    for entry in dir_iter {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.is_file() {
            if let Some(file_name) = path.file_name() {
                let file_name = file_name.to_string_lossy();
                if file_name.ends_with(suffix) {
                    names.push(file_name.into_owned());
                }
            }
        }
    }
    Ok(names)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn subdir_names_3_subdirs() {
        let temp_dir = tempfile::tempdir().unwrap();
        let temp_dir_path = temp_dir.path();

        let subdir1 = temp_dir_path.join("subdir1");
        let subdir2 = temp_dir_path.join("subdir2");
        let subdir3 = temp_dir_path.join("subdir3");
        let file1 = temp_dir_path.join("file1");
        let file2 = temp_dir_path.join("file2");

        fs::create_dir(&subdir1).unwrap();
        fs::create_dir(&subdir2).unwrap();
        fs::create_dir(&subdir3).unwrap();
        File::create(&file1).unwrap();
        File::create(&file2).unwrap();

        let names = subdir_names(temp_dir_path).unwrap();
        assert_eq!(names.len(), 3);
        assert!(names.contains(&"subdir1".to_string()));
        assert!(names.contains(&"subdir2".to_string()));
        assert!(names.contains(&"subdir3".to_string()));
    }
}
