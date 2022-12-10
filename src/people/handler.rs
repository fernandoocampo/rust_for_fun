use std::str::FromStr;
use crate::people::person::{
    Person,
    PersonID
};
use warp::{
    Reply,
    Rejection,
    hyper::StatusCode,
    reject::Reject
};

#[derive(Debug)]
struct InvalidID;
impl Reject for InvalidID {}

pub async fn get_people() -> Result<impl Reply, Rejection> {
    let person = Person::new(
        PersonID::from_str("1").expect("no id provided"),
        "Fernando".to_string(),
    );

    match person.id.0.parse::<i32>() {
        Err(_) => {
            Err(warp::reject::custom(InvalidID))
        },
        Ok(_) => {
            Ok(warp::reply::json(
                &person
            ))
        }
    }
}

pub async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    println!("wtf {:?}", r);
    if let Some(_invalid_id) = r.find::<InvalidID>() {
        Ok(warp::reply::with_status(
            "No valid ID presented",
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Route not found",
            StatusCode::NOT_FOUND,
        ))
    }
}