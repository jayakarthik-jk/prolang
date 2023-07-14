#[cfg(test)]
use crate::datatypes::DataType;
#[cfg(test)]
use crate::operators::arithmetic::Arithmetic::Subtraction;

#[test]
fn subtraction_string() {
    assert_eq!(
        Subtraction.evaluate(
            DataType::String("Hello".to_string()),
            DataType::String(" World".to_string())
        ),
        DataType::NAN
    );
}

#[test]
fn subtraction_float() {
    assert_eq!(
        Subtraction.evaluate(DataType::Float(5.0), DataType::Float(2.0)),
        DataType::Float(3.0)
    );
}

#[test]
fn subtraction_integer() {
    assert_eq!(
        Subtraction.evaluate(DataType::Integer(5), DataType::Integer(10)),
        DataType::Integer(-5)
    );
}

#[test]
fn subtraction_boolean() {
    assert_eq!(
        Subtraction.evaluate(DataType::Boolean(true), DataType::Boolean(false)),
        DataType::Integer(1)
    );
}

#[test]
fn subtraction_nan() {
    assert_eq!(
        Subtraction.evaluate(DataType::NAN, DataType::NAN),
        DataType::NAN
    );
}

#[test]
fn subtraction_infinity() {
    assert_eq!(
        Subtraction.evaluate(DataType::Infinity, DataType::Infinity),
        DataType::Infinity
    );
}

#[test]
fn subtraction_string_and_string() {
    assert_eq!(
        Subtraction.evaluate(
            DataType::String("Hello".to_string()),
            DataType::String(" World".to_string())
        ),
        DataType::NAN
    );
}

#[test]
fn subtraction_string_and_float() {
    assert_eq!(
        Subtraction.evaluate(DataType::String("Hello".to_string()), DataType::Float(2.0)),
        DataType::NAN
    );
}

#[test]
fn subtraction_string_and_integer() {
    assert_eq!(
        Subtraction.evaluate(DataType::String("Hello".to_string()), DataType::Integer(2)),
        DataType::NAN
    );
}

#[test]
fn subtraction_string_and_boolean() {
    assert_eq!(
        Subtraction.evaluate(
            DataType::String("Hello".to_string()),
            DataType::Boolean(true)
        ),
        DataType::NAN
    );
}

#[test]
fn subtraction_string_and_nan() {
    assert_eq!(
        Subtraction.evaluate(DataType::String("Hello".to_string()), DataType::NAN),
        DataType::NAN
    );
}

#[test]
fn subtraction_string_and_infinity() {
    assert_eq!(
        Subtraction.evaluate(DataType::String("Hello".to_string()), DataType::Infinity),
        DataType::NAN
    );
}

#[test]
fn subtraction_float_and_string() {
    assert_eq!(
        Subtraction.evaluate(DataType::Float(2.0), DataType::String("Hello".to_string())),
        DataType::NAN
    );
}

#[test]
fn subtraction_float_and_float() {
    assert_eq!(
        Subtraction.evaluate(DataType::Float(2.0), DataType::Float(2.0)),
        DataType::Float(0.0)
    );
}

#[test]
fn subtraction_float_and_integer() {
    assert_eq!(
        Subtraction.evaluate(DataType::Float(2.0), DataType::Integer(2)),
        DataType::Float(0.0)
    );
}

#[test]
fn subtraction_float_and_boolean() {
    assert_eq!(
        Subtraction.evaluate(DataType::Float(2.0), DataType::Boolean(true)),
        DataType::Float(1.0)
    );
}

#[test]
fn subtraction_float_and_nan() {
    assert_eq!(
        Subtraction.evaluate(DataType::Float(2.0), DataType::NAN),
        DataType::NAN
    );
}

#[test]
fn subtraction_float_and_infinity() {
    assert_eq!(
        Subtraction.evaluate(DataType::Float(2.0), DataType::Infinity),
        DataType::Infinity
    );
}

#[test]
fn subtraction_integer_and_string() {
    assert_eq!(
        Subtraction.evaluate(DataType::Integer(2), DataType::String("Hello".to_string())),
        DataType::NAN
    );
}

#[test]
fn subtraction_integer_and_float() {
    assert_eq!(
        Subtraction.evaluate(DataType::Integer(2), DataType::Float(2.0)),
        DataType::Float(0.0)
    );
}

