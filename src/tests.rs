use super::fixed::*;

macro_rules! fixed_fraction {
    ($n: expr, $d: expr) => {
        FixedPoint::<$d>::new($n)
    };
}

#[test]
fn fixed_point_to_integer_conversion() {
    assert_eq!(fixed_fraction!(1, 1).integral(), 1, "Integral part of 1 is not 1.");
    assert_eq!(fixed_fraction!(3, 2).integral(), 1, "Integral part of 1.5 is not 1.");
    assert_eq!(fixed_fraction!(3, 2).fractional(), (1, 2), "Fractional part of 1.5 is not 1/2.");
    assert_eq!(fixed_fraction!(1, 2).integral(), 0, "Integral part of 1/2 is not 0.");
    assert_eq!(fixed_fraction!(1, 2).fractional(), (1, 2), "Fractional part of 1/2 is not 0.");
    assert_eq!(fixed_fraction!(2, 2).integral(), 1, "Integral part of 2/2 is not 1.");
    assert_eq!(fixed_fraction!(2, 2).fractional(), (0, 1), "Fractional part of 2/2 is not 0.")
}

#[test]
fn fixed_point_to_float_conversion() {
    assert_eq!(f32::from(fixed_fraction!(1, 1)), 1.0, "Fixed point 1 does not convert to floating point 1.");
    assert_eq!(f32::from(fixed_fraction!(3, 2)), 1.5, "Fixed point 1.5 does not convert to floating point 1.5.");
    assert_eq!(f32::from(fixed_fraction!(1, 2)), 0.5, "Fixed point 0.5 does not convert to floating point 0.5.");
    assert_eq!(f32::from(fixed_fraction!(2, 2)), 1.0, "Fixed point 2/2 does not convert to floating point 1.");
}

#[test]
fn fixed_point_equality() {
    assert_eq!(fixed_fraction!(1, 1), fixed_fraction!(1, 1), "1 is not equal to 1.");
    assert_eq!(fixed_fraction!(1, 2), fixed_fraction!(1, 2), "1/2 is not equal to 1/2.");
    assert_eq!(fixed_fraction!(3, 2), fixed_fraction!(3, 2), "1.5 is not equal to 1.5.");
    assert_ne!(fixed_fraction!(3, 2), fixed_fraction!(1, 1), "1.5 is equal to 1.");
    assert_ne!(fixed_fraction!(3, 2), fixed_fraction!(1, 1), "1.5 is equal to 0.5.");
    assert_eq!(fixed_fraction!(1, 1), fixed_fraction!(2, 2), "1 is not equal to a different representation of 2/2.");
}

#[test]
fn fixed_point_addition() {
    assert_eq!(fixed_fraction!(1, 1) + fixed_fraction!(1, 1), fixed_fraction!(2, 1), "1 + 1 is not 2.");
    assert_eq!(fixed_fraction!(1, 1) + fixed_fraction!(2, 1), fixed_fraction!(3, 1), "1 + 2 is not 3.");
    assert_eq!(fixed_fraction!(2, 1) + fixed_fraction!(1, 1), fixed_fraction!(3, 1), "2 + 1 is not 3.");
    
    assert_eq!(fixed_fraction!(1, 2) + fixed_fraction!(1, 2), fixed_fraction!(1, 1), "1/2 + 1/2 is not 1.");
    assert_eq!(fixed_fraction!(1, 4) + fixed_fraction!(1, 4), fixed_fraction!(1, 2), "1/4 + 1/4 is not 1/2.");
    assert_eq!(fixed_fraction!(1, 2) + fixed_fraction!(1, 4), fixed_fraction!(3, 4), "1/2 + 1/4 is not 3/4");
}

#[test]
fn fixed_point_multiplication() {
    assert_eq!(fixed_fraction!(1, 1) * fixed_fraction!(1, 1), fixed_fraction!(1, 1), "1 * 1 is not 1.");
    assert_eq!(fixed_fraction!(1, 1) * fixed_fraction!(2, 1), fixed_fraction!(2, 1), "1 * 2 is not 2.");
    assert_eq!(fixed_fraction!(1, 1) * fixed_fraction!(1, 2), fixed_fraction!(1, 2), "1 * 1/2 is not 1/2.");
    assert_eq!(fixed_fraction!(1, 2) * fixed_fraction!(2, 1), fixed_fraction!(1, 1), "1/2 * 2 is not 1.");
    assert_eq!(fixed_fraction!(1, 2) * fixed_fraction!(3, 4), fixed_fraction!(3, 8), "1/2 * 3/4 is not 3/8.");
}


#[test]
fn fixed_point_division() {
    assert_eq!(fixed_fraction!(1, 1) / fixed_fraction!(1, 2), fixed_fraction!(2, 1), "1 / 1/2 is not 2.");
}
