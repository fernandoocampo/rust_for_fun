use std::io::{Error, ErrorKind};
use std::str::FromStr;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Person {
    pub id: PersonID,
    pub name: String,
}

impl Person {
    pub fn new(id: PersonID, name: String) -> Self {
        Person { id, name }
    }
}

#[derive(Debug, Serialize)]
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
