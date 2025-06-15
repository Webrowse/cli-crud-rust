use std::io::{self, Read};
use std::fs::File;

pub fn list_tasks() -> io::Result<String> {
    let mut file = File::open("Tasks.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let filtered_empty_lines = content.lines()
        .filter(|line| !line.trim().is_empty())
        .collect::<Vec<&str>>();
    Ok(filtered_empty_lines.join("\n"))
}