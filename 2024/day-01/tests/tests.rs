use day_01::parse_input;

#[test]
fn test_parse_input() {
    let input = "3   4
        4   3
        2   5
        1   3
        3   9
        3   3";

    let expected_left = vec![3, 4, 2, 1, 3, 3];
    let expected_right = vec![4, 3, 5, 3, 9, 3];

    let (left, right) = parse_input(input);

    assert_eq!(left, expected_left, "Left column parsing failed");
    assert_eq!(right, expected_right, "Right column parsing failed");
}
