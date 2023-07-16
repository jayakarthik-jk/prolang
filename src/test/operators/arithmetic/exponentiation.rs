#[cfg(test)]
use crate::common::datatypes::DataType;
#[cfg(test)]
use crate::common::operators::arithmetic::Arithmetic::Exponentiation;

#[test]
fn exponentiation_string() {
    assert_eq!(
        Exponentiation.evaluate(
            DataType::String("Hello".to_string()),
            DataType::String(" World".to_string())
        ),
        DataType::NAN
    );
}

#[test]
fn exponentiation_float() {
    assert_eq!(
        Exponentiation.evaluate(DataType::Float(5.0), DataType::Float(2.0)),
        DataType::Float(25.0)
    );
}

#[test]
fn exponentiation_integer() {
    assert_eq!(
        Exponentiation.evaluate(DataType::Integer(5), DataType::Integer(10)),
        DataType::Integer(9765625)
    );
}

#[test]
fn exponentiation_boolean() {
    assert_eq!(
        Exponentiation.evaluate(DataType::Boolean(true), DataType::Boolean(false)),
        DataType::Integer(1)
    );
}

#[test]
fn exponentiation_nan() {
    assert_eq!(
        Exponentiation.evaluate(DataType::NAN, DataType::NAN),
        DataType::NAN
    );
}

#[test]
fn exponentiation_infinity() {
    assert_eq!(
        Exponentiation.evaluate(DataType::Infinity, DataType::Infinity),
        DataType::NAN
    );
}

#[test]
fn exponentiation_string_and_string() {
    assert_eq!(
        Exponentiation.evaluate(
            DataType::String("Hello".to_string()),
            DataType::String(" World".to_string())
        ),
        DataType::NAN
    );
}

#[test]
fn exponentiation_string_and_float() {
    assert_eq!(
        Exponentiation.evaluate(DataType::String("Hello".to_string()), DataType::Float(2.0)),
        DataType::NAN
    );
}

#[test]
fn exponentiation_string_and_integer() {
    assert_eq!(
        Exponentiation.evaluate(DataType::String("Hello".to_string()), DataType::Integer(2)),
        DataType::NAN
    );
}

#[test]
fn exponentiation_string_and_boolean() {
    assert_eq!(
        Exponentiation.evaluate(
            DataType::String("Hello".to_string()),
            DataType::Boolean(true)
        ),
        DataType::NAN
    );
}

#[test]
fn exponentiation_string_and_nan() {
    assert_eq!(
        Exponentiation.evaluate(DataType::String("Hello".to_string()), DataType::NAN),
        DataType::NAN
    );
}

#[test]
fn exponentiation_string_and_infinity() {
    assert_eq!(
        Exponentiation.evaluate(DataType::String("Hello".to_string()), DataType::Infinity),
        DataType::NAN
    );
}

#[test]
fn exponentiation_float_and_string() {
    assert_eq!(
        Exponentiation.evaluate(DataType::Float(2.0), DataType::String("Hello".to_string())),
        DataType::NAN
    );
}

#[test]
fn exponentiation_float_and_float() {
    assert_eq!(
        Exponentiation.evaluate(DataType::Float(2.0), DataType::Float(2.0)),
        DataType::Float(4.0)
    );
}

#[test]
fn exponentiation_float_and_integer() {
    assert_eq!(
        Exponentiation.evaluate(DataType::Float(2.0), DataType::Integer(2)),
        DataType::Float(4.0)
    );
}

#[test]
fn exponentiation_float_and_boolean() {
    assert_eq!(
        Exponentiation.evaluate(DataType::Float(2.0), DataType::Boolean(true)),
        DataType::Float(2.0)
    );
}

#[test]
fn exponentiation_float_and_nan() {
    assert_eq!(
        Exponentiation.evaluate(DataType::Float(2.0), DataType::NAN),
        DataType::NAN
    );
}

#[test]
fn exponentiation_float_and_infinity() {
    assert_eq!(
        Exponentiation.evaluate(DataType::Float(2.0), DataType::Infinity),
        DataType::NAN
    );
}

#[test]
fn exponentiation_integer_and_string() {
    assert_eq!(
        Exponentiation.evaluate(DataType::Integer(2), DataType::String("Hello".to_string())),
        DataType::NAN
    );
}

#[test]
fn exponentiation_integer_and_float() {
    assert_eq!(
        Exponentiation.evaluate(DataType::Integer(2), DataType::Float(2.0)),
        DataType::Float(4.0)
    );
}

