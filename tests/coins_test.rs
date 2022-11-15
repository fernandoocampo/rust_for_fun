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
