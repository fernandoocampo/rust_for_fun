use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    io::{Error, ErrorKind},
    str::FromStr,
    sync::Arc,
};

use tokio::sync::RwLock;

#[derive(Debug)]
pub struct Pagination {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    pub id: PersonID,
    pub name: String,
}

impl Person {
    pub fn new(id: PersonID, name: String) -> Self {
        Person { id, name }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, Hash, PartialEq)]
pub struct PersonID(pub String);

impl FromStr for PersonID {
    type Err = std::io::Error;

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() {
            false => Ok(PersonID(id.to_string())),
            true => Err(Error::new(ErrorKind::InvalidInput, "No id provided")),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Store {
    pub people: Arc<RwLock<HashMap<PersonID, Person>>>,
}

impl Store {
    pub fn new() -> Self {
        Store {
            people: Arc::new(RwLock::new(Self::init())),
        }
    }

    fn init() -> HashMap<PersonID, Person> {
        let file = include_str!("../../people.json");
        serde_json::from_str(file).expect("can't read people.json")
    }

    // fn add_person(mut self, person: Person) -> Self {
    //     self.people.insert(person.id.clone(), person);
    //     self
    // }
}
