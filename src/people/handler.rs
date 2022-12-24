use crate::people::person::{Pagination, Person, PersonID, Store};
use std::{collections::HashMap, fmt, fmt::Display, fmt::Formatter, num::ParseIntError};
use warp::{
    filters::{body::BodyDeserializeError, cors::CorsForbidden},
    hyper::StatusCode,
    reject::Reject,
    Rejection, Reply,
};

#[derive(Debug)]
pub enum Error {
    ParseError(ParseIntError),
    MissingParameters,
    PersonNotFound,
}

impl Reject for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Error::ParseError(ref err) => write!(f, "Cannot parse parameter: {err}"),
            Error::MissingParameters => write!(f, "Missing parameter"),
            Error::PersonNotFound => write!(f, "Person not found"),
        }
    }
}

#[derive(Debug)]
struct InvalidID;
impl Reject for InvalidID {}

pub fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
    if params.contains_key("start") && params.contains_key("end") {
        return Ok(Pagination {
            start: params
                .get("start")
                .unwrap()
                .parse::<usize>()
                .map_err(Error::ParseError)?,
            end: params
                .get("end")
                .unwrap()
                .parse::<usize>()
                .map_err(Error::ParseError)?,
        });
    }
    Err(Error::MissingParameters)
}

pub async fn get_people(
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl Reply, Rejection> {
    if params.is_empty() {
        let res: Vec<Person> = store.people.read().await.values().cloned().collect();
        return Ok(warp::reply::json(&res));
    }

    let pagination = extract_pagination(params)?;
    let res: Vec<Person> = store.people.read().await.values().cloned().collect();
    let res = &res[pagination.start..pagination.end];

    Ok(warp::reply::json(&res))
}

pub async fn update_person(
    id: String,
    store: Store,
    person: Person,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.people.write().await.get_mut(&PersonID(id)) {
        Some(p) => *p = person,
        None => return Err(warp::reject::custom(Error::PersonNotFound)),
    }

    Ok(warp::reply::with_status("Person updated", StatusCode::OK))
}

pub async fn add_person(store: Store, person: Person) -> Result<impl warp::Reply, warp::Rejection> {
    store.people.write().await.insert(person.id.clone(), person);

    Ok(warp::reply::with_status("People added", StatusCode::OK))
}

pub async fn delete_person(id: String, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    match store.people.write().await.remove(&PersonID(id)) {
        Some(_) => Ok(warp::reply::with_status("Person deleted", StatusCode::OK)),
        None => Err(warp::reject::custom(Error::PersonNotFound)),
    }
}

pub async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(error) = r.find::<Error>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::RANGE_NOT_SATISFIABLE,
        ))
    } else if let Some(error) = r.find::<CorsForbidden>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::FORBIDDEN,
        ))
    } else if let Some(error) = r.find::<BodyDeserializeError>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Route not found".to_string(),
            StatusCode::NOT_FOUND,
        ))
    }
}
