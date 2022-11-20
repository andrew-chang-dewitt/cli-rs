use std::{any::Any, collections::HashMap, io};

// Args can do the following
trait Arguable<T> {
    fn get_name(&self) -> &str;
    fn get_val(&self) -> Option<T>;
    fn set_val(&self, val: T);
}

#[derive(Debug)]
struct Arg<T> {
    name: String,
    value: Option<Box<T>>,
}

impl<T> Arg<T> {
    pub fn new(name: &str) -> Self {
        Arg {
            name: name.to_string(),
            value: None,
        }
    }
}

impl<T> Arguable<T> for Arg<T> {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_val(&self) -> Option<T> {
        &self.value
    }

    fn set_val(&self, val: T) {
        self.value = Some(Box::new(val));
    }
}

pub struct Cli {
    args: Vec<dyn Arguable<dyn Any>>,
    args_by_name: HashMap<&'static str, usize>,
}

/// Set up a command and execute it
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
impl Cli {
    pub fn new() -> Self {
        Cli {
            args: Vec::new(),
            args_by_name: HashMap::new(),
        }
    }

    pub fn arg<T>(self, new_arg: Arg<T>) -> Self {
        // add to args list
        self.args.push(Box::new(new_arg));
        // hash by name for lookup
        let idx = self.args.len() - 1;
        self.args_by_name.insert(&new_arg.get_name(), idx);

        self
    }

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
            if let Some(&exists) = self.get_arg(&arg) {
                dbg!(exists)
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

    fn get_arg(&self, arg_name: &str) -> Option<&dyn Arguable<dyn Any>> {
        let idx = self.args_by_name.get(arg_name)?;
        self.args.get(*idx)
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
