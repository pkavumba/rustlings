// `if let` is convenient when you care about one enum variant and want to ignore
// the rest. Complete the function without writing a full `match` expression.

#[derive(Debug, PartialEq)]
enum Command {
    Print(String),
    Wait,
    Stop,
}

fn collect_printed(commands: Vec<Command>) -> Vec<String> {
    let mut printed = Vec::new();

    for command in commands {
        // TODO: If the command is `Command::Print`, move its text into
        // `printed`. Ignore all other commands.
    }

    printed
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_one_variant_with_if_let() {
        let printed = collect_printed(vec![
            Command::Wait,
            Command::Print(String::from("hello")),
            Command::Stop,
            Command::Print(String::from("enum")),
        ]);

        assert_eq!(printed, ["hello", "enum"]);
    }
}
