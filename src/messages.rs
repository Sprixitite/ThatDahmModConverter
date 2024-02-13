use core::fmt;
use std::{collections::HashMap, fmt::Display, io::{self, Write}};

use colored::Colorize;
use lazy_static::lazy_static;

use crate::app;

macro_rules! strings {
    ( $x:expr ) => ($x);
    ( $x:expr, $( $y:expr ),+ ) => {
        format!(
            "{} {} ",
            $x,
            strings!($($y),+)
        )
    };
}

macro_rules! compose {
    ( $x:ident, $y:ident, $z:ident ) => {
        fn $x<S: AsRef<str>>(to: S) -> String {
            return $y($z(to));
        }
    }
}

fn i<S: AsRef<str>>(to: S) -> String {
    return format!("{}", to.as_ref().italic());
}

fn b<S: AsRef<str>>(to: S) -> String {
    return format!("{}", to.as_ref().bold());
}

fn d<S: AsRef<str>>(to: S) -> String {
    return format!("{}", to.as_ref().dimmed());
}

compose!(di, d, i);
compose!(bi, b, i);

fn red<S: AsRef<str>>(to: S) -> String {
    return format!("{}", to.as_ref().red());
}

fn blu<S: AsRef<str>>(to: S) -> String {
    return format!("{}", to.as_ref().blue());
}

fn grn<S: AsRef<str>>(to: S) -> String {
    return format!("{}", to.as_ref().green());
}

fn ylw<S: AsRef<str>>(to: S) -> String {
    return format!("{}", to.as_ref().yellow());
}

#[derive(Debug, Clone)]
struct MessageNotFoundError {
    msg: String
}

impl fmt::Display for MessageNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Attempt to use nonexistent string \"{}\"", self.msg)
    }
}

impl std::error::Error for MessageNotFoundError {
}

pub fn getmsg(which: &str) -> &'static Message {
    let result = STATIC_STRINGS.get(which);
    match result {
        Some(v) => v,
        None => app::error("getmsg_fail", Some(&MessageNotFoundError{msg: which.to_owned()}))
    }
}

pub struct Message {
    msg: Vec<String>,
    msg_type: MessageType
}

enum MessageType {
    ERROR,
    WARN,
    INFO
}

impl Message {
    fn new(msg: String, msg_type: MessageType) -> Message {
        let split_msg: Vec<String> = msg.split("{}").map(|x| x.to_string()).collect();
        Message { msg: split_msg, msg_type: msg_type }
    }

    /// Equivalent to print_newlines(1)
    pub fn print(&self) {
        self.print_newlines(1);
    }

    pub fn print_newlines(&self, newlines: usize) {
        self.print_fmt(self.msg(), newlines);
    }

    fn print_fmt(&self, msg: String, newlines: usize) {
        let prefix;
        match self.msg_type {
            MessageType::ERROR => prefix = bi(red("\nError! ")),
            MessageType::WARN => prefix = bi(ylw("Warning! ")),
            MessageType::INFO => prefix = String::from("")
        }
        
        print!("{}{}{}", prefix, msg, "\n".repeat(newlines));
        if newlines > 0 {
            io::stdout().flush();
        }
    }

    pub fn msg(&self) -> String {
        return self.msg.concat();
    }

    /// Equivalent to print_args_newlines(1)
    pub fn print_args(&self, args: &[&str]) {
        self.print_args_newlines(args, 1);
    }

    pub fn print_args_newlines(&self, args: &[&str], newlines: usize) {
        self.print_fmt(format!("{}", self.fmt_args(0, args)), newlines);
    }

    fn fmt_args(&self, idx: usize, args: &[&str]) -> String {
        if idx >= self.msg.len() { return String::from(""); }
        let idx2 = idx + 1;
        let recurse = self.fmt_args(idx2, args);
        return format!("{}{}{}", self.msg[idx], args.get(idx).unwrap_or(&""), recurse)
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg())
    }
}

