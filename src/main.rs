extern crate shigoto;
extern crate clap;
extern crate chrono;


use clap::{Arg, App, SubCommand};
use shigoto::cmd;
use std::process;

fn main() {
    let matches = App::new("Shigoto")
        .version("0.1.0")
        .author("Hitesh Paul <paulhitesh.hp@gmail.com>")
        .about("Simple Command line tool to organize TODO tasks")
        .subcommand(SubCommand::with_name("add")
                    .about("Adds a task to your task list")
                    .arg(Arg::with_name("TASK")
                         .help("Adds the given task"))
                    .arg(Arg::with_name("TAG")
                         .help("Give a tag to this task")
                         .default_value(""))
                    .arg(Arg::with_name("DUE")
                         .help("Due date for this task")
                         .default_value("")))
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
    let mut conf = match shigoto::config::Config::new() {
        Ok(r) => r,
        Err(e) => panic!("Shitt something bad happened!!!{:?}",e)
    };
    match matches.subcommand() {
        ("add", Some(sub_m)) => {
            match cmd::add::execute(&mut conf, sub_m.value_of("TASK").unwrap()) {
                Ok(_) => {},
                Err(e) => {
                    println!("Failed to execute {:?}", e);
                    process::exit(1);
                }
            };},
            ("list", Some(_)) => {
                match cmd::list::execute(conf) {
                    Ok(_) => {},
                    Err(e) => {
                    println!("Failed to execute {:?}", e);
                    process::exit(1);
                    }
                }
            },
            ("rm", Some(sub_m)) => {
                match cmd::remove::execute(&mut conf, sub_m.value_of("TASK_ID").unwrap()) {
                    Ok(_) => {},
                    Err(e) => {
                    println!("Failed to execute {:?}", e);
                    process::exit(1);
                    }
                }

            },
        _ => {}
    }
}
