//! Shigoto is a command line tool to manage tasks. It is heavily inspired from taskwarrior.
//! Currently, it is in a very early state. Backward compatibilty will be guaranteed once v 1.0 is
//! released.
extern crate clap;
extern crate term;
extern crate serde;
extern crate chrono;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

extern crate prettytable;

/// Holds all the user data related to tasks.
pub mod config;

/// Holds all functions related to every command
pub mod cmd;

/// Data Structure for each task
pub mod task;


