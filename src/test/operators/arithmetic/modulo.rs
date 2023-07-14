#[cfg(test)]
use crate::datatypes::DataType;
#[cfg(test)]
use crate::operators::arithmetic::Arithmetic::Modulo;

#[test]
fn modulo_string() {
    assert_eq!(
        Modulo.evaluate(
            DataType::String("Hello".to_string()),
            DataType::String(" World".to_string())
        ),
        DataType::NAN
    );
}

#[test]
fn modulo_float() {
    assert_eq!(
        Modulo.evaluate(DataType::Float(5.0), DataType::Float(2.0)),
        DataType::Float(1.0)
    );
}

#[test]
fn modulo_integer() {
    assert_eq!(
        Modulo.evaluate(DataType::Integer(5), DataType::Integer(10)),
        DataType::Integer(5)
    );
}

#[test]
fn modulo_boolean() {
    assert_eq!(
        Modulo.evaluate(DataType::Boolean(true), DataType::Boolean(false)),
        DataType::Infinity
    );
}

#[test]
fn modulo_nan() {
    assert_eq!(Modulo.evaluate(DataType::NAN, DataType::NAN), DataType::NAN);
}

#[test]
fn modulo_infinity() {
    assert_eq!(
        Modulo.evaluate(DataType::Infinity, DataType::Infinity),
        DataType::NAN
    );
}

#[test]
fn modulo_string_and_string() {
    assert_eq!(
        Modulo.evaluate(
            DataType::String("Hello".to_string()),
            DataType::String(" World".to_string())
        ),
        DataType::NAN
    );
}

#[test]
fn modulo_string_and_float() {
    assert_eq!(
        Modulo.evaluate(DataType::String("Hello".to_string()), DataType::Float(2.0)),
        DataType::NAN
    );
}

#[test]
fn modulo_string_and_integer() {
    assert_eq!(
        Modulo.evaluate(DataType::String("Hello".to_string()), DataType::Integer(2)),
        DataType::NAN
    );
}

#[test]
fn modulo_string_and_boolean() {
    assert_eq!(
        Modulo.evaluate(
            DataType::String("Hello".to_string()),
            DataType::Boolean(true)
        ),
        DataType::NAN
    );
}

#[test]
fn modulo_string_and_nan() {
    assert_eq!(
        Modulo.evaluate(DataType::String("Hello".to_string()), DataType::NAN),
        DataType::NAN
    );
}

#[test]
fn modulo_string_and_infinity() {
    assert_eq!(
        Modulo.evaluate(DataType::String("Hello".to_string()), DataType::Infinity),
        DataType::NAN
    );
}

#[test]
fn modulo_float_and_string() {
    assert_eq!(
        Modulo.evaluate(DataType::Float(2.0), DataType::String("Hello".to_string())),
        DataType::NAN
    );
}

#[test]
fn modulo_float_and_float() {
    assert_eq!(
        Modulo.evaluate(DataType::Float(2.0), DataType::Float(2.0)),
        DataType::Float(0.0)
    );
}

#[test]
fn modulo_float_and_integer() {
    assert_eq!(
        Modulo.evaluate(DataType::Float(2.0), DataType::Integer(2)),
        DataType::Float(0.0)
    );
}

#[test]
fn modulo_float_and_boolean() {
    assert_eq!(
        Modulo.evaluate(DataType::Float(2.0), DataType::Boolean(true)),
        DataType::Float(0.0)
    );
}

#[test]
fn modulo_float_and_nan() {
    assert_eq!(
        Modulo.evaluate(DataType::Float(2.0), DataType::NAN),
        DataType::NAN
    );
}

#[test]
fn modulo_float_and_infinity() {
    assert_eq!(
        Modulo.evaluate(DataType::Float(2.0), DataType::Infinity),
        DataType::Float(2.0)
    );
}

#[test]
fn modulo_integer_and_string() {
    assert_eq!(
        Modulo.evaluate(DataType::Integer(2), DataType::String("Hello".to_string())),
        DataType::NAN
    );
}

#[test]
fn modulo_integer_and_float() {
    assert_eq!(
        Modulo.evaluate(DataType::Integer(2), DataType::Float(2.0)),
        DataType::Float(0.0)
    );
}

