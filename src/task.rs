use std::string::ToString;
use chrono::prelude::*;
use chrono::Duration;
use chrono::{NaiveDate};



#[allow(dead_code)]
fn is_leap_year(year: i32) -> bool {
    NaiveDate::from_ymd_opt(year, 2, 29).is_some()
}

fn last_day_of_month(year: i32, month: u32) -> u32 {
    NaiveDate::from_ymd_opt(year, month + 1, 1).unwrap_or(NaiveDate::from_ymd(year + 1, 1, 1)).pred().day()
}

fn time_from_str(due:&str) -> DateTime<Local> {
    match due.to_lowercase().as_str() {
        // End dates
        "eod"   => Local::now() + Duration::hours(24),
        "eow"   => Local::now() + Duration::weeks(1),
        "eocw"  => Local::now() + Duration::weeks(1),
        "eom"   => {let dt = Local::now();
                    let last_day = last_day_of_month(dt.year(), dt.month());
                    dt + Duration::days(last_day as i64 - dt.day() as i64)
                    },
        // TODO Next year fix
        "eoq"   => {let dt = Local::now();
                    let month: u32 = match dt.month() {
                        1...3 => 3,
                        4...6 => 6,
                        7...9 => 9,
                        10...12 => 12,
                        e => e
                    };
                    Local.from_local_datetime(&NaiveDate::from_ymd(dt.year(), month, dt.day())
                                              .and_hms(dt.hour(), dt.minute(), dt.second()))
                                              .unwrap()
                    }
        "eoy"   => {let dt = Local::now();
                    Local.from_local_datetime(&NaiveDate::from_ymd(dt.year(), 12, 31)
                                              .and_hms(0, 0, 0))
                        .unwrap()
                    },

        // Start dates(Start of the next...)
        "sod"   =>  Local::now() + Duration::hours(24),
        "sow"   =>  {let dt = Local::now();
                    let num = dt.weekday().number_from_monday();
                    dt + Duration::days(7 as i64 - num as i64)
                    },
        "socw"   =>  {let dt = Local::now();
                    let num = dt.weekday().number_from_monday();
                    dt + Duration::days(7 as i64 - num as i64)
                    }
        // TODO unwrap_or logic
        "som"   => {let dt = Local::now();
                    Local.from_local_datetime(&NaiveDate::from_ymd(dt.year(), dt.month() + 1, 1)
                                              .and_hms(dt.hour(), dt.minute(), dt.second()))
                        .unwrap()
                    },
        // TODO Next year fix
        "soq"   => {let dt = Local::now();
                    let month: u32 = match dt.month() {
                        1...3 => 4,
                        4...6 => 7,
                        7...9 => 10,
                        10...12 => 1,
                        e => e
                    };
                    Local.from_local_datetime(&NaiveDate::from_ymd(dt.year(), month, dt.day())
                                              .and_hms(dt.hour(), dt.minute(), dt.second()))
                                              .unwrap()
                    },
        "soy"   => {let dt = Local::now();
                    Local.from_local_datetime(&NaiveDate::from_ymd(dt.year()+1, 12, 31)
                                              .and_hms(0, 0, 0))
                        .unwrap()
                    },
        _       => Local::now(),
    }
}


/// Priority defines the urgency level of tasks
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Priority {
    /// Task with High priority
    High,

    /// Task with Medium priority
    Medium,
    
    /// Task with Low priority
    Low
}

impl ToString for Priority {
    fn to_string (&self) -> String {
        match self {
            &Priority::High => "High".to_string(),
            &Priority::Medium => "Medium".to_string(),
            &Priority::Low => "Low".to_string(),
        }
    }

}

fn priority_from_str(data: &str) -> Priority {
    match data.to_lowercase().as_str() {
        "high" => Priority::High,
        "medium" => Priority::Medium,
        "low"   => Priority::Low,
        _ => Priority::Medium,
    }
}


/// Task is the data structure which holds each item. User data consists of a number of tasks
/// mapped with unique `ID`. A (HashMap)[std::collections::HashMap] type is used to store the
/// collection of tasks.
#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    /// Defines the urgency for the task
    pub priority: Priority,

    /// Time when the task was created. Currently uses system's localtime
    pub created_on: DateTime<Local>, 

    /// Due date for the task. Currently uses system's localtime
    pub due: DateTime<Local>,

    /// Name for the task
    pub name: String,

    /// Bool to indicate whether the task is completed or not.
    pub is_completed: bool,

    /// Tags associated with the task.
    pub tags: Vec<String>,
}


impl Task {
    /// Test function to creat a task with some defaults.
    /// ```
    /// use shigoto::task::Task;
    ///
    /// let task = Task::with_default("test_task");
    ///
    /// assert_eq!("test_task".to_string(), task.name);
    /// ```
    pub fn with_default(name: &str) -> Task {
        Task {
            priority: Priority::Medium,
            created_on: Local::now(),
            due: Local::now(),
            name: name.to_string(),
            is_completed: false,
            tags: {
                let mut v = Vec::new();
                v.push(String::from("hello"));
                v
            }
        }
    }

    /// Function to create a new task with supplied parameters
    /// ```
    /// use shigoto::task::Task;
    /// use shigoto::task::Priority;
    ///
    /// let task = Task::new("A test task", "high", "test,cargo".split(",").map(|v| String::from(v))
    ///                     .collect(), "eod");
    /// assert_eq!("A test task".to_string(), task.name);
    /// assert_eq!(Priority::High, task.priority);
    /// ```
    pub fn new(name: &str, priority: &str, tags: Vec<String>, due: &str)
        -> Task {
        Task {
            priority: priority_from_str(priority),
            created_on: Local::now(),
            due: time_from_str(due),
            name: name.to_string(),
            is_completed: false,
            tags
        }
    }

}
