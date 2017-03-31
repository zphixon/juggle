
extern crate juggle;

use juggle::*;

#[test]
fn val_eq() {
    assert!(Value::Number(3) == Value::Number(3));
}

#[test]
#[should_panic]
fn val_not_eq() {
    assert!(Value::Number(3) == Value::Number(2));
}

#[test]
#[should_panic]
fn val_types_not_eq() {
    assert!(Value::Number(3) == Value::Bool(false));
}

#[test]
fn val_num_gt() {
    assert!(Value::Number(3) > Value::Number(2));
}

#[test]
fn val_num_le() {
    assert!(Value::Number(2) <= Value::Number(3));
}

#[test]
fn val_arr_lt() {
    assert!(Value::Array(vec![]) < Value::Array(vec![Value::Number(1)]));
}

