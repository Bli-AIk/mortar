use crate::handler::file_handler::FileHandler;
use crate::token::lex_with_output;

mod handler;
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

    lex_with_output(&content);
}
