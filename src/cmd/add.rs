use config::{Config, Task};
use serde_json::Error;


pub fn execute(conf: &mut Config, name: &str) -> Result<(), Error> {
    let task = Task::with_default(name);
    conf.user_data.tasks.push(task);
    Ok(())
}
