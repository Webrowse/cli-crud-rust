


use std::{env, fs::{File, OpenOptions}, io::{self, Write}};
fn main() -> io::Result<()>{
    let arg:Vec<String> = env::args().skip(1).collect();
    if arg.len() > 0 && arg[0] == "add".to_string(){
        writing_tasks(arg[1..].join(" "))?;
    }
    Ok(())
}
fn writing_tasks(task: String) -> io::Result<()>{
    let mut byte_task = task.as_bytes().to_vec();
    byte_task.extend_from_slice(b"\n");
    let file_read = File::open("Tasks.txt");
    match file_read{
        Ok(_file) => {
            let mut file = OpenOptions::new()
                .append(true)
                .open("Tasks.txt")?;
            
            file.write_all( &byte_task)?;
            println!("File updated successfully: Tasks.txt");
        },
        Err(ref e) if e.kind() == io::ErrorKind::NotFound => {
            let mut file = File::create("Tasks.txt")?;
            file.write_all(task.as_bytes())?;
            println!("New File created successfully: Tasks.txt");
        },
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            return Err(e);
        }
    }
    Ok(())
}