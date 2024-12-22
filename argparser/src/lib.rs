use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub enum ArgParserError {
    CalledForHelp,
    MissingArgument(String),
    MissingRequiredFlag(String),
}

impl fmt::Display for ArgParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArgParserError::CalledForHelp => {
                write!(f, "Used --help flag.")
            }
            ArgParserError::MissingArgument(flag) => {
                write!(f, "No argument supplied for flag: {}", flag)
            }
            ArgParserError::MissingRequiredFlag(flag) => {
                write!(
                    f,
                    "Missing required flag: {}\
                    \r\nUse --help to see all available and required flags",
                    flag
                )
            }
        }
    }
}

impl std::error::Error for ArgParserError {}

#[derive(Debug)]
struct Arg {
    flag_name: String,
    description: String,
    required: bool,
}

impl Arg {
    fn new(flag_name: &str, description: &str, required: bool) -> Self {
        Arg {
            flag_name: flag_name.to_string(),
            description: description.to_string(),
            required,
        }
    }
}

impl fmt::Display for Arg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "--{} (required: {}): {}",
            self.flag_name, self.required, self.description,
        )
    }
}

pub struct ArgParser {
    description: String,
    args: Vec<String>,
    added_args: Vec<Arg>,
    args_map: HashMap<String, usize>,
}

impl ArgParser {
    pub fn new(args: impl Iterator<Item = String>, description: &str) -> Self {
        let mut parser = ArgParser {
            description: String::from(description),
            args: args.collect(),
            added_args: Vec::new(),
            args_map: HashMap::new(),
        };
        parser.add_arg(
            "help",
            "Prints an overview with all available flags.",
            false,
        );
        parser
    }

    pub fn add_arg(&mut self, flag_name: &str, description: &str, required: bool) {
        self.added_args
            .push(Arg::new(flag_name, description, required));
    }

    pub fn parse_args(&mut self) -> Result<(), ArgParserError> {
        // Check for the help flag.
        let mut args = self.added_args.iter();
        if let Some(help_arg) = args.next() {
            let help_flag = format!("--{}", help_arg.flag_name);
            if let Some(_) = self.args.iter().position(|arg_rec| *arg_rec == help_flag) {
                self.print_help();
                return Err(ArgParserError::CalledForHelp);
            }
        }

        // Iterate over all arguments.
        for arg in args {
            let flag_name = format!("--{}", arg.flag_name);
            if let Some(idx) = self.args.iter().position(|arg_rec| *arg_rec == flag_name) {
                if idx + 1 < self.args.len() {
                    self.args_map.insert(arg.flag_name.clone(), idx + 1);
                } else {
                    return Err(ArgParserError::MissingArgument(flag_name));
                }
            } else {
                if arg.required {
                    return Err(ArgParserError::MissingRequiredFlag(flag_name));
                }
            }
        }

        Ok(())
    }

    pub fn get(&self, arg: &str) -> Option<&str> {
        let idx = self.args_map.get(arg)?;
        Some(self.args[*idx].as_str())
    }

    fn print_help(&self) {
        println!("{}", self.description);
        for arg in &self.added_args {
            println!("{}", arg)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn access_arg() {
        let test_env_args = vec![
            "App_name".to_string(),
            "--test".to_string(),
            "test_arg".to_string(),
        ]
        .into_iter();

        let mut arg_parser = ArgParser::new(test_env_args, "Test parser.");
        arg_parser.add_arg("test", "This is a test flag", true);
        arg_parser.parse_args().unwrap();

        match arg_parser.get("test") {
            Some(arg) => assert_eq!("test_arg", arg),
            None => panic!("Expected the return of valid argument."),
        }
    }

    #[test]
    fn missing_required_flag() {
        let test_env_args = vec![
            "App_name".to_string(),
            "--test".to_string(),
            "test_arg".to_string(),
        ]
        .into_iter();

        let mut arg_parser = ArgParser::new(test_env_args, "Test parser.");
        arg_parser.add_arg("required", "This is a required flag", true);

        match arg_parser.parse_args() {
            Err(ArgParserError::MissingRequiredFlag(flag)) => {
                assert_eq!(flag, "--required".to_string());
            }
            _ => panic!("Expected MissingRequiredFlag error!"),
        }
    }

    #[test]
    fn missing_not_required_flag() {
        let test_env_args = vec![
            "App_name".to_string(),
            "--test".to_string(),
            "test_arg".to_string(),
        ]
        .into_iter();

        let mut arg_parser = ArgParser::new(test_env_args, "Test parser.");
        arg_parser.add_arg("optional", "This is an optional flag", false);

        match arg_parser.parse_args() {
            Err(ArgParserError::MissingRequiredFlag(_)) => {
                panic!("Not expected an error!")
            }
            _ => {
                let arg = arg_parser.get("optional");
                assert_eq!(None, arg);
            }
        }
    }

    #[test]
    fn called_for_help() {
        let test_env_args = vec!["App_name".to_string(), "--help".to_string()].into_iter();

        let mut arg_parser = ArgParser::new(test_env_args, "Test parser.");

        match arg_parser.parse_args() {
            Err(ArgParserError::CalledForHelp) => {}
            _ => panic!("Expected CalledForHelp error!"),
        }
    }
}
