use std::io::{self, stdin};

pub fn read_user_input_from_terminal() -> Result<String, io::Error> {
    let mut buffer = String::new();
    let stdin = stdin();

    let result = stdin.read_line(&mut buffer);

    match result {
        Ok(_result) => Ok(buffer),
        Err(err) => panic!("{err}"),
    }
}
