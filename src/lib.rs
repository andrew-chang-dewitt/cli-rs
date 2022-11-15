use std::{collections::HashMap, io};

pub struct Cli {
    args: HashMap<String, String>,
}

impl Cli {
    pub fn new() -> Self {
        Cli {
            args: HashMap::new(),
        }
    }

    /// Execute the given arguments on the configured CLI program
    ///
    /// # Example
    ///
    /// ```
    /// # use cli::Cli;
    ///
    /// let args = ["command", "--arg1", "value1"]
    ///     .iter()
    ///     .map(|&s| String::from(s));
    ///
    /// let mut cli = Cli::new();
    /// cli.run(args).unwrap();
    /// ```
    pub fn run(&mut self, mut args: impl Iterator<Item = String>) -> io::Result<()> {
        // ignore the first arg for now
        args.next();
        self.parse_args(args)?;
        // self.execute()
        Ok(())
    }

    fn parse_args(&mut self, mut args: impl Iterator<Item = String>) -> io::Result<()> {
        let mut res = Ok(());
        // loop over args
        for arg in args {
            // check if argument exists
            if let Some(exists) = self.get_arg(&arg) {
                println!("arg exists: {exists}");
            } else {
                res = Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("Argument {arg} does not exist!"),
                ));
                break;
            }
        }

        res
    }

    fn get_arg(&self, arg_name: &String) -> Option<&String> {
        self.args.get(arg_name)
    }
}

impl Default for Cli {
    fn default() -> Self {
        Self::new()
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
// }
