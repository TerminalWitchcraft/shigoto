use config::{Config, Task};
use serde_json::Error;


pub fn execute(conf: &mut Config, name: &str) -> Result<(), Error> {
    let task = Task::with_default(name);
    conf.user_data.add_task(task).unwrap();
    conf.save().unwrap();
    //match conf.save() {
    //    Ok(_) => {},
    //    Err(e) => {
    //        println!("error occured! {:?}", e);
    //    }
    //};
    Ok(())
}
