use i_float::fix_float::FixConvert;
use i_float::fix_vec::FixVec;

#[test]
fn test_function_0() {
    let a = FixVec::new_number(1, 1);
    let b = FixVec::new_number(1, -1);
    let c = a + b;

    assert_eq!(c, FixVec::new_number(2, 0));
}

#[test]
fn test_function_1() {
    let a = FixVec::new_number(3, 4);

    assert_eq!(a.fix_sqr_length(), 25.fix());
}

#[test]
fn test_function_2() {
    let a = FixVec::new_number(3, 4);

    assert_eq!(a.length(), 5.fix());
}