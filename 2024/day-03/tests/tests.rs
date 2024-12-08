use day_03::{
    part_one,
    part_two
};

#[test]
fn test_part_one() {
    let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    let expected = 161;
    let actual = part_one(input);

    assert_eq!(expected, actual);
}
