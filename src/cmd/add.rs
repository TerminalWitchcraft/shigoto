use config::Config;
use task::Task;
use serde_json::Error;


pub fn execute(conf: &mut Config, name: &str, priority: &str,
               tags: &str, due: &str)
    -> Result<(), Error> {
    let task = Task::new(name, priority,
                         tags
                         .split(",")
                         .map(|v| String::from(v))
                         .collect::<Vec<String>>(),
                         due);
    conf.user_data.add_task(task).unwrap();
    conf.save().unwrap();
    Ok(())
}