lazy_static!{
    static ref STATIC_STRINGS: HashMap<&'static str, Message> = {
        [
            (
                MessageType::ERROR,
                "create_context_fail",
                strings!(
                    di("\nCreation of Lua context failed!"),
                    di("\nPlease re-run with the -v flag and file a bug report if the issue persists.")
                )
            ),
            (
                MessageType::ERROR,
                "builtin_lua_fail",
                strings!(
                    di("\nExecution of internal lua code failed!"),
                    di("\nPlease re-run with the -v flag and file a bug report, the program will now exit.")
                )
            ),
            (
                MessageType::ERROR,
                "dialogue_confirm_fail",
                strings!(
                    di( "\nExecution of confirmation dialogue failed!" ),
                    di( "\nPlease re-run with the -v flag and file a report if the issue persists." )
                )
            ),
            (
                MessageType::ERROR,
                "getmsg_fail",
                strings!(
                    di( "Retrieval of string failed!" ),
                    di( "\nPlease re-run with the -v flag, and file a bug report." )
                )
            ),
            (
                MessageType::ERROR,
                "cwd_access_failure",
                strings!(
                    di( "\nFailed to access current working directory!" ),
                    di( "\nPlease re-run with the -v flag, and file a bug report." )
                )
            ),
            (
                MessageType::ERROR,
                "cwd_read_failure",
                strings!(
                    di(   "Failed to get an iterator over the current working directory!" ),
                    di( "\nPlease re-run with the -v flag, and file a bug report." )
                )
            ),
            (
                MessageType::ERROR,
                "exec_basemod_failure",
                strings!(
                    di( "Failed evaluating" ),
                    di( "{}" ),
                    di( "Please re-run with the -v flag, and file a bug report." )
                )
            ),
            (
                MessageType::ERROR,
                "serde_serialization_failure",
                strings!(
                    di( "Failed serializing table!" ),
                    di( "Please re-run with the -v flag, and file a bug report." )
                )
            ),
            (
                MessageType::WARN,
                "unknown_argument",
                strings!(
                    di( "Unknown argument" ),
                     i( "\"{}\"" )
                )
            ),
            (
                MessageType::WARN,
                "fail_read_direntry",
                strings!(
                    di( "Failed to read directory entry with error" ),
                     i( "\"{}\"" ),
                    di( "Will ignore and continue running!" )
                )
            ),
            (
                MessageType::INFO,
                "legacy_basemod_nearby",
                strings!(
                    di( "Legacy base.lua file found in current folder!" ),
                    di( "Attempting to derive a mod.txt..." )
                )
            ),
            (
                MessageType::INFO,
                "legacy_basemod_not_nearby",
                strings!(
                    di( "No legacy base.lua file found!" ),
                    di( "Will check subfolders for base.lua definitions..." )
                )
            ),
            (
                MessageType::INFO,
                "prompt_derive_multiselect",
                i(red( "Which mods would you like to derive a mod.txt for?" ))
            ),
            (
                MessageType::INFO,
                "postprompt_derive_multiselect",
                di(red( "Deriving a mod.txt for the following mods:" ))
            ),
            (
                MessageType::INFO,
                "init_lua_context",
                di("Initialising Lua context...")
            ),
            (
                MessageType::INFO,
                "init_dahm_context",
                di("Initialising fake Dahm context...")
            ),
            (
                MessageType::INFO,
                "title_msg",
                strings!(
                    di(     "You are using"         ),
                    i(red(  "ThatDahmModConverter"  )),
                    i(      "v0.0.1"                )
                )
            ),
            (
                MessageType::INFO,
                "madeby_msg",
                strings!(
                    di(     "...made with"          ),
                    di(red( "<3"                    )),
                    di(     "by"                    ),
                    i(      "Sprixitite"            )
                )
            )
        ].map(|x| (x.1, Message::new(x.2, x.0))).into()
    };
}