#[test]
fn subtraction_integer_and_integer() {
    assert_eq!(
        Subtraction.evaluate(DataType::Integer(2), DataType::Integer(2)),
        DataType::Integer(0)
    );
}

#[test]
fn subtraction_integer_and_boolean() {
    assert_eq!(
        Subtraction.evaluate(DataType::Integer(2), DataType::Boolean(true)),
        DataType::Integer(1)
    );
}

#[test]
fn subtraction_integer_and_nan() {
    assert_eq!(
        Subtraction.evaluate(DataType::Integer(2), DataType::NAN),
        DataType::NAN
    );
}

#[test]
fn subtraction_integer_and_infinity() {
    assert_eq!(
        Subtraction.evaluate(DataType::Integer(2), DataType::Infinity),
        DataType::Infinity
    );
}

#[test]
fn subtraction_boolean_and_string() {
    assert_eq!(
        Subtraction.evaluate(
            DataType::Boolean(true),
            DataType::String("Hello".to_string())
        ),
        DataType::NAN
    );
}

#[test]
fn subtraction_boolean_and_float() {
    assert_eq!(
        Subtraction.evaluate(DataType::Boolean(true), DataType::Float(2.0)),
        DataType::Float(1.0)
    );
}

#[test]
fn subtraction_boolean_and_integer() {
    assert_eq!(
        Subtraction.evaluate(DataType::Boolean(true), DataType::Integer(2)),
        DataType::Integer(1)
    );
}

#[test]
fn subtraction_boolean_and_boolean() {
    assert_eq!(
        Subtraction.evaluate(DataType::Boolean(true), DataType::Boolean(true)),
        DataType::Integer(0)
    );
}

#[test]
fn subtraction_boolean_and_nan() {
    assert_eq!(
        Subtraction.evaluate(DataType::Boolean(true), DataType::NAN),
        DataType::NAN
    );
}

#[test]
fn subtraction_boolean_and_infinity() {
    assert_eq!(
        Subtraction.evaluate(DataType::Boolean(true), DataType::Infinity),
        DataType::Infinity
    );
}

#[test]
fn subtraction_nan_and_string() {
    assert_eq!(
        Subtraction.evaluate(DataType::NAN, DataType::String("Hello".to_string())),
        DataType::NAN
    );
}

#[test]
fn subtraction_nan_and_float() {
    assert_eq!(
        Subtraction.evaluate(DataType::NAN, DataType::Float(2.0)),
        DataType::NAN
    );
}

#[test]
fn subtraction_nan_and_integer() {
    assert_eq!(
        Subtraction.evaluate(DataType::NAN, DataType::Integer(2)),
        DataType::NAN
    );
}

#[test]
fn subtraction_nan_and_boolean() {
    assert_eq!(
        Subtraction.evaluate(DataType::NAN, DataType::Boolean(true)),
        DataType::NAN
    );
}

#[test]
fn subtraction_nan_and_nan() {
    assert_eq!(
        Subtraction.evaluate(DataType::NAN, DataType::NAN),
        DataType::NAN
    );
}

#[test]
fn subtraction_nan_and_infinity() {
    assert_eq!(
        Subtraction.evaluate(DataType::NAN, DataType::Infinity),
        DataType::NAN
    );
}

#[test]
fn subtraction_infinity_and_string() {
    assert_eq!(
        Subtraction.evaluate(DataType::Infinity, DataType::String("Hello".to_string())),
        DataType::NAN
    );
}

#[test]
fn subtraction_infinity_and_float() {
    assert_eq!(
        Subtraction.evaluate(DataType::Infinity, DataType::Float(2.0)),
        DataType::Infinity
    );
}

#[test]
fn subtraction_infinity_and_integer() {
    assert_eq!(
        Subtraction.evaluate(DataType::Infinity, DataType::Integer(2)),
        DataType::Infinity
    );
}

#[test]
fn subtraction_infinity_and_boolean() {
    assert_eq!(
        Subtraction.evaluate(DataType::Infinity, DataType::Boolean(true)),
        DataType::Infinity
    );
}

#[test]
fn subtraction_infinity_and_nan() {
    assert_eq!(
        Subtraction.evaluate(DataType::Infinity, DataType::NAN),
        DataType::NAN
    );
}

#[test]
fn subtraction_infinity_and_infinity() {
    assert_eq!(
        Subtraction.evaluate(DataType::Infinity, DataType::Infinity),
        DataType::Infinity
    );
}
