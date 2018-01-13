use std::io::Error;
use config::{Config};
use config::Show;

pub fn execute(conf: Config) -> Result<(), Error> {
    conf.user_data.show()?;
    Ok(())
}
