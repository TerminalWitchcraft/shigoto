use config::Config;
use task::Task;
use serde_json::Error;


pub fn execute(conf: &mut Config, name: &str, priority: &str, tags: &str) -> Result<(), Error> {
    let task = Task::new(name, priority, tags.split_terminator(",").collect());
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
