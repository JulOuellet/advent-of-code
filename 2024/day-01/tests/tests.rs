use day_01::{
    parse_input,
    part_one,
    part_two
};

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

#[test]
fn test_part_one() {
    let left = vec![3, 4, 2, 1, 3, 3];
    let right = vec![4, 3, 5, 3, 9, 3];

    let expected_dist = 11;

    let dist = part_one(&left, &right);

    assert_eq!(dist, expected_dist, "Part one failed");
}

#[test]
fn test_part_two() {
    let left = vec![3, 4, 2, 1, 3, 3];
    let right = vec![4, 3, 5, 3, 9, 3];

    let expected_score = 31;

    let score = part_two(&left, &right);

    assert_eq!(score, expected_score, "Part two failed");
}

