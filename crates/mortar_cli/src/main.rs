use clap::{Arg, Command};
use mortar_compiler::{FileHandler, ParseHandler, Serializer};
use std::process;

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
                .help("Output file path"),
        )
        .arg(
            Arg::new("pretty")
                .short('p')
                .long("pretty")
                .action(clap::ArgAction::SetTrue)
                .help("Generate formatted JSON with indentation"),
        )
        .arg(
            Arg::new("verbose-lexer")
                .short('v')
                .long("verbose-lexer")
                .action(clap::ArgAction::SetTrue)
                .help("Show verbose lexer output"),
        )
        .arg(
            Arg::new("show-source")
                .short('s')
                .long("show-source")
                .action(clap::ArgAction::SetTrue)
                .help("Show original source text"),
        )
        .arg(
            Arg::new("check-only")
                .short('c')
                .long("check")
                .action(clap::ArgAction::SetTrue)
                .help("Only check for errors and warnings without generating output"),
        )
        .get_matches();

    let input_path = matches.get_one::<String>("input").unwrap();
    let pretty = matches.get_flag("pretty");
    let verbose_lexer = matches.get_flag("verbose-lexer");
    let show_source = matches.get_flag("show-source");
    let check_only = matches.get_flag("check-only");

    // Read source file
    let content = match FileHandler::read_source_file(input_path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            process::exit(1);
        }
    };

    if show_source {
        println!("--- Original Source ---");
        println!("{}", content);
        println!("--- End Source ---");
        println!();
    }

    // Parse with diagnostics
    let (parse_result, diagnostics) = ParseHandler::parse_source_code_with_diagnostics(
        &content, 
        input_path.clone(), 
        verbose_lexer
    );

    // Print diagnostics
    diagnostics.print_diagnostics(&content);

    // Check for errors (including parse errors)
    if diagnostics.has_errors() {
        eprintln!("\nCompilation failed due to errors.");
        process::exit(1);
    }

    let program = match parse_result {
        Ok(program) => program,
        Err(_) => {
            // Parse error was already reported through diagnostics
            process::exit(1);
        }
    };

    println!("Parsed successfully!");

    // Only generate output if not in check-only mode
    if !check_only {
        // Generate .mortared file
        let output_path = matches
            .get_one::<String>("output")
            .map(|s| s.as_str())
            .unwrap_or(input_path);

        match Serializer::save_to_file(&program, output_path, pretty) {
            Ok(()) => println!("Successfully generated .mortared file"),
            Err(err) => {
                eprintln!("Failed to generate .mortared file: {}", err);
                process::exit(1);
            }
        }
    }
}
