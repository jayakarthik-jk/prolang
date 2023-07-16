#[cfg(test)]
use crate::common::datatypes::DataType;
#[cfg(test)]
use crate::common::operators::arithmetic::Arithmetic::Division;

#[test]
fn division_string() {
    assert_eq!(
        Division.evaluate(
            DataType::String("Hello".to_string()),
            DataType::String(" World".to_string())
        ),
        DataType::NAN
    );
}

#[test]
fn division_float() {
    assert_eq!(
        Division.evaluate(DataType::Float(5.0), DataType::Float(2.0)),
        DataType::Float(2.5)
    );
}

#[test]
fn division_integer() {
    assert_eq!(
        Division.evaluate(DataType::Integer(5), DataType::Integer(10)),
        DataType::Float(0.5)
    );
}

#[test]
fn division_boolean() {
    assert_eq!(
        Division.evaluate(DataType::Boolean(true), DataType::Boolean(false)),
        DataType::Infinity
    );
}

#[test]
fn division_nan() {
    assert_eq!(
        Division.evaluate(DataType::NAN, DataType::NAN),
        DataType::NAN
    );
}

#[test]
fn division_infinity() {
    assert_eq!(
        Division.evaluate(DataType::Infinity, DataType::Infinity),
        DataType::NAN
    );
}

#[test]
fn division_string_and_string() {
    assert_eq!(
        Division.evaluate(
            DataType::String("Hello".to_string()),
            DataType::String(" World".to_string())
        ),
        DataType::NAN
    );
}

#[test]
fn division_string_and_float() {
    assert_eq!(
        Division.evaluate(DataType::String("Hello".to_string()), DataType::Float(2.0)),
        DataType::NAN
    );
}

#[test]
fn division_string_and_integer() {
    assert_eq!(
        Division.evaluate(DataType::String("Hello".to_string()), DataType::Integer(2)),
        DataType::NAN
    );
}

#[test]
fn division_string_and_boolean() {
    assert_eq!(
        Division.evaluate(
            DataType::String("Hello".to_string()),
            DataType::Boolean(true)
        ),
        DataType::NAN
    );
}

#[test]
fn division_string_and_nan() {
    assert_eq!(
        Division.evaluate(DataType::String("Hello".to_string()), DataType::NAN),
        DataType::NAN
    );
}

#[test]
fn division_string_and_infinity() {
    assert_eq!(
        Division.evaluate(DataType::String("Hello".to_string()), DataType::Infinity),
        DataType::NAN
    );
}

#[test]
fn division_float_and_string() {
    assert_eq!(
        Division.evaluate(DataType::Float(2.0), DataType::String("Hello".to_string())),
        DataType::NAN
    );
}

#[test]
fn division_float_and_float() {
    assert_eq!(
        Division.evaluate(DataType::Float(2.0), DataType::Float(2.0)),
        DataType::Float(1.0)
    );
}

#[test]
fn division_float_and_integer() {
    assert_eq!(
        Division.evaluate(DataType::Float(2.0), DataType::Integer(2)),
        DataType::Float(1.0)
    );
}

#[test]
fn division_float_and_boolean() {
    assert_eq!(
        Division.evaluate(DataType::Float(2.0), DataType::Boolean(true)),
        DataType::Float(2.0)
    );
}

#[test]
fn division_float_and_nan() {
    assert_eq!(
        Division.evaluate(DataType::Float(2.0), DataType::NAN),
        DataType::NAN
    );
}

#[test]
fn division_float_and_infinity() {
    assert_eq!(
        Division.evaluate(DataType::Float(2.0), DataType::Infinity),
        DataType::Float(0.0)
    );
}

#[test]
fn division_integer_and_string() {
    assert_eq!(
        Division.evaluate(DataType::Integer(2), DataType::String("Hello".to_string())),
        DataType::NAN
    );
}

#[test]
fn division_integer_and_float() {
    assert_eq!(
        Division.evaluate(DataType::Integer(2), DataType::Float(2.0)),
        DataType::Float(1.0)
    );
}

