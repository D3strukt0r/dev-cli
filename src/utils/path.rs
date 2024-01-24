// https://codereview.stackexchange.com/a/236771
pub fn find_recursively(starting_directory: &std::path::Path, file_name: &str) -> Option<std::path::PathBuf> {
    let mut path: std::path::PathBuf = starting_directory.into();
    let file = std::path::Path::new(file_name);

    loop {
        path.push(file);

        if path.is_file() {
            break Some(path);
        }

        if !(path.pop() && path.pop()) { // remove file && remove parent
            break None;
        }
    }
}
