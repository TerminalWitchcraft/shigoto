extern crate serde_json;

use serde_json::Error;
use super::{Config, UserData, Task};


pub fn add(conf: &mut Config, name: &str) -> Result<(), Error> {
    let task = Task::with_default(name);
    conf.user_data.tasks.push(task);
    Ok(())
}

pub fn done(conf: &mut Config) -> Result<(), Error> {
    println!("Hello");
    Ok(())
}

pub fn remove(conf: &mut Config) -> Result<(), Error> {
    println!("Hello");
    Ok(())
}

pub fn list(conf: Config) -> Result<(), Error> {
    for task in conf.user_data.tasks.iter() {
        println!("{id}: {name}",
                 id=task.id,
                 name=task.name);
    }
    Ok(())
}