#[test]
fn division_integer_and_integer() {
    assert_eq!(
        Division.evaluate(DataType::Integer(2), DataType::Integer(2)),
        DataType::Float(1.0)
    );
}

#[test]
fn division_integer_and_boolean() {
    assert_eq!(
        Division.evaluate(DataType::Integer(2), DataType::Boolean(true)),
        DataType::Float(2.0)
    );
}

#[test]
fn division_integer_and_nan() {
    assert_eq!(
        Division.evaluate(DataType::Integer(2), DataType::NAN),
        DataType::NAN
    );
}

#[test]
fn division_integer_and_infinity() {
    assert_eq!(
        Division.evaluate(DataType::Integer(2), DataType::Infinity),
        DataType::Float(0.0)
    );
}

#[test]
fn division_boolean_and_string() {
    assert_eq!(
        Division.evaluate(
            DataType::Boolean(true),
            DataType::String("Hello".to_string())
        ),
        DataType::NAN
    );
}

#[test]
fn division_boolean_and_float() {
    assert_eq!(
        Division.evaluate(DataType::Boolean(true), DataType::Float(2.0)),
        DataType::Float(0.5)
    );
}

#[test]
fn division_boolean_and_integer() {
    assert_eq!(
        Division.evaluate(DataType::Boolean(true), DataType::Integer(2)),
        DataType::Float(0.5)
    );
}

#[test]
fn division_boolean_and_boolean() {
    assert_eq!(
        Division.evaluate(DataType::Boolean(true), DataType::Boolean(true)),
        DataType::Float(1.0)
    );
}

#[test]
fn division_boolean_and_nan() {
    assert_eq!(
        Division.evaluate(DataType::Boolean(true), DataType::NAN),
        DataType::NAN
    );
}

#[test]
fn division_boolean_and_infinity() {
    assert_eq!(
        Division.evaluate(DataType::Boolean(true), DataType::Infinity),
        DataType::Float(0.0)
    );
}

#[test]
fn division_nan_and_string() {
    assert_eq!(
        Division.evaluate(DataType::NAN, DataType::String("Hello".to_string())),
        DataType::NAN
    );
}

#[test]
fn division_nan_and_float() {
    assert_eq!(
        Division.evaluate(DataType::NAN, DataType::Float(2.0)),
        DataType::NAN
    );
}

#[test]
fn division_nan_and_integer() {
    assert_eq!(
        Division.evaluate(DataType::NAN, DataType::Integer(2)),
        DataType::NAN
    );
}

#[test]
fn division_nan_and_boolean() {
    assert_eq!(
        Division.evaluate(DataType::NAN, DataType::Boolean(true)),
        DataType::NAN
    );
}

#[test]
fn division_nan_and_nan() {
    assert_eq!(
        Division.evaluate(DataType::NAN, DataType::NAN),
        DataType::NAN
    );
}

#[test]
fn division_nan_and_infinity() {
    assert_eq!(
        Division.evaluate(DataType::NAN, DataType::Infinity),
        DataType::NAN
    );
}

#[test]
fn division_infinity_and_string() {
    assert_eq!(
        Division.evaluate(DataType::Infinity, DataType::String("Hello".to_string())),
        DataType::NAN
    );
}

#[test]
fn division_infinity_and_float() {
    assert_eq!(
        Division.evaluate(DataType::Infinity, DataType::Float(2.0)),
        DataType::Infinity
    );
}

#[test]
fn division_infinity_and_integer() {
    assert_eq!(
        Division.evaluate(DataType::Infinity, DataType::Integer(2)),
        DataType::Infinity
    );
}

#[test]
fn division_infinity_and_boolean() {
    assert_eq!(
        Division.evaluate(DataType::Infinity, DataType::Boolean(true)),
        DataType::Infinity
    );
}

#[test]
fn division_infinity_and_nan() {
    assert_eq!(
        Division.evaluate(DataType::Infinity, DataType::NAN),
        DataType::NAN
    );
}

#[test]
fn division_infinity_and_infinity() {
    assert_eq!(
        Division.evaluate(DataType::Infinity, DataType::Infinity),
        DataType::NAN
    );
}
