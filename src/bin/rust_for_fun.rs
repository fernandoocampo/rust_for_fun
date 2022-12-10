extern crate rust_for_fun;

use warp::{
    Filter,
    http::Method
};
use rust_for_fun::greetings::hi::hello;
use rust_for_fun::people::handler;



#[tokio::main]
async fn main() {
    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[
            Method::PUT,
            Method::DELETE,
            Method::GET,
            Method::POST
        ]);

    let hello_greeting = hello();
    println!("> {}, starting on 1337", hello_greeting);

    // let hello = warp::get()
    //     .map(|| format!("Hello, World!"));
    
    let get_people = warp::get()
        .and(warp::path("people"))
        .and(warp::path::end())
        .and_then(handler::get_people)
        .recover(handler::return_error);
    
    let routes = get_people.with(cors);

    warp::serve(routes)
        .run(([127,0,0,1], 1337))
        .await;
}
