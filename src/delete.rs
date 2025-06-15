
use std::io::{self, Write};
use std::fs::File;
use crate::list;

pub fn delete(del:String) -> io::Result<()>{
    let del_target = match del.parse::<usize>(){
        Ok(a)  => a,
        Err(e) => {
            eprintln!("Invalid delete input: {}",e);
            return Ok(());
        }
    };
    let file_content = match list::list_tasks(){
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error while fetching file_content in Delete fn, Error: {}",e);
            return Err(e);
        }
    };
    let lines: Vec<String> = file_content.lines().map(|x|x.to_string()).collect();
    let mut current_task:usize = 0;
    let mut new_list: Vec<String> = Vec::new();
    for line in lines.into_iter(){
        current_task += 1;

        if del_target == current_task {
            println!("Deleted {}: {}",del_target, line);
        }else{
            new_list.push(line);
        }
    }

    let new_content = new_list.join("\n");
    let new_content_n = format!("{}\n",new_content);
    let mut file = File::create("Tasks.txt")?;
    file.write_all(new_content_n.as_bytes())?;

    Ok(())
}