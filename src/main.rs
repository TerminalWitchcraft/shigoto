extern crate clap;
//extern crate termion;
extern crate serde;
extern crate chrono;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;


use std::fs::{ File, OpenOptions };
use std::io::{ Read, Write, Error };
use std::path::Path;
use clap::{Arg, App, SubCommand, ArgMatches};
use chrono::prelude::*;

fn main() {
    let matches = App::new("Shigoto")
        .version("0.1.0")
        .author("Hitesh Paul <paulhitesh.hp@gmail.com>")
        .about("Simple Command line tool to organize TODO tasks")
        .subcommand(SubCommand::with_name("add")
                    .about("Adds a task to your task list")
                    .arg(Arg::with_name("TASK")
                         .help("Adds the given task")))
        .subcommand(SubCommand::with_name("done")
                    .about("Marks a task as completed")
                    .arg(Arg::with_name("TASK_ID"))
                        .help("Marks a task with given TASK_ID as completed"))
        .subcommand(SubCommand::with_name("rm")
                    .about("Removes a task from task list")
                    .arg(Arg::with_name("TASK_ID")
                         .help("Removes the task corresponding to TASK_ID")))
        .subcommand(SubCommand::with_name("list")
                    .about("Show the task list"))
        .get_matches();

    //if let Some(sub_matches) = matches.subcommand_matches("add") {
    //    if sub_matches.is_present("TASK") {
    //        println!("Thank god you are educated!")
    //    } else {
    //        println!("{Red}Damn!! No task to add...",
    //                 Red = color::Fg(color::Red))
    //    }
    //}
    match matches.subcommand() {
        ("add", Some(sub_m)) => {run(&sub_m, "TASK")},
        ("done", Some(sub_m)) => {},
        ("rm", Some(sub_m)) => {},
        ("list", Some(sub_m)) => {},
        _ => {}
    }

    let conf = match Config::new("user.json") {
        Ok(r) => r,
        Err(_) => panic!("Shitt something bad happened!!!")
    };
}

fn run(sub_m: &ArgMatches, name: &str) {
    if let Some(val) = sub_m.value_of(name) {
        println!("Printing... {:?}", val)
    } else {
        println!("No value")
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: u32,
    priority: i8,
    created_on: DateTime<Utc>, 
    due: DateTime<Utc>,
    name: String,
    is_completed: bool,
    tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct UserData {
    tasks: Vec<Task>,
}

impl UserData {
    fn new() -> UserData {
        let tasks = Vec::new();
        UserData {
            tasks
        }
    }
}

struct Config {
    user_data: UserData,
}

impl Config {
    fn new(file_name: &str) -> serde::export::Result<Config, Box<Error>> {
        let file = OpenOptions::new()
            .create(true)
            .read(true)
            .open(file_name)?;
        let user_data: UserData = match serde_json::from_reader(file) {
            Ok(r) => r,
            Err(_) => UserData::new(),
        };
        Ok(Config { user_data })
    }
}
