use rust_for_fun::greetings::hi;

#[test]
fn test_hello() {
    // GIVEN
    let expected_greeting = String::from("Hello");
    // WHEN
    let got = hi::hello();
    // THEN
    assert_eq!(got, expected_greeting);
}

#[test]
fn test_welcome() {
    // GIVEN
    let person_name = "Lee";
    let expected_greeting = String::from("Welcome Lee");
    // WHEN
    let got: String = hi::welcome(person_name);
    // THEN
    assert_eq!(got, expected_greeting);
}
