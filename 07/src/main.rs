use std::fs;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Node {
    name: String,
    size: Option<usize>,
    children: Vec<Node>,
}

impl Node {
    fn file(name: impl Into<String>, size: usize) -> Self {
        Node {
            name: name.into(),
            size: Some(size),
            children: vec![],
        }
    }

    fn directory(name: impl Into<String>) -> Self {
        Node {
            name: name.into(),
            size: None,
            children: vec![],
        }
    }
}

impl TryFrom<&str> for Node {
    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let split_line = line.split_whitespace().collect::<Vec<&str>>();
        if split_line.len() < 2 {
            return Err("Invalid input - no whitespace detected");
        }

        let name = split_line[1];

        if split_line[0] == "dir" {
            return Ok(Node::directory(name));
        }

        if let Ok(file_size) = split_line[0].parse::<usize>() {
            return Ok(Node::file(name, file_size));
        }

        Err("Invalid input - unknown node type")
    }

    type Error = &'static str;
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Command {
    ChangeDirectory(String),
    ListDirectory,
}

fn parse_command(line: &str) -> Result<Command, &'static str> {
    if line.starts_with("$ dir ") {
        return Ok(Command::ChangeDirectory(
            line.split_whitespace().last().unwrap().to_string(),
        ));
    }

    if line.starts_with("$ ls") {
        return Ok(Command::ListDirectory);
    }

    Err("Unknown command")
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum InputLine {
    Node(Node),
    Command(Command),
}

fn parse_input_line(line: &str) -> InputLine {
    if line.starts_with("$ ") {
        return InputLine::Command(parse_command(line).unwrap());
    }

    InputLine::Node(Node::try_from(line).unwrap())
}

fn parse_input(input: &String) -> Node {
    let mut root = Node::directory("/");
    let mut current_path = Vec::<&mut Node>::new();
    current_path.push(&mut root);

    for line in input.lines() {
        match parse_input_line(line) {
            InputLine::Node(node) => {
                let cwd = current_path.last_mut().unwrap();
                cwd.children.push(node);
            }
            InputLine::Command(command) => match command {
                Command::ChangeDirectory(new_cwd_name) => {
                    
                }
                Command::ListDirectory => {}
            },
        }
    }

    root
}

fn main() {
    let input = fs::read_to_string("./input").unwrap();
    let root_node = parse_input(&input);
}

#[cfg(test)]
mod tests {
    use crate::{parse_command, Command, Node};

    #[test]
    fn test_node_parsing() {
        let inputs = [
            "dir qwe",
            "dir asdASD",
            "ddddir qweqwe",
            "qweqweasdad",
            "1234 test",
            "2356 example",
            "123 123",
        ];

        let expected = [
            Ok(Node::directory("qwe")),
            Ok(Node::directory("asdASD")),
            Err("Invalid input - unknown node type"),
            Err("Invalid input - no whitespace detected"),
            Ok(Node::file("test", 1234)),
            Ok(Node::file("example", 2356)),
            Ok(Node::file("123", 123)),
        ];

        inputs
            .iter()
            .map(|&input| Node::try_from(input))
            .zip(expected.iter())
            .for_each(|(result, expected)| {
                assert_eq!(result, *expected);
            });
    }

    #[test]
    fn test_command_parsing() {
        let inputs = [
            "not a command",
            "$ also not a command",
            "$ dir test_dir",
            "$ dir anotherTestDir",
            "$ ls",
            "$ le",
        ];

        let expected = [
            Err("Unknown command"),
            Err("Unknown command"),
            Ok(Command::ChangeDirectory("test_dir".to_string())),
            Ok(Command::ChangeDirectory("anotherTestDir".to_string())),
            Ok(Command::ListDirectory),
            Err("Unknown command"),
        ];

        inputs
            .iter()
            .map(|&input| parse_command(input))
            .zip(expected.iter())
            .for_each(|(result, expected)| {
                assert_eq!(result, *expected);
            });
    }
}
