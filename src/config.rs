use std::fs::{ OpenOptions};
use std::fs;
use std::io::Error;
use std::env;
use std::path::PathBuf;
use std::collections::HashMap;
use serde;
use serde_json;
use chrono::prelude::*;

use prettytable::Table;
use prettytable::row::Row;
use prettytable::cell::Cell;
use prettytable::format;

pub trait Show {
    fn show(&self) -> Result<(), Error>;
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub priority: i8,
    pub created_on: DateTime<Utc>, 
    pub due: DateTime<Utc>,
    pub name: String,
    pub is_completed: bool,
    pub tags: Vec<String>,
}


impl Task {
    pub fn with_default(name: &str) -> Task {
        Task {
            priority: 2,
            created_on: Utc::now(),
            due: Utc::now(),
            name: name.to_string(),
            is_completed: false,
            tags: {
                let mut v = Vec::new();
                v.push(String::from("hello"));
                v
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserData {
    //pub tasks: Vec<Task>,
    pub tasks: HashMap<usize, Task>,
    count: usize,
}


impl UserData {
    pub fn new() -> UserData {
        let tasks: HashMap<usize, Task> = HashMap::new();
        UserData {
            tasks,
            count: 0,
        }
    }

    pub fn add_task(&mut self, task: Task) -> Result<(), Box<Error>> {
        let id: usize = self.count + 1;
        self.tasks.insert(id, task);
        self.count = self.count + 1;
        Ok(())
    }

    pub fn rm_task(&mut self, id:usize) -> Result<(), Box<Error>> {
        self.tasks.remove(&id);
        self.count = self.count -1;
        Ok(())
    }
}


impl Show for UserData {
    fn show(&self) -> Result<(), Error> {
        let mut table = Table::new();
        table.set_titles(Row::new(vec![
                               Cell::new("ID"),
                               Cell::new("Name"),
                               Cell::new("Created"),
                               Cell::new("Priority"),
                               Cell::new("Due"),
                               Cell::new("Completed?"),
                               Cell::new("Tags"),
        ]));
        for (id, task) in self.tasks.iter() {
            table.add_row(Row::new(vec![
                                   Cell::new(&id.to_string()),
                                   Cell::new(&task.name),
                                   Cell::new(&task.created_on.to_string()),
                                   Cell::new(&task.priority.to_string()),
                                   Cell::new(&task.due.to_string()),
                                   Cell::new(&task.is_completed.to_string()),
                                   Cell::new(&task.tags.join(",")),
            ]));
        }
        table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
        table.printstd();
    Ok(())
    }
}


pub struct Config {
    pub data_file: PathBuf,
    pub user_data: UserData,
}

impl Config {
    pub fn new() -> serde::export::Result<Config, Box<Error>> {
        let data_path: PathBuf = env::var("XDG_DATA_HOME")
            .map(|p| PathBuf::from(p).join("shigoto"))
            .unwrap_or_else(|_| {
                let home = env::home_dir().expect("No Home directory");
                home.join(".local").join("share").join("shigoto")
            });
        if !data_path.exists() {
            fs::create_dir_all(&data_path)
                .expect("Cannot create data_dir");
        }
        let data_file = data_path.join("data.json");

        if !data_file.exists() {
            fs::File::create(&data_file).expect("Failed to create file");
            return Ok(Config {
                data_file: data_file,
                user_data: UserData::new()
            })
        }
        let file = OpenOptions::new()
            .read(true)
            .open(&data_file)?;
        let user_data: UserData = match serde_json::from_reader(file) {
            Ok(r) => r,
            Err(_) => UserData::new(),
        };
        Ok(Config { data_file: data_file, user_data: user_data })
    }
}

impl Config {
    pub fn save(&mut self) -> Result<(), Error> {
        let f = OpenOptions::new()
            .write(true)
            .open(&self.data_file)?;
        serde_json::to_writer(f, &self.user_data)?;
        Ok(())
    }
}
