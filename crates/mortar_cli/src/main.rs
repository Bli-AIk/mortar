use anyhow::{Context, Result, bail};
use clap::{Arg, Command};
use mortar_compiler::{FileHandler, Language, ParseHandler, Serializer};
use std::process;

mod i18n;
use i18n::{Language as CliLanguage, get_text};

fn cli_language_to_compiler_language(lang: CliLanguage) -> Language {
    match lang {
        CliLanguage::English => Language::English,
        CliLanguage::Chinese => Language::Chinese,
    }
}

fn build_command(language: CliLanguage) -> Command {
    Command::new("mortar")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Bli-AIk <haikun2333@gmail.com>")
        .about(get_text("app_about", language))
        .arg(
            Arg::new("input")
                .help(get_text("input_help", language))
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help(get_text("output_help", language)),
        )
        .arg(
            Arg::new("pretty")
                .short('p')
                .long("pretty")
                .action(clap::ArgAction::SetTrue)
                .help(get_text("pretty_help", language)),
        )
        .arg(
            Arg::new("verbose-lexer")
                .short('v')
                .long("verbose-lexer")
                .action(clap::ArgAction::SetTrue)
                .help(get_text("verbose_lexer_help", language)),
        )
        .arg(
            Arg::new("show-source")
                .short('s')
                .long("show-source")
                .action(clap::ArgAction::SetTrue)
                .help(get_text("show_source_help", language)),
        )
        .arg(
            Arg::new("check-only")
                .short('c')
                .long("check")
                .action(clap::ArgAction::SetTrue)
                .help(get_text("check_only_help", language)),
        )
        .arg(
            Arg::new("lang")
                .short('L')
                .long("lang")
                .value_name("LANGUAGE")
                .help(get_text("language_help", language)),
        )
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {:#}", e);
        process::exit(1);
    }
}

fn run() -> Result<()> {
    // First parse language setting with a simple parser
    let args: Vec<String> = std::env::args().collect();
    let language = if let Some(pos) = args.iter().position(|arg| arg == "--lang" || arg == "-L") {
        if let Some(lang_str) = args.get(pos + 1) {
            CliLanguage::from_str(lang_str).unwrap_or(CliLanguage::from_env())
        } else {
            CliLanguage::from_env()
        }
    } else {
        CliLanguage::from_env()
    };

    let matches = build_command(language).get_matches();

    let input_path = matches.get_one::<String>("input").unwrap();
    let pretty = matches.get_flag("pretty");
    let verbose_lexer = matches.get_flag("verbose-lexer");
    let show_source = matches.get_flag("show-source");
    let check_only = matches.get_flag("check-only");

    // Read source file
    let content = FileHandler::read_source_file(input_path)
        .with_context(|| get_text("error_reading_file", language))?;

    if show_source {
        println!("{}", get_text("original_source", language));
        println!("{}", content);
        println!("{}", get_text("end_source", language));
        println!();
    }

    // Parse with diagnostics
    let compiler_language = cli_language_to_compiler_language(language);
    let (parse_result, diagnostics) = ParseHandler::parse_source_code_with_diagnostics_and_language(
        &content,
        input_path.clone(),
        verbose_lexer,
        compiler_language,
    );

    // Print diagnostics
    diagnostics.print_diagnostics(&content);

    // Check for errors (including parse errors)
    if diagnostics.has_errors() {
        bail!("\n{}", get_text("compilation_failed", language));
    }

    let program = parse_result.map_err(|_| anyhow::anyhow!("Parse failed (internal)"))?;

    println!("{}", get_text("parsed_successfully", language));

    // Only generate output if not in check-only mode
    if !check_only {
        // Generate .mortared file
        let json_content = Serializer::serialize_to_json(&program, pretty)
            .map_err(|e| anyhow::anyhow!(e))
            .with_context(|| get_text("failed_to_generate", language))?;

        let output_path = if let Some(out) = matches.get_one::<String>("output") {
            std::path::PathBuf::from(out)
        } else {
            std::path::Path::new(input_path).with_extension("mortared")
        };

        if std::path::Path::new(input_path) == output_path {
            bail!("{}", get_text("output_same_as_input", language));
        }

        std::fs::write(&output_path, json_content).with_context(|| {
            format!(
                "{} {}",
                get_text("failed_to_generate", language),
                output_path.display()
            )
        })?;

        println!(
            "{} {}",
            get_text("generated", language),
            output_path.display()
        );
    }

    Ok(())
}

#[cfg(test)]
mod main_tests;
