pub fn hello() -> String {
    String::from("Hello")
}

pub fn welcome(name: &str) -> String {
    let mut welcome = String::from("Welcome ");
    welcome.push_str(name);

    welcome
}
