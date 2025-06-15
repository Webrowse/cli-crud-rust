use std::{env, io};

use crate::list;
use crate::delete;
use crate::complete;
use crate::add;

pub fn matching() -> io::Result<()> {
    let arg: Vec<String> = env::args().skip(1).collect();
    
    //if arguments are there, check if first argument is "add"
    if arg.len() > 0 {
        
        if arg[0] == "add" {
            add::writing_tasks(arg[1..].join(" "))?;
            println!("{}",list::list_tasks().expect("error"));
        }
        if arg[0] == "list" {
            let content = list::list_tasks()?;
            for (index, value) in content.lines()
                .filter(|x| !x.trim().is_empty())
                .enumerate() {
                println!("{}. {}", index + 1, value.trim());
            }
            
        }
        if arg[0] == "complete" {
            complete::complete(arg[1].clone())?;
            println!("{}",list::list_tasks().expect("Complete ran into trouble"));
        }
        
        if arg[0] == "delete" {
            delete::delete(arg[1].clone())?;
            println!("{}",list::list_tasks().expect("delete ran into trouble"));
        }
    };
    
    Ok(())
}