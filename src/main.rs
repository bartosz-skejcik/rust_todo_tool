mod utils;

use std::error::Error;

use serde::Deserialize;
use utils::filter::{self, filter_todos};

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct Todo {
    user_id: i64,
    id: i64,
    title: String,
    completed: bool,
}

async fn fetch_todos() -> Result<Vec<Todo>, Box<dyn Error>> {
    let url = "https://jsonplaceholder.typicode.com/todos";

    let res = reqwest::get(url).await?;

    let body = res
        .json::<Vec<Todo>>()
        .await;

    match body {
        Ok(data) => Ok(data),
        Err(e) => Err(Box::new(e)),
    }
}

fn print_todos(todos: &Vec<Todo>) {
    let mut max_todo_length: i32 = 0;

    todos
        .iter()
        .for_each(|todo| {
            let todo_length = todo
                .title
                .len() as i32;
            if todo_length > max_todo_length {
                max_todo_length = todo_length;
            }
        });

    let header = format!(
        "{:5}\t| {:5}\t| {:width$}\t| {:5}",
        "User ID",
        "ID",
        "Title",
        "Completed",
        width = max_todo_length as usize
    );

    println!(" {}", header);
    println!("{}", "-".repeat(header.len() + 12));

    for todo in todos {
        println!(
            " {:^8}\t| {:^5}\t| {:width$}\t| {:5}",
            todo.user_id,
            todo.id,
            todo.title,
            todo.completed,
            width = max_todo_length as usize
        );
    }
}

fn prompt(label: &str, default_value: &str) -> String {
    println!("{}", label);
    let mut value = String::new();
    std::io::stdin()
        .read_line(&mut value)
        .expect("Failed to read input");

    match value.trim() {
        "" => default_value.to_string(),
        _ => value
            .trim()
            .to_string(),
    }
}

fn handle_input(input: &str, todos: &Vec<Todo>) {
    let mut filtered_todos: Vec<Todo> = todos.to_vec();
    match input {
        "1" => {
            let user_id = prompt("User ID", "0");
            let user_id = user_id
                .parse::<i64>()
                .expect("Invalid user ID");
            filtered_todos = filter_todos(todos, filter::FilteringMethod::UserId(user_id));
        }
        "2" => {
            let title = prompt("Todo title:", "");
            filtered_todos = filter_todos(todos, filter::FilteringMethod::Title(title));
        }
        "3" => {
            let completed = prompt("Complete status [true/false]:", "true");
            match completed.as_str() {
                "true" | "t" => {
                    filtered_todos = filter_todos(todos, filter::FilteringMethod::Completed(true))
                }
                _ => {
                    filtered_todos = filter_todos(todos, filter::FilteringMethod::Completed(false))
                }
            }
        }
        "4" => {
            println!("Exiting...");
            std::process::exit(0);
        }
        _ => {
            println!("Invalid input");
        }
    }

    print_todos(&filtered_todos);
}

#[tokio::main]
async fn main() {
    let todos = fetch_todos()
        .await
        .expect("Failed to fetch todos");
    print_todos(&todos);

    loop {
        println!("\nFilter todos by:");
        println!("1. User ID");
        println!("2. Title");
        println!("3. Completed");
        println!("4. Exit");
        println!();

        let filter_method = prompt("> ", "4");

        handle_input(&filter_method, &todos);
    }
}
