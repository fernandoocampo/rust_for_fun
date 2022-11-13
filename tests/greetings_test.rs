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
