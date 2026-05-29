use core::fmt;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;

#[derive(Serialize, Deserialize)]
struct Todo {
    status: Status,
    title: String,
}

#[derive(Serialize, Deserialize)]
enum Status {
    Pending,
    Completed,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let text = match self {
            Status::Pending => "pending",
            Status::Completed => "completed",
        };
        write!(f, "{text}")
    }
}

fn list_todos(todos: &[Todo]) {
    for (i, todo) in todos.iter().enumerate() {
        println!("{}: [{}]  {}", i, todo.status, todo.title);
    }
}

fn delete_todo(todos: &mut Vec<Todo>) {
    println!("Enter todo's number to delete");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let number: usize = match input.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Not a valid number.");
            return;
        }
    };

    println!("Are you sure to delete todo number {number}? (Y/n)");
    let mut confirm = String::new();
    io::stdin()
        .read_line(&mut confirm)
        .expect("failed to read line");
    let confirm = confirm.trim().to_lowercase();
    if confirm == "n" || confirm == "no" {
        println!("Cancelled.");
        return;
    }

    let before = todos.len();
    todos.remove(number);
    if todos.len() == before {
        println!("No todo with number {number}.");
    } else {
        println!("Deleted.");
    }
}

fn create_todo(todos: &mut Vec<Todo>) {
    println!("Enter todo name: ");
    let mut title = String::new();
    io::stdin()
        .read_line(&mut title)
        .expect("Failed to read line");
    if title.is_empty() {
        println!("Cancelled");
        return;
    }
    todos.push(Todo {
        title,
        status: Status::Pending,
    });
}

fn check_todo(todos: &mut [Todo]) {
    println!("Enter todo's number to check");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let number: usize = match input.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Not a valid number.");
            return;
        }
    };
    if number >= todos.len() {
        println!("Error index too big");
        return;
    }
    todos[number].status = Status::Completed;
}

fn main() -> Result<()> {
    let mut todos = import_todos();
    // `()` can be used when no completer is required
    let mut rl = DefaultEditor::new()?;
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;
                println!("Line: {}", line);
                match line.as_str() {
                    "add" | "create" => create_todo(&mut todos),
                    "ls" => list_todos(&todos),
                    "rm" | "delete" => delete_todo(&mut todos),
                    "check" => check_todo(&mut todos),
                    "quit" => break,
                    other => println!("unknown command: {other}"),
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    export_todos(todos);
    rl.save_history("history.txt")
        .expect("Failed to save history");
    Ok(())
}

fn import_todos() -> Vec<Todo> {
    let file_path = "data/todos.json";
    let content = fs::read_to_string(file_path).expect("Couldn't find or load todos");
    let todos: Vec<Todo> = serde_json::from_str(&content).expect("couldn't parse the todos");
    todos
}

fn export_todos(todos: Vec<Todo>) {
    let file_path = "data/todos.json";
    let todos: String = serde_json::to_string(&todos).expect("couldn't serialize todos");
    fs::write(file_path, todos).expect("Couldn't write file");
}
