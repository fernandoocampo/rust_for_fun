extern crate rust_for_fun;

use rust_for_fun::greetings::hi::hello;
use rust_for_fun::people::{handler, person};
use warp::{http::Method, Filter};

#[tokio::main]
async fn main() {
    let store = person::Store::new();
    let store_filter = warp::any().map(move || store.clone());

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST]);

    let hello_greeting = hello();
    println!("> {hello_greeting}, starting server on 1337");

    let get_people = warp::get()
        .and(warp::path("people"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter.clone())
        .and_then(handler::get_people);

    let get_person = warp::get()
        .and(warp::path("people"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(handler::get_person);

    let add_people = warp::post()
        .and(warp::path("people"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(handler::add_person);

    let update_people = warp::put()
        .and(warp::path("people"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(handler::update_person);

    let delete_people = warp::delete()
        .and(warp::path("people"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(handler::delete_person);

    let routes = get_people
        .or(get_person)
        .or(add_people)
        .or(update_people)
        .or(delete_people)
        .with(cors)
        .recover(handler::return_error);

    warp::serve(routes).run(([127, 0, 0, 1], 1337)).await;
    println!("ending server");
}
