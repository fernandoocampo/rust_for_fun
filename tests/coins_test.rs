use rust_for_fun::coins::cents;

#[test]
fn test_value_in_cents() {
    // GIVEN
    let given_value = cents::Coin::Penny;
    let expected_result: u8 = 1;
    // WHEN
    let got = cents::value_in_cents(given_value);
    // THEN
    assert_eq!(got, expected_result);
}

#[test]
fn test_plus_one() {
    // GIVEN
    let given_value_one = Some(5);
    let given_value_two = None;

    let expected_result_one = Some(6);
    let expected_result_two = None;
    // WHEN
    let got_one = cents::plus_one(given_value_one);
    let got_two = cents::plus_one(given_value_two);
    // THEN
    assert_eq!(got_one, expected_result_one);
    assert_eq!(got_two, expected_result_two);
}
