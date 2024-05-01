use std::{
    ffi::OsStr,
    io::Read,
    path::{Path, PathBuf},
};

use clap::Parser;
use libbfi::{
    builtin::{
        bf_runtime::BrainfuckRuntime,
        trivial_tokenizers::{Blub, Brainfuck, Ook},
    },
    runtime::Runner,
    token::Tokenizer,
};

/// Anyfuck
/// Interpreter for brainfuck-like languages
#[derive(Parser)]
struct Args {
    /// Override bf engine memory size.
    #[arg(long, default_value = "1000000")]
    memory: usize,

    /// Specify language. If a path argument is provided,
    /// language will be inferred from the file extension.
    /// If no language is specified and the language cannot
    /// be inferred from the filename, brainfuck will be assumed.
    #[arg(long, short)]
    language: Option<Language>,

    /// Path to input file. If not provided, will read from stdin.
    path: Option<PathBuf>,
}

#[derive(clap::ValueEnum, Clone, PartialEq)]
enum Language {
    Brainfuck,
    Ook,
    Blub,
}

fn main() {
    let args = Args::parse();

    let lang = args
        .language
        .or_else(|| {
            args.path
                .as_ref()
                .and_then(|path| infer_language_from_path(&path))
        })
        .unwrap_or(Language::Brainfuck);

    // Read input from file or stdin.
    let input = match args.path {
        Some(path) => std::fs::read_to_string(path).unwrap(),
        None => {
            let mut input = String::new();
            std::io::stdin().read_to_string(&mut input).unwrap();
            input
        }
    };
    let mut bf = BrainfuckRuntime::with_memory_size(args.memory);

    let tokenizer = match lang {
        Language::Brainfuck => Brainfuck::to_tokens,
        Language::Ook => Ook::to_tokens,
        Language::Blub => Blub::to_tokens,
    };
    let tokens = tokenizer(input).expect("Failed to parse program");

    bf.add_tokens(tokens)
        .run_full_stack(&mut std::io::stdin().lock(), &mut std::io::stdout());
}

fn infer_language_from_path(path: &Path) -> Option<Language> {
    match path.extension().and_then(OsStr::to_str) {
        Some("bf") | Some("b") => Some(Language::Brainfuck),
        Some("ook") => Some(Language::Ook),
        Some("blub") => Some(Language::Blub),
        _ => None,
    }
}
