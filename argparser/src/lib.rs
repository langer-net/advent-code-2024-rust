#[derive(Debug)]
struct Arg {
    name: String,
    description: String,
}

impl Arg {
    fn new(name: &str, description: &str) -> Self {
        Arg {
            name: String::from(name),
            description: String::from(description),
        }
    }
}

pub struct ArgParser {
    description: String,
    args: Vec<Arg>,
}

impl ArgParser {
    pub fn new(description: &str) -> Self {
        ArgParser {
            description: String::from(description),
            args: Vec::new(),
        }
    }

    pub fn print_help(&self) {
        println!("{}", self.description);
        for arg in &self.args {
            println!("{:?}", arg)
        }
    }

    pub fn add_arg(&mut self, name: &str, description: &str) {
        self.args.push(Arg::new(name, description));
    }

    pub fn parse_args(&self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
