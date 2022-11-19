use std::fmt;

trait DoAThing {
    fn do_thing(self);
}

struct AString {
    name: String,
    value: String,
}

impl DoAThing for AString {
    fn do_thing(self) {
        println!("{} is {}", self.name, self.value);
    }
}

struct ANum {
    name: String,
    value: usize,
}

impl DoAThing for ANum {
    fn do_thing(self) {
        println!("{} is a number: {}", self.name, self.value);
    }
}

#[test]
fn test_the_thing() {
    let coll: Vec<Box<dyn DoAThing>> = vec![
        Box::new(AString {
            name: "first thing".to_string(),
            value: "a string".to_string(),
        }),
        Box::new(ANum {
            name: "second thing".to_string(),
            value: 1,
        }),
    ];

    for c in coll {
        c.do_thing();
    }
}
// use std::{collections::HashMap, io};
//
// #[derive(Debug)]
// struct Arg<T> {
//     name: String,
//     value: Option<T>,
// }
//
// impl<T> Arg<T> {
//     pub fn new(name: &str) -> Self {
//         Arg {
//             name: name.to_string(),
//             value: None,
//         }
//     }
// }
//
// trait ArgStuff {
//     fn get_name(self) -> String;
// }
//
// impl<T> ArgStuff for Arg<T> {
//     fn get_name(self) -> String {
//         self.name
//     }
// }
//
// pub struct Cli {
//     args: Vec<Box<dyn ArgStuff>>,
//     args_by_name: HashMap<&'static str, usize>,
// }
//
// /// Set up a command and execute it
// ///
// /// # Example
// ///
// /// ```
// /// # use cli::Cli;
// ///
// /// let args = ["command", "--arg1", "value1"]
// ///     .iter()
// ///     .map(|&s| String::from(s));
// ///
// /// let mut cli = Cli::new();
// /// cli.run(args).unwrap();
// /// ```
// impl Cli {
//     pub fn new() -> Self {
//         Cli {
//             args: Vec::new(),
//             args_by_name: HashMap::new(),
//         }
//     }
//
//     pub fn arg<T>(self, new_arg: Arg<T>) -> Self {
//         // add to args list
//         self.args.push(Box::new(new_arg));
//         // hash by name for lookup
//         let idx = self.args.len() - 1;
//         self.args_by_name.insert(&new_arg.get_name(), idx);
//
//         self
//     }
//
//     pub fn run(&mut self, mut args: impl Iterator<Item = String>) -> io::Result<()> {
//         // ignore the first arg for now
//         args.next();
//         self.parse_args(args)?;
//         // self.execute()
//         Ok(())
//     }
//
//     fn parse_args(&mut self, mut args: impl Iterator<Item = String>) -> io::Result<()> {
//         let mut res = Ok(());
//         // loop over args
//         for arg in args {
//             // check if argument exists
//             if let Some(&exists) = self.get_arg(&arg) {
//                 dbg!(exists)
//             } else {
//                 res = Err(io::Error::new(
//                     io::ErrorKind::NotFound,
//                     format!("Argument {arg} does not exist!"),
//                 ));
//                 break;
//             }
//         }
//
//         res
//     }
//
//     fn get_arg(&self, arg_name: &str) -> Option<&Box<dyn ArgStuff>> {
//         let idx = self.args_by_name.get(arg_name)?;
//         self.args.get(*idx)
//     }
// }
//
// impl Default for Cli {
//     fn default() -> Self {
//         Self::new()
//     }
// }
//
// // #[cfg(test)]
// // mod tests {
// //     use super::*;
// // }
