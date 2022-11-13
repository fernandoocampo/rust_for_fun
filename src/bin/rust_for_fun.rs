extern crate rust_for_fun;

use rust_for_fun::greetings::hi::hello;

fn main() {
    let hello = hello();

    println!("> {}", hello);
}
