use std::io::{self, Write};
use std::fs::OpenOptions;

pub fn format_task(task: &str) -> String{
    format!("[ ] {}\n",task)
}

pub fn writing_tasks(task: String) -> io::Result<()> {
    let formatted_task = format_task(&task);
    let combined_args = formatted_task.as_bytes();

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("Tasks.txt")?;

    file.write_all(combined_args)?;
    println!("File updated successfully: Tasks.txt");

    Ok(())
}

//Tests

#[cfg(test)]
mod add_test{
    use super::*;
    
    #[test]
    fn test_format_task(){
        let input = "Buy milk";
        let expected = "[ ] Buy milk\n";
        assert_eq!(format_task(input), expected);
    }
}