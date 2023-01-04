use rust_for_fun::calculator::sum::Sum;

#[test]
fn test_which_two() {
    // GIVEN
    let target = 9;
    let nums = vec![2, 7, 11, 15];
    let expected_result = vec![0, 1];
    // WHEN
    let result = Sum::which_two(nums, target);
    // THEN
    assert_eq!(result.get(1), expected_result.get(0));
    assert_eq!(result.get(0), expected_result.get(1));
}

#[test]
fn test_which_two_two() {
    // GIVEN
    let target = 6;
    let nums = vec![3, 2, 4];
    let expected_result = vec![1, 2];
    // WHEN
    let result = Sum::which_two(nums, target);
    // THEN
    assert_eq!(result.get(1), expected_result.get(0));
    assert_eq!(result.get(0), expected_result.get(1));
}
