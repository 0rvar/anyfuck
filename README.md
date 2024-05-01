# Anyfuck - Brainfuck-Like Languages Interpreter

This command-line tool interprets scripts written in Brainfuck and a few of its variants (Ook and Blub). It allows for execution directly from a file or via standard input (stdin).

## Features

- **Flexible Input:** Execute scripts from a file or directly via stdin.
- **Memory Customization:** Configure the memory size of the interpreter's runtime environment.
- **Language Inference:** Automatically infer the script language from the file extension or specify it manually.

## Installation

```bash
cargo install anyfuck
```

## Usage

```bash
anyfuck program.bf
anyfuck program.ook
anyfuck program.blub
anyfuck --memory 10000000 --language ook < some_program.txt
```

### Options

- `--memory <SIZE>`: Overrides the default brainfuck engine memory size (default is 1,000,000 cells).
- `-l, --language <LANGUAGE>`: Explicitly specify the language (Brainfuck, Ook, Blub).
- `--path <FILE>`: Path to the input file. If not provided, the interpreter will read from stdin.

## Supported Languages

- **Brainfuck**: Extensions `.bf` or `.b`
- **Ook**: Extension `.ook`
- **Blub**: Extension `.blub`

## Contributing

Contributions to improve the interpreter or add support for more variants of brainfuck-like languages are welcome. Please submit pull requests or raise issues as needed.
