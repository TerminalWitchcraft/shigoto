use std::string::ToString;
use chrono::prelude::*;
use chrono::Duration;
use chrono::{NaiveDate};


#[derive(Serialize, Deserialize, Debug)]
pub enum Date {
    Exact(Exact),
    Start(Start),
    End(End),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Exact {
    Dmy, // Day, Month, Year
    Mdy, // Month, Day, Year
}

impl ToString for Exact {
    fn to_string(&self) -> String {
        match self {
            &Exact::Dmy => "DMY".to_string(),
            &Exact::Mdy => "MDY".to_string()
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Start {
    Sow, // Start of Week
    Socw, // Start of Calender Week
    Som, // Start of Month
    Soq, // Start of quarter
    Soy, // Start of year
}

impl ToString for Start {
    fn to_string(&self) -> String {
        match self {
            &Start::Sow     => "sow".to_string(),
            &Start::Socw    => "socw".to_string(),
            &Start::Som     => "som".to_string(),
            &Start::Soq     => "soq".to_string(),
            &Start::Soy     => "soy".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum End {
    Eow, // End of Week
    Eocw, // End of Calender Week
    Eom, // End of Month
    Eoq, // End of quarter
    Eoy, // End of year
}

impl ToString for End {
    fn to_string(&self) -> String {
        match self {
            &End::Eow     => "eow".to_string(),
            &End::Eocw    => "eocw".to_string(),
            &End::Eom     => "eom".to_string(),
            &End::Eoq     => "eoq".to_string(),
            &End::Eoy     => "eoy".to_string(),
        }
    }
}

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
        //"eoq"   => {}
        "eoy"   => {let dt = Local::now();
                    Local.from_local_datetime(&NaiveDate::from_ymd(dt.year(), 12, 31)
                                              .and_hms(0, 0, 0))
                        .unwrap()
                    },
        _       => Local::now(),

        // Start dates(Start of the next...)
        //"sod"   =>
        //"sow"   =>
        //"socw"  =>
        //"som"   =>
        //"soq"   =>
        //"soy"   =>
    }
}



#[derive(Serialize, Deserialize, Debug)]
pub enum Priority {
    High,
    Medium,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub priority: Priority,
    pub created_on: DateTime<Local>, 
    pub due: DateTime<Local>,
    pub name: String,
    pub is_completed: bool,
    pub tags: Vec<String>,
}


impl Task {
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
