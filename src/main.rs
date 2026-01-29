use std::{process, str::FromStr, vec};

use chrono::Local;

use crate::utils::{
    file::{
        Todo, Todos, add_new_todo, create_folder_for_app_data, read_today_file,
        remove_todo_by_number,
    },
    input::read_user_input_from_terminal,
};

mod utils;

#[derive(Debug)]
enum ExpectedInput {
    ViewToday = 1,
    AddNewTodo = 2,
    DeleteTodo = 3,
    Exit = 4,
}

impl FromStr for ExpectedInput {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        match s {
            "1" => Ok(ExpectedInput::ViewToday),
            "2" => Ok(ExpectedInput::AddNewTodo),
            "3" => Ok(ExpectedInput::DeleteTodo),
            _ => Ok(ExpectedInput::Exit),
        }
    }
}

fn main() {
    let todos_directory: String = String::from("todos");
    let current_day_string = Local::now().format("%Y-%m-%d").to_string();

    create_folder_for_app_data(&todos_directory);
    println!("------ Todo app ------");
    main_screen(&current_day_string);
}

fn main_screen(today_string: &String) {
    println!("\n\nWhat would you like to do? \n\n");
    println!("1. View today's tasks");
    println!("2. Add new todo");
    println!("3. Delete todo");
    println!("4. Exit");

    let input = read_user_input_from_terminal();

    match input {
        Ok(_input) => {
            let parsed_input: ExpectedInput = _input.parse().unwrap_or(ExpectedInput::Exit);

            match parsed_input {
                ExpectedInput::ViewToday => view_today(today_string, false),
                ExpectedInput::AddNewTodo => add_todo(today_string),
                ExpectedInput::DeleteTodo => remove_todo(today_string),
                ExpectedInput::Exit => exit(),
            }
        }
        Err(err) => panic!("FAILED: {err}"),
    }
}

fn exit() {
    process::exit(0)
}

fn add_todo(today_string: &String) {
    view_today(today_string, true);
    let today_file_path = format!("todos/{today_string}.json");

    println!("\nSpecify the title of the todo");
    let input = read_user_input_from_terminal();

    match input {
        Ok(input) => {
            let new_todo = Todo {
                is_completed: false,
                title: input.trim().to_string(),
            };

            add_new_todo(&today_file_path, new_todo).unwrap();
        }
        Err(err) => panic!("FAILED: {err}"),
    }

    main_screen(today_string);
}

fn remove_todo(today_string: &String) {
    view_today(today_string, true);
    let today_file_path = format!("todos/{today_string}.json");

    println!("\nSpecify the number of todo you want to delete (single)");
    let input = read_user_input_from_terminal();

    match input {
        Ok(input) => {
            remove_todo_by_number(&today_file_path, &input.trim().to_string()).unwrap();
        }
        Err(err) => panic!("FAILED: {err}"),
    }

    main_screen(today_string);
}

fn view_today(today_string: &String, skip_main_screen: bool) {
    println!("\n\nTodos for today ({})\n\n", today_string);

    let default_todos = Todos { todos: vec![] };

    let today_file_path = format!("todos/{today_string}.json");
    let result = read_today_file(&today_file_path).unwrap_or(default_todos);

    let mut counter = 1;
    for todo in result.todos {
        let mut display_x = String::from("");

        if todo.is_completed {
            display_x = String::from("X");
        }
        println!("{}. [{}] {}", counter, display_x, todo.title);
        counter += 1;
    }

    if !skip_main_screen {
        main_screen(today_string);
    }
}
