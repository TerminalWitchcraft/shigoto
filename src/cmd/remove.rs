use serde_json::Error;
use config::{Config};

pub fn execute(conf: &mut Config) -> Result<(), Error> {
    println!("Hello");
    Ok(())
}
