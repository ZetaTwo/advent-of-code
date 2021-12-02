use std::str::FromStr;
use std::{error::Error, fmt};

#[derive(Debug)]
pub enum CommandParseError {
    CommandError,
    ParseIntError(std::num::ParseIntError),
}

impl From<std::num::ParseIntError> for CommandParseError {
    fn from(e: std::num::ParseIntError) -> CommandParseError {
        CommandParseError::ParseIntError(e)
    }
}

impl Error for CommandParseError {}

impl From<CommandParseError> for utils::InputError {
    fn from(e: CommandParseError) -> utils::InputError {
        utils::InputError::ParseError(Box::new(e))
    }
}

impl fmt::Display for CommandParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to parse command")
    }
}

#[derive(Debug, PartialEq)]
pub enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl FromStr for Command {
    type Err = CommandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();

        let amount = parts[1].parse::<u32>()?;

        match parts[0] {
            "forward" => Ok(Command::Forward(amount)),
            "down" => Ok(Command::Down(amount)),
            "up" => Ok(Command::Up(amount)),
            _ => Err(CommandParseError::CommandError),
        }
    }
}

pub struct State {
    pub horizontal: u32,
    pub depth: u32,
}

impl State {
    fn apply_command(&self, command: &Command) -> State {
        match command {
            Command::Down(down) => State {
                horizontal: self.horizontal,
                depth: self.depth + down,
            },
            Command::Up(up) => State {
                horizontal: self.horizontal,
                depth: self.depth - up,
            },
            Command::Forward(forward) => State {
                horizontal: self.horizontal + forward,
                depth: self.depth,
            },
        }
    }
}

pub struct State2 {
    pub horizontal: i32,
    pub depth: i32,
    aim: i32,
}

impl State2 {
    fn apply_command(&self, command: &Command) -> State2 {
        match command {
            Command::Down(down) => State2 {
                horizontal: self.horizontal,
                depth: self.depth,
                aim: self.aim + *down as i32,
            },
            Command::Up(up) => State2 {
                horizontal: self.horizontal,
                depth: self.depth,
                aim: self.aim - *up as i32,
            },
            Command::Forward(forward) => State2 {
                horizontal: self.horizontal + *forward as i32,
                depth: self.depth + self.aim * *forward as i32,
                aim: self.aim,
            },
        }
    }
}

pub fn run_commands<I>(commands: &I) -> State
where
    for<'a> &'a I: IntoIterator<Item = &'a Command>,
{
    commands.into_iter().fold(
        State {
            horizontal: 0,
            depth: 0,
        },
        |state, command| state.apply_command(command),
    )
}

pub fn run_commands2<I>(commands: &I) -> State2
where
    for<'a> &'a I: IntoIterator<Item = &'a Command>,
{
    commands.into_iter().fold(
        State2 {
            horizontal: 0,
            depth: 0,
            aim: 0,
        },
        |state, command| state.apply_command(command),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::BufReader;

    #[test]
    fn test_parse_command() {
        assert_eq!("forward 5".parse::<Command>().unwrap(), Command::Forward(5));
        assert_eq!("down 5".parse::<Command>().unwrap(), Command::Down(5));
        assert_eq!("forward 8".parse::<Command>().unwrap(), Command::Forward(8));
        assert_eq!("up 3".parse::<Command>().unwrap(), Command::Up(3));
        assert_eq!("down 8".parse::<Command>().unwrap(), Command::Down(8));
        assert_eq!("forward 2".parse::<Command>().unwrap(), Command::Forward(2));
    }

    #[test]
    fn test_apply_commands() {
        let commands = vec![
            Command::Forward(5),
            Command::Down(5),
            Command::Forward(8),
            Command::Up(3),
            Command::Down(8),
            Command::Forward(2),
        ];
        let res = run_commands(&commands);
        assert_eq!(res.depth, 10);
        assert_eq!(res.horizontal, 15);
    }

    #[test]
    fn test_apply_commands2() {
        let commands = vec![
            Command::Forward(5),
            Command::Down(5),
            Command::Forward(8),
            Command::Up(3),
            Command::Down(8),
            Command::Forward(2),
        ];
        let res = run_commands2(&commands);
        assert_eq!(res.depth, 60);
        assert_eq!(res.horizontal, 15);
        assert_eq!(res.aim, 10);
    }

    #[test]
    fn test_day02a() {
        let file = File::open("../input/2.in").expect("Failed to open test data");
        let reader = BufReader::new(file);
        let input =
            utils::get_parsed_lines::<Command, _>(reader).expect("Failed to read test data");

        let res = run_commands(&input);

        assert_eq!(res.depth * res.horizontal, 1727835);
    }

    #[test]
    fn test_day02b() {
        let file = File::open("../input/2.in").expect("Failed to open test data");
        let reader = BufReader::new(file);
        let input =
            utils::get_parsed_lines::<Command, _>(reader).expect("Failed to read test data");

        let res = run_commands2(&input);

        assert_eq!(res.depth * res.horizontal, 1544000595);
    }
}
