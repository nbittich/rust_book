use std::fs;
pub fn read_file(file_name: &str) -> std::io::Result<String> {
    fs::read_to_string(file_name)
}
