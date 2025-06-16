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
    let file_content = match list::list_tasks() {
        Ok(content) => content,
        Err(e) => {
            eprintln!(
                "Error while fetching file_content in Delete fn, Error: {}",
                e
            );
            return Err(e);
        }
    };
    let lines: Vec<String> = file_content.lines().map(|x| x.to_string()).collect();
    let mut current_task: usize = 0;
    let mut new_list: Vec<String> = Vec::new();
    for line in lines.into_iter() {
        current_task += 1;

        if del_target(&del) == Ok(current_task) {
            println!("Deleted {:?}: {}", del_target(&del), line);
        } else {
            new_list.push(line);
        }
    }

    let new_content = new_list.join("\n");
    let new_content_n = format!("{}\n", new_content);
    let mut file = File::create("Tasks.txt")?;
    file.write_all(new_content_n.as_bytes())?;

    Ok(())
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_del_target(){
        let x = del_target("3");
        let y: usize = 3;
        assert_eq!(x.unwrap(),y);
    }
    #[test]
    fn test_del_target_fail(){
        let x: Result<usize, String> = del_target("alpha");
        assert!(x.is_err());
    }
}