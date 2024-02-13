use std::env::args;
use std::error::Error;
use std::path::Path;
use std::string::*;

use lazy_static::lazy_static;

use crate::messages;

pub struct ProgramFlags {
    pub is_verbose: bool,
    pub is_interactive: bool
}

impl ProgramFlags {
    pub fn new() -> ProgramFlags {
        ProgramFlags {
            is_verbose: false,
            is_interactive: true
        }
    }
}

lazy_static!{
    pub static ref PROGRAM_SETTINGS: ProgramFlags = {

        let mut settings = ProgramFlags::new();

        let args: Vec<String> = args().collect();
        let mut unexplained_args: Vec<&str> = vec![];

        for arg in &args[1..] {
            match arg.as_str() {
                "-v" => settings.is_verbose = true,
                "-ni" => settings.is_interactive = false,
                _ => unexplained_args.push(arg.as_str())
            };
        };

        let unknown_arg_msg = messages::getmsg("unknown_argument");
        for unexplained in unexplained_args {
            let pathchecker = Path::new(unexplained);
            if !pathchecker.exists() {
                unknown_arg_msg.print_args(&[unexplained]);
            }
        }
        print!("\n");

        settings

    };
}

pub fn error(which: &str, err: Option<&dyn Error>) -> ! {
    messages::getmsg(which).print();
    if PROGRAM_SETTINGS.is_verbose && err.is_some() {
        println!(
            "{}",
            err.unwrap()
        );
    }
    panic!();
}