use rust_for_fun::people::person;
use std::str::FromStr;

#[test]
fn test_new_person() {
    // GIVEN
    let person_id = person::PersonID::from_str("1").expect("No id provided");
    let name = String::from("Fernando");
    let expected_person = person::Person {
        id: person::PersonID::from_str("1").expect("No id provided"),
        name: String::from("Fernando"),
    };
    // WHEN
    let got = person::Person::new(person_id, name);
    // THEN
    assert_eq!(got.name, expected_person.name);
}