#[test]
fn exponentiation_integer_and_integer() {
    assert_eq!(
        Exponentiation.evaluate(DataType::Integer(2), DataType::Integer(2)),
        DataType::Integer(4)
    );
}

#[test]
fn exponentiation_integer_and_boolean() {
    assert_eq!(
        Exponentiation.evaluate(DataType::Integer(2), DataType::Boolean(true)),
        DataType::Integer(2)
    );
}

#[test]
fn exponentiation_integer_and_nan() {
    assert_eq!(
        Exponentiation.evaluate(DataType::Integer(2), DataType::NAN),
        DataType::NAN
    );
}

#[test]
fn exponentiation_integer_and_infinity() {
    assert_eq!(
        Exponentiation.evaluate(DataType::Integer(2), DataType::Infinity),
        DataType::NAN
    );
}

#[test]
fn exponentiation_boolean_and_string() {
    assert_eq!(
        Exponentiation.evaluate(
            DataType::Boolean(true),
            DataType::String("Hello".to_string())
        ),
        DataType::NAN
    );
}

#[test]
fn exponentiation_boolean_and_float() {
    assert_eq!(
        Exponentiation.evaluate(DataType::Boolean(true), DataType::Float(2.0)),
        DataType::Float(1.0)
    );
}

#[test]
fn exponentiation_boolean_and_integer() {
    assert_eq!(
        Exponentiation.evaluate(DataType::Boolean(true), DataType::Integer(2)),
        DataType::Integer(1)
    );
}

#[test]
fn exponentiation_boolean_and_boolean() {
    assert_eq!(
        Exponentiation.evaluate(DataType::Boolean(true), DataType::Boolean(true)),
        DataType::Integer(1)
    );
}

#[test]
fn exponentiation_boolean_and_nan() {
    assert_eq!(
        Exponentiation.evaluate(DataType::Boolean(true), DataType::NAN),
        DataType::NAN
    );
}

#[test]
fn exponentiation_boolean_and_infinity() {
    assert_eq!(
        Exponentiation.evaluate(DataType::Boolean(true), DataType::Infinity),
        DataType::NAN
    );
}

#[test]
fn exponentiation_nan_and_string() {
    assert_eq!(
        Exponentiation.evaluate(DataType::NAN, DataType::String("Hello".to_string())),
        DataType::NAN
    );
}

#[test]
fn exponentiation_nan_and_float() {
    assert_eq!(
        Exponentiation.evaluate(DataType::NAN, DataType::Float(2.0)),
        DataType::NAN
    );
}

#[test]
fn exponentiation_nan_and_integer() {
    assert_eq!(
        Exponentiation.evaluate(DataType::NAN, DataType::Integer(2)),
        DataType::NAN
    );
}

#[test]
fn exponentiation_nan_and_boolean() {
    assert_eq!(
        Exponentiation.evaluate(DataType::NAN, DataType::Boolean(true)),
        DataType::NAN
    );
}

#[test]
fn exponentiation_nan_and_nan() {
    assert_eq!(
        Exponentiation.evaluate(DataType::NAN, DataType::NAN),
        DataType::NAN
    );
}

#[test]
fn exponentiation_nan_and_infinity() {
    assert_eq!(
        Exponentiation.evaluate(DataType::NAN, DataType::Infinity),
        DataType::NAN
    );
}

#[test]
fn exponentiation_infinity_and_string() {
    assert_eq!(
        Exponentiation.evaluate(DataType::Infinity, DataType::String("Hello".to_string())),
        DataType::NAN
    );
}

#[test]
fn exponentiation_infinity_and_float() {
    assert_eq!(
        Exponentiation.evaluate(DataType::Infinity, DataType::Float(2.0)),
        DataType::Infinity
    );
}

#[test]
fn exponentiation_infinity_and_integer() {
    assert_eq!(
        Exponentiation.evaluate(DataType::Infinity, DataType::Integer(2)),
        DataType::Infinity
    );
}

#[test]
fn exponentiation_infinity_and_boolean() {
    assert_eq!(
        Exponentiation.evaluate(DataType::Infinity, DataType::Boolean(true)),
        DataType::Infinity
    );
}

#[test]
fn exponentiation_infinity_and_nan() {
    assert_eq!(
        Exponentiation.evaluate(DataType::Infinity, DataType::NAN),
        DataType::NAN
    );
}

#[test]
fn exponentiation_infinity_and_infinity() {
    assert_eq!(
        Exponentiation.evaluate(DataType::Infinity, DataType::Infinity),
        DataType::NAN
    );
}
