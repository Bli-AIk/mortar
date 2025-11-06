use clap::{Arg, Command};
use mortar_compiler::{FileHandler, ParseHandler, Serializer};

fn main() {
    let matches = Command::new("mortar")
        .version("0.1.0")
        .author("Bli-AIk <haikun2333@gmail.com>")
        .about("Mortar language compiler")
        .arg(
            Arg::new("input")
                .help("Input .mortar file")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("Output file path")
        )
        .get_matches();

    let input_path = matches.get_one::<String>("input").unwrap();
    
    // Read source file
    let content = match FileHandler::read_source_file(input_path) {
        Ok(content) => content,
        Err(_) => return,
    };

    println!("{}", content);

    let program = match ParseHandler::parse_source_code(&content) {
        Ok(program) => program,
        Err(err) => {
            eprintln!("Parse error: {}", err);
            return;
        }
    };

    println!("Parsed successfully!");

    // Generate .mortared file
    let output_path = matches.get_one::<String>("output")
        .map(|s| s.as_str())
        .unwrap_or(input_path);
        
    match Serializer::save_to_file(&program, output_path) {
        Ok(()) => println!("Successfully generated .mortared file"),
        Err(err) => eprintln!("Failed to generate .mortared file: {}", err),
    }
}