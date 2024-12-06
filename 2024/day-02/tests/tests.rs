use day_02::{
    parse_input,
    part_one,
};

#[test]
fn test_parse_input() {
    let input = "7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9";

    let expected = vec![
        vec![7, 6, 4, 2, 1],
        vec![1, 2, 7, 8, 9],
        vec![9, 7, 6, 2, 1],
        vec![1, 3, 2, 4, 5],
        vec![8, 6, 4, 4, 1],
        vec![1, 3, 6, 7, 9]
    ];

    let actual = parse_input(input);

    assert_eq!(actual, expected);
}

#[test]
fn test_part_one() {
    let reports = vec![
        vec![7, 6, 4, 2, 1],
        vec![1, 2, 7, 8, 9],
        vec![9, 7, 6, 2, 1],
        vec![1, 3, 2, 4, 5],
        vec![8, 6, 4, 4, 1],
        vec![1, 3, 6, 7, 9]
    ];

    let expected = 2;

    let actual = part_one(reports);

    assert_eq!(actual, expected);
}
