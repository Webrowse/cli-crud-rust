use std::{env, io};

use crate::add;
use crate::complete;
use crate::delete;
use crate::list;

pub fn matching() -> io::Result<()> {
    let arg: Vec<String> = env::args().skip(1).collect();

    if arg.is_empty() {
        eprintln!("Please provide a command: add, list, complete, or delete");
        return Ok(());
    }

    match arg[0].as_str() {
        "add" => {
            if arg.len() > 1 {
                add::writing_tasks(arg[1..].join(" "))?;
                println!("{}", list::list_tasks().expect("error"));
            } else {
                eprintln!("Please provide the tasks to add")
            };
        }
        "list" => {
            let content = list::list_tasks()?;
            for (index, value) in content.lines().filter(|x| !x.trim().is_empty()).enumerate() {
                println!("{:<3} {}", index + 1, value.trim());
            }
        }
        "complete" => {
            if arg.len() > 1 {
                complete::complete(arg[1].clone())?;
                println!("{}", list::list_tasks().expect("Complete ran into trouble"));
            } else {
                eprintln!("Please enter the task number");
            }
        }
        "delete" => {
            if arg.len() > 1 {
                delete::delete(arg[1].clone())?;
                println!("{}", list::list_tasks().expect("delete ran into trouble"));
            } else {
                eprintln!(
                    r#"Please give proper command\n such as following:\n "bare_tasker delete 1" "#
                )
            }
        }
        _ => {
            eprintln!("Invalid command");
        }
    };

    Ok(())
}
