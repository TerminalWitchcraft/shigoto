use serde_json::Error;
use config::{Config};

pub fn execute(conf: Config) -> Result<(), Error> {
    for task in conf.user_data.tasks.iter() {
        println!("{id}: {name}",
                 id=task.id,
                 name=task.name);
    }
    Ok(())
}
