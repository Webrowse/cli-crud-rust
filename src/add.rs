use std::io::{self, Write};
use std::fs::OpenOptions;

pub fn writing_tasks(task: String) -> io::Result<()> {
    let formatted_task = format!("[ ] {} \n", task);
    let combined_args = formatted_task.as_bytes();

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("Tasks.txt")?;

    file.write_all(&combined_args)?;
    println!("File updated successfully: Tasks.txt");

    Ok(())
}
