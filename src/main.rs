use core::fmt;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};
use serde::{Deserialize, Serialize};
use std::fs;

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

enum Action {
    Continue,
    Break,
}

const FILE_PATH: &str = "data/todos.json";

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

fn delete_todo(todos: &mut Vec<Todo>, args: &Vec<&str>) {
    let Some(name) = args.first() else {
        println!("No arguments, need one");
        return;
    };
    if args.len() > 1 {
        println!("Too much arguments, need only one");
        return;
    }
    let Some(i) = todos.iter().position(|todo| todo.title == *name) else {
        println!("Todo name incorrect");
        return;
    };
    todos.remove(i);
}

fn create_todo(todos: &mut Vec<Todo>, args: &Vec<&str>) {
    let Some(name) = args.first() else {
        println!("No arguments, need one");
        return;
    };
    if args.len() > 1 {
        println!("Too much arguments, need only one");
        return;
    }
    todos.push(Todo {
        title: String::from(*name),
        status: Status::Pending,
    });
}

fn check_todo(todos: &mut [Todo], args: &Vec<&str>) {
    let Some(id) = args.first() else {
        println!("No arguments, need one");
        return;
    };
    let Some(i) = todos.iter().position(|t| t.title == *id) else {
        println!("No todo named: {}", *id);
        return;
    };
    todos[i].status = Status::Completed
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
                if let Action::Break = handle_input(&line, &mut todos) {
                    break;
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

fn handle_input(input: &str, todos: &mut Vec<Todo>) -> Action {
    let mut input = input.split_whitespace();
    let command = input.next();
    let args: Vec<&str> = input.collect();
    match command {
        Some("add") | Some("create") => create_todo(todos, &args),
        Some("ls") => list_todos(&todos),
        Some("rm") | Some("delete") => delete_todo(todos, &args),
        Some("check") => check_todo(todos, &args),
        Some("quit") => return Action::Break,
        Some(other) => println!("unknown command: {}", other),
        None => {}
    }
    Action::Continue
}

// fn verify_args(args: &Vec<&str>, todos: &Vec<Todo>) {
//     let args_nb = args.len();
//     if args_nb != 1 {
//         println!("Wrong number of arguments!");
//         Action::Break
//     } else {
//         let correct = todos.iter().any(|todo| todo.title == args[0]);
//         if !correct {
//             println!("Bad argument, todo named {} doesn't exist", args[0]);
//             Action::Break
//         } else {
//             Action::Continue
//         }
//     }
// }

fn import_todos() -> Vec<Todo> {
    let content = fs::read_to_string(FILE_PATH).expect("Couldn't find or load todos");
    let todos: Vec<Todo> = serde_json::from_str(&content).expect("couldn't parse the todos");
    todos
}

fn export_todos(todos: Vec<Todo>) {
    let todos: String = serde_json::to_string(&todos).expect("couldn't serialize todos");
    fs::write(FILE_PATH, todos).expect("Couldn't write file");
}
