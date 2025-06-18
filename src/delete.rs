use crate::list;
use std::fs::File;
use std::io::{self, Write};

pub fn del_target(del: &str) -> Result<usize, String> {
    let del_target = match del.parse::<usize>() {
        Ok(a) => a,
        Err(e) => {
            return Err(format!("Invalid delete input: {}", e));
        }
    };
    Ok(del_target)
}

pub fn delete(del: String) -> io::Result<()> {
    let index = match del_target(del.as_str()) {
        Ok(index) => index,
        Err(e) => {
            eprintln!("{}", e);
            return Ok(());
        }
    };

    let file_content = match list::list_tasks() {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error fetching file_content: {}", e);
            return Err(e);
        }
    };
    let lines: Vec<String> = file_content.lines().map(|x| x.to_string()).collect();
    let (new_list, deleted_task) = delete_task_from_list(lines, index);

    match deleted_task {
        Some(task) => println!("Deleted {}: {}", index, task),
        None => println!("No task found at {} index", index),
    }

    let new_content = new_list.join("\n") + "\n";
    let mut file = File::create("Tasks.txt")?;
    file.write_all(new_content.as_bytes())?;

    Ok(())
}

pub fn delete_task_from_list(
    lines: Vec<String>,
    del_index: usize,
) -> (Vec<String>, Option<String>) {
    let mut current = 0;
    let mut deleted = None;
    let mut result = Vec::new();

    for line in lines {
        current += 1;
        if current == del_index {
            deleted = Some(line);
        } else {
            result.push(line);
        }
    }
    (result, deleted)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_del_target() {
        let x = del_target("3");
        let y: usize = 3;
        assert_eq!(x.unwrap(), y);
    }
    #[test]
    fn test_del_target_fail() {
        let x: Result<usize, String> = del_target("alpha");
        assert!(x.is_err());
    }
    #[test]
    fn test_delete_task_from_list() {
        let line: Vec<String> = vec!["first task", "second task", "3rd one"]
            .iter()
            .map(|x| x.to_string())
            .collect();
        let index: usize = 2;
        let (string, option) = delete_task_from_list(line, index);
        let expected_str: Vec<String> = vec!["first task", "3rd one"]
            .iter()
            .map(|x| x.to_string())
            .collect();
        let expected_opt: Option<String> = Some("second task".to_string());

        assert_eq!(string, expected_str);
        assert_eq!(option, expected_opt);
    }
}
