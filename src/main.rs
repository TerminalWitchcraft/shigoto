extern crate shigoto;
extern crate clap;


use clap::{Arg, App, SubCommand, ArgMatches};
use shigoto::Show;

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
        //("done", Some(sub_m)) => {},
        //("rm", Some(sub_m)) => {},
        //("list", Some(sub_m)) => {},
        _ => {}
    }

    let mut conf = match shigoto::Config::new() {
        Ok(r) => r,
        Err(e) => panic!("Shitt something bad happened!!!{:?}",e)
    };
    shigoto::commands::add(&mut conf, "hello").unwrap();
    shigoto::commands::add(&mut conf, "a_task").unwrap();
    conf.user_data.show().unwrap();

    //shigoto::commands::list(conf).unwrap();
}

fn run(sub_m: &ArgMatches, name: &str) {
    if let Some(val) = sub_m.value_of(name) {
        println!("Printing... {:?}", val)
    } else {
        println!("No value")
    }
}

