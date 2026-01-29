use serde::{
    Deserialize, Serialize,
    de::{self, value::Error},
};

use std::{
    fs::{self, File, create_dir, exists},
    io::{self, BufReader, Write},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    pub(crate) is_completed: bool,
    pub(crate) title: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Todos {
    pub(crate) todos: Vec<Todo>,
}

pub fn read_file_to_json<T: de::DeserializeOwned>(file: File) -> T {
    let reader = BufReader::new(file);

    let json: T = serde_json::from_reader(reader).expect("Failed to read json");

    json
}

pub fn write_string_to_file(file_path: &String, content: String) -> bool {
    let content = fs::write(file_path, content).unwrap();

    return true;
}

pub fn read_today_file(file_path: &String) -> Result<Todos, serde_json::Error> {
    // Open the file in read-only mode with buffer.
    let file = File::open(file_path);

    match file {
        Ok(result) => {
            let today: Todos = read_file_to_json(result);

            return Ok(today);
        }
        Err(_err) => {
            let default_todo = Todos { todos: vec![] };
            let file_content =
                serde_json::to_string(&default_todo).unwrap_or(String::from("{\"todos\":[]}"));

            write_string_to_file(file_path, file_content);

            return Ok(default_todo);
        }
    };
}

pub fn add_new_todo(file_path: &String, new_todo: Todo) -> Result<bool, io::Error> {
    let file = File::open(file_path).unwrap();

    let current_content: Todos = read_file_to_json(file);
    let current_todos = current_content.todos;
    let mut new_todos = vec![];

    for todo in current_todos {
        new_todos.push(todo);
    }

    new_todos.push(new_todo);

    let new_file_content = Todos { todos: new_todos };

    let new_file = File::create_new(file_path);

    match new_file {
        Ok(mut file) => {
            let file_content =
                serde_json::to_string(&new_file_content).unwrap_or(String::from("{\"todos\":[]}"));

            file.write_all(file_content.as_bytes()).unwrap();
        }
        Err(_err) => {
            let mut file = File::options().write(true).open(file_path).unwrap();
            let file_content =
                serde_json::to_string(&new_file_content).unwrap_or(String::from("{\"todos\":[]}"));

            file.write_all(file_content.as_bytes()).unwrap();
        }
    }

    Ok(true)
}

pub fn remove_todo_by_number(file_path: &String, todo_number: &String) -> Result<bool, io::Error> {
    let file = File::open(file_path).unwrap();

    let current_content: Todos = read_file_to_json(file);
    let current_todos = current_content.todos;
    let mut new_todos = vec![];

    let mut count = 1;
    for todo in current_todos {
        println!("Count: {}", count.to_string());
        println!("todo_number: {}", todo_number);
        println!("check: {}", &count.to_string() != todo_number);

        if &count.to_string() != todo_number {
            new_todos.push(todo);
        }
        assert_eq!(&count.to_string(), todo_number);
        count += 1;
    }

    let new_file_content = Todos { todos: new_todos };

    println!("{:?}", new_file_content);

    let new_file = File::create_new(file_path);

    match new_file {
        Ok(mut file) => {
            let file_content =
                serde_json::to_string(&new_file_content).unwrap_or(String::from("{\"todos\":[]}"));

            file.write_all(file_content.as_bytes()).unwrap();
        }
        Err(_err) => {
            let file_content =
                serde_json::to_string(&new_file_content).unwrap_or(String::from("{\"todos\":[]}"));

            fs::write(file_path, file_content).unwrap();
        }
    }

    Ok(true)
}

pub fn create_folder_for_app_data(file_path: &String) {
    let directory_already_exists = exists(file_path);

    match directory_already_exists {
        Ok(ok) => {
            if ok == true {
                println!("SKIPPING CREATING DIRECTORY: Directory already exists")
            } else {
                println!("CREATING DIRECTORY: Creating directory to store each day json files");
                let result = create_dir(file_path);

                match result {
                    Ok(_result) => println!("CREATED DIRECTORY: Success"),
                    Err(_) => panic!("Failed to create directory"),
                }
            }
        }
        Err(_) => panic!("Failed to read if directory exists or not"),
    }
}
