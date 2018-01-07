extern crate clap;
extern crate termion;

use clap::{Arg, App, SubCommand};
use termion::color;

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

    if let Some(matches) = matches.subcommand_matches("add") {
        if matches.is_present("TASK") {
            println!("Thank god you are educated!")
        } else {
            println!("{Red}Damn!! No task to add...",
                     Red = color::Fg(color::Red))
        }
    }
}