#[test]
fn modulo_integer_and_integer() {
    assert_eq!(
        Modulo.evaluate(DataType::Integer(2), DataType::Integer(2)),
        DataType::Integer(0)
    );
}

#[test]
fn modulo_integer_and_boolean() {
    assert_eq!(
        Modulo.evaluate(DataType::Integer(2), DataType::Boolean(true)),
        DataType::Integer(0)
    );
}

#[test]
fn modulo_integer_and_nan() {
    assert_eq!(
        Modulo.evaluate(DataType::Integer(2), DataType::NAN),
        DataType::NAN
    );
}

#[test]
fn modulo_integer_and_infinity() {
    assert_eq!(
        Modulo.evaluate(DataType::Integer(2), DataType::Infinity),
        DataType::Integer(2)
    );
}

#[test]
fn modulo_boolean_and_string() {
    assert_eq!(
        Modulo.evaluate(
            DataType::Boolean(true),
            DataType::String("Hello".to_string())
        ),
        DataType::NAN
    );
}

#[test]
fn modulo_boolean_and_float() {
    assert_eq!(
        Modulo.evaluate(DataType::Boolean(true), DataType::Float(2.0)),
        DataType::Float(1.0)
    );
}

#[test]
fn modulo_boolean_and_integer() {
    assert_eq!(
        Modulo.evaluate(DataType::Boolean(true), DataType::Integer(2)),
        DataType::Integer(1)
    );
}

#[test]
fn modulo_boolean_and_boolean() {
    assert_eq!(
        Modulo.evaluate(DataType::Boolean(true), DataType::Boolean(true)),
        DataType::Integer(0)
    );
}

#[test]
fn modulo_boolean_and_nan() {
    assert_eq!(
        Modulo.evaluate(DataType::Boolean(true), DataType::NAN),
        DataType::NAN
    );
}

#[test]
fn modulo_boolean_and_infinity() {
    assert_eq!(
        Modulo.evaluate(DataType::Boolean(true), DataType::Infinity),
        DataType::Integer(0)
    );
}

#[test]
fn modulo_nan_and_string() {
    assert_eq!(
        Modulo.evaluate(DataType::NAN, DataType::String("Hello".to_string())),
        DataType::NAN
    );
}

#[test]
fn modulo_nan_and_float() {
    assert_eq!(
        Modulo.evaluate(DataType::NAN, DataType::Float(2.0)),
        DataType::NAN
    );
}

#[test]
fn modulo_nan_and_integer() {
    assert_eq!(
        Modulo.evaluate(DataType::NAN, DataType::Integer(2)),
        DataType::NAN
    );
}

#[test]
fn modulo_nan_and_boolean() {
    assert_eq!(
        Modulo.evaluate(DataType::NAN, DataType::Boolean(true)),
        DataType::NAN
    );
}

#[test]
fn modulo_nan_and_nan() {
    assert_eq!(Modulo.evaluate(DataType::NAN, DataType::NAN), DataType::NAN);
}

#[test]
fn modulo_nan_and_infinity() {
    assert_eq!(
        Modulo.evaluate(DataType::NAN, DataType::Infinity),
        DataType::NAN
    );
}

#[test]
fn modulo_infinity_and_string() {
    assert_eq!(
        Modulo.evaluate(DataType::Infinity, DataType::String("Hello".to_string())),
        DataType::NAN
    );
}

#[test]
fn modulo_infinity_and_float() {
    assert_eq!(
        Modulo.evaluate(DataType::Infinity, DataType::Float(2.0)),
        DataType::NAN
    );
}

#[test]
fn modulo_infinity_and_integer() {
    assert_eq!(
        Modulo.evaluate(DataType::Infinity, DataType::Integer(2)),
        DataType::NAN
    );
}

#[test]
fn modulo_infinity_and_boolean() {
    assert_eq!(
        Modulo.evaluate(DataType::Infinity, DataType::Boolean(true)),
        DataType::NAN
    );
}

#[test]
fn modulo_infinity_and_nan() {
    assert_eq!(
        Modulo.evaluate(DataType::Infinity, DataType::NAN),
        DataType::NAN
    );
}

#[test]
fn modulo_infinity_and_infinity() {
    assert_eq!(
        Modulo.evaluate(DataType::Infinity, DataType::Infinity),
        DataType::NAN
    );
}
