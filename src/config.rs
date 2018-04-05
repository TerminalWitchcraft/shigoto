use std::fs::{ OpenOptions};
use std::fs;
use std::io::Error;
use std::env;
use std::path::PathBuf;
use std::collections::HashMap;

use task::Task;
use task::Priority;

use serde;
use serde_json;
use prettytable::Table;
use prettytable::row::Row;
use prettytable::cell::Cell;
use prettytable::format;
use term::{color, Attr};

pub trait Show {
    fn show(&self) -> Result<(), Error>;
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
        if id <= self.count {
            self.tasks.remove(&id);
            self.count = self.count -1;
        } else {
            println!("Invalid Id. Use sg list to view available tasks");
        }
        Ok(())
    }

    pub fn done(&mut self, id:usize) -> Result<(), Box<Error>> {
        if id <= self.count {
            if let Some(x) = self.tasks.get_mut(&id) {
                x.is_completed = true;
            };
        } else {
            println!("Invalid Id. Use sg list to view available tasks");
        }
        Ok(())
    }
}


impl Show for UserData {
    fn show(&self) -> Result<(), Error> {
        if self.count == 0 {
            println!("No data found. Type sg --help for usage")
        } else {
            let mut table = Table::new();
            let format = format::FormatBuilder::new()
                .column_separator('|')
                .borders('|')
                .separators(&[format::LinePosition::Top],
                            format::LineSeparator::new('─', '┬', '┌', '┐'))
                .separators(&[format::LinePosition::Bottom],
                            format::LineSeparator::new('─', '┴', '└', '┘'))
                .separators(&[format::LinePosition::Title],
                            format::LineSeparator::new('=', '┼', '├', '┤'))
                .separators(&[format::LinePosition::Intern],
                            format::LineSeparator::new('─', '┼', '├', '┤'))
                .padding(1,1)
                .build();
            table.set_format(format);
            table.set_titles(Row::new(vec![
                                   Cell::new("ID")
                                   .with_style(Attr::ForegroundColor(color::BRIGHT_BLUE)),
                                   Cell::new("Name")
                                   .with_style(Attr::ForegroundColor(color::BRIGHT_BLUE)),
                                   Cell::new("Created")
                                   .with_style(Attr::ForegroundColor(color::BRIGHT_BLUE)),
                                   Cell::new("Priority")
                                   .with_style(Attr::ForegroundColor(color::BRIGHT_BLUE)),
                                   Cell::new("Due")
                                   .with_style(Attr::ForegroundColor(color::BRIGHT_BLUE)),
                                   Cell::new("Completed?")
                                   .with_style(Attr::ForegroundColor(color::BRIGHT_BLUE)),
                                   Cell::new("Tags")
                                   .with_style(Attr::ForegroundColor(color::BRIGHT_BLUE)),
            ]));
            for (id, task) in self.tasks.iter() {
                table.add_row(Row::new(vec![
                                       Cell::new(&id.to_string()),
                                       Cell::new(&task.name),
                                       Cell::new(&task.created_on.format("%R %a, %d %b %y'").to_string()),
                                       Cell::new(&task.priority.to_string())
                                       .with_style(Attr::ForegroundColor(match task.priority {
                                           Priority::High => color::RED,
                                           Priority::Medium => color::YELLOW,
                                           _ => color::BLUE,
                                       })),
                                       Cell::new(&task.due.format("%R %a, %d %b %y'").to_string()),
                                       Cell::new(&task.is_completed.to_string())
                                       .with_style(Attr::ForegroundColor(match task.is_completed {
                                           true => color::BRIGHT_GREEN,
                                           false => color::BRIGHT_YELLOW
                                       })),
                                       Cell::new(&task.tags.join(",")),
                ]));
            }
            table.printstd();
        }
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

        if !data_file.is_file() {
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
            .truncate(true)
            .open(&self.data_file)?;
        serde_json::to_writer(f, &self.user_data)?;
        Ok(())
    }
}
