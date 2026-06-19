// Build a small command-line interface with `clap`'s derive macros.

use clap::{Parser, Subcommand};

#[derive(Debug, Parser, PartialEq)]
#[command(name = "todo")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, PartialEq, Subcommand)]
enum Command {
    Add { title: String },
    List { completed: bool },
    Done { id: u32 },
}

fn parse_args(args: &[&str]) -> Cli {
    Cli::parse_from(args)
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_add_command() {
        assert_eq!(
            parse_args(&["todo", "add", "learn crates"]),
            Cli {
                command: Command::Add {
                    title: "learn crates".to_string(),
                },
            },
        );
    }

    #[test]
    fn parses_list_flag() {
        assert_eq!(
            parse_args(&["todo", "list", "--completed"]),
            Cli {
                command: Command::List { completed: true },
            },
        );
    }
}
