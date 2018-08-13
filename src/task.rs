use std::string::ToString;
use chrono::prelude::*;


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


#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub priority: Priority,
    pub created_on: DateTime<Utc>, 
    pub due: DateTime<Utc>,
    pub name: String,
    pub is_completed: bool,
    pub tags: Vec<String>,
}


impl Task {
    pub fn with_default(name: &str) -> Task {
        Task {
            priority: Priority::Medium,
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

    pub fn new(name: &str, priority: &str, tags: Vec<String>) -> Task {
        Task {
            priority: Priority::Medium,
            created_on: Utc::now(),
            due: Utc::now(),
            name: name.to_string(),
            is_completed: false,
            tags
        }
    }
    //fn p_highlight(priority: i8) -> u16 {
    //    if priority == 1 {
    //        color::RED
    //    } else if priority == 2 {
    //        color::YELLOW
    //    } else {
    //        color::BLUE
    //    }
    //}

}
