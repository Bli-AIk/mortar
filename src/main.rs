use crate::handler::file_handler::FileHandler;
use crate::parser::ParseHandler;

mod handler;
mod parser;
mod token;
mod tests;

fn main() {
    let path = "hello.mortar";

    // Read source file
    let content = match FileHandler::read_source_file(path) {
        Ok(content) => content,
        Err(_) => return,
    };

    println!("{}", content);

    let program = match ParseHandler::parse_source_code(&content) {
        Ok(program) => program,
        Err(err) => {
            eprintln!("Parse error: {}", err);
            return;
        },
    };

    println!("Parsed successfully: {:#?}", program);
}
