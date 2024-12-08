use day_03::{
    part_one,
    part_two,
    parse_instructions
};
use std::collections::BTreeMap;

#[test]
fn test_part_one() {
    let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    let expected = 161;
    let actual = part_one(input);

    assert_eq!(expected, actual);
}

#[test]
fn test_part_two() {
    let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    let expected = 48;
    let actual = part_two(input);

    assert_eq!(expected, actual);
}

#[test]
fn test_parse_instructions() {
    let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
   
    let expected: BTreeMap<usize, String> = [
            (0, "mul(2,4)".to_string()),
            (1, "don't()".to_string()),
            (2, "mul(5,5)".to_string()),
            (3, "mul(11,8)".to_string()),
            (4, "do()".to_string()),
            (5, "mul(8,5)".to_string()),
        ].iter().cloned().collect();

    let actual = parse_instructions(input);

    assert_eq!(expected, actual);
}
