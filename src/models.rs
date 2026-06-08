// src/models.rs
use chrono::NaiveDate;
use std::fmt;

// Struct
#[derive(Debug, Clone)]
pub struct Account {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Category {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Counterparty {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Transaction {
    pub account_id: i32,
    pub date: NaiveDate,
    pub amount: f64,
    pub direction: String,
    pub note: String,
    pub category_id: i32,
    pub counterparty_id: Option<i32>,
}

// Display methods for Struct
impl fmt::Display for Account {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl fmt::Display for Counterparty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
