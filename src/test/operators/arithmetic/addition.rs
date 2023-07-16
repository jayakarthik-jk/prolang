#[cfg(test)]
use crate::common::datatypes::DataType;
#[cfg(test)]
use crate::common::operators::arithmetic::Arithmetic::Addition;
#[test]
fn addition_string() {
    assert_eq!(
        Addition.evaluate(
            DataType::String("Hello".to_string()),
            DataType::String(" World".to_string())
        ),
        DataType::String("Hello World".to_string())
    );
}

#[test]
fn addition_float() {
    assert_eq!(
        Addition.evaluate(DataType::Float(3.7), DataType::Float(2.5)),
        DataType::Float(6.2)
    );
}

#[test]
fn addition_integer() {
    assert_eq!(
        Addition.evaluate(DataType::Integer(5), DataType::Integer(10)),
        DataType::Integer(15)
    );
}

#[test]
fn addition_boolean() {
    assert_eq!(
        Addition.evaluate(DataType::Boolean(true), DataType::Boolean(false)),
        DataType::Integer(1)
    );
}

#[test]
fn addition_nan() {
    assert_eq!(
        Addition.evaluate(DataType::NAN, DataType::NAN),
        DataType::NAN
    );
}

#[test]
fn addition_infinity() {
    assert_eq!(
        Addition.evaluate(DataType::Infinity, DataType::Infinity),
        DataType::Infinity
    );
}

#[test]
fn addition_string_and_string() {
    assert_eq!(
        Addition.evaluate(
            DataType::String("Hello".to_string()),
            DataType::String(" World".to_string())
        ),
        DataType::String("Hello World".to_string())
    );
}

#[test]
fn addition_string_and_float() {
    assert_eq!(
        Addition.evaluate(DataType::String("Hello".to_string()), DataType::Float(2.0)),
        DataType::String("Hello2".to_string())
    );
}

#[test]
fn addition_string_and_integer() {
    assert_eq!(
        Addition.evaluate(DataType::String("Hello".to_string()), DataType::Integer(2)),
        DataType::String("Hello2".to_string())
    );
}

#[test]
fn addition_string_and_boolean() {
    assert_eq!(
        Addition.evaluate(
            DataType::String("Hello".to_string()),
            DataType::Boolean(true)
        ),
        DataType::String("Hellotrue".to_string())
    );
}

#[test]
fn addition_string_and_nan() {
    assert_eq!(
        Addition.evaluate(DataType::String("Hello".to_string()), DataType::NAN),
        DataType::String("HelloNAN".to_string())
    );
}

#[test]
fn addition_string_and_infinity() {
    assert_eq!(
        Addition.evaluate(DataType::String("Hello".to_string()), DataType::Infinity),
        DataType::String("HelloInfinity".to_string())
    );
}

#[test]
fn addition_float_and_string() {
    assert_eq!(
        Addition.evaluate(DataType::Float(2.0), DataType::String("Hello".to_string())),
        DataType::String("2Hello".to_string())
    );
}

#[test]
fn addition_float_and_float() {
    assert_eq!(
        Addition.evaluate(DataType::Float(2.0), DataType::Float(2.0)),
        DataType::Float(4.0)
    );
}

#[test]
fn addition_float_and_integer() {
    assert_eq!(
        Addition.evaluate(DataType::Float(2.0), DataType::Integer(2)),
        DataType::Float(4.0)
    );
}

#[test]
fn addition_float_and_boolean() {
    assert_eq!(
        Addition.evaluate(DataType::Float(2.0), DataType::Boolean(true)),
        DataType::Float(3.0)
    );
}

#[test]
fn addition_float_and_nan() {
    assert_eq!(
        Addition.evaluate(DataType::Float(2.0), DataType::NAN),
        DataType::NAN
    );
}

#[test]
fn addition_float_and_infinity() {
    assert_eq!(
        Addition.evaluate(DataType::Float(2.0), DataType::Infinity),
        DataType::Infinity
    );
}

#[test]
fn addition_integer_and_string() {
    assert_eq!(
        Addition.evaluate(DataType::Integer(2), DataType::String("Hello".to_string())),
        DataType::String("2Hello".to_string())
    );
}

#[test]
fn addition_integer_and_float() {
    assert_eq!(
        Addition.evaluate(DataType::Integer(2), DataType::Float(2.0)),
        DataType::Float(4.0)
    );
}

#[test]
fn addition_integer_and_integer() {
    assert_eq!(
        Addition.evaluate(DataType::Integer(2), DataType::Integer(2)),
        DataType::Integer(4)
    );
}

#[test]
fn addition_integer_and_boolean() {
    assert_eq!(
        Addition.evaluate(DataType::Integer(2), DataType::Boolean(true)),
        DataType::Integer(3)
    );
}

#[test]
fn addition_integer_and_nan() {
    assert_eq!(
        Addition.evaluate(DataType::Integer(2), DataType::NAN),
        DataType::NAN
    );
}

#[test]
fn addition_integer_and_infinity() {
    assert_eq!(
        Addition.evaluate(DataType::Integer(2), DataType::Infinity),
        DataType::Infinity
    );
}

#[test]
fn addition_boolean_and_string() {
    assert_eq!(
        Addition.evaluate(
            DataType::Boolean(true),
            DataType::String("Hello".to_string())
        ),
        DataType::String("trueHello".to_string())
    );
}

#[test]
fn addition_boolean_and_float() {
    assert_eq!(
        Addition.evaluate(DataType::Boolean(true), DataType::Float(2.0)),
        DataType::Float(3.0)
    );
}

#[test]
fn addition_boolean_and_integer() {
    assert_eq!(
        Addition.evaluate(DataType::Boolean(true), DataType::Integer(2)),
        DataType::Integer(3)
    );
}

#[test]
fn addition_boolean_and_boolean() {
    assert_eq!(
        Addition.evaluate(DataType::Boolean(true), DataType::Boolean(true)),
        DataType::Integer(2)
    );
}

#[test]
fn addition_boolean_and_nan() {
    assert_eq!(
        Addition.evaluate(DataType::Boolean(true), DataType::NAN),
        DataType::NAN
    );
}

#[test]
fn addition_boolean_and_infinity() {
    assert_eq!(
        Addition.evaluate(DataType::Boolean(true), DataType::Infinity),
        DataType::Infinity
    );
}

#[test]
fn addition_nan_and_string() {
    assert_eq!(
        Addition.evaluate(DataType::NAN, DataType::String("Hello".to_string())),
        DataType::String("NANHello".to_string())
    );
}

#[test]
fn addition_nan_and_float() {
    assert_eq!(
        Addition.evaluate(DataType::NAN, DataType::Float(2.0)),
        DataType::NAN
    );
}

#[test]
fn addition_nan_and_integer() {
    assert_eq!(
        Addition.evaluate(DataType::NAN, DataType::Integer(2)),
        DataType::NAN
    );
}

#[test]
fn addition_nan_and_boolean() {
    assert_eq!(
        Addition.evaluate(DataType::NAN, DataType::Boolean(true)),
        DataType::NAN
    );
}

#[test]
fn addition_nan_and_nan() {
    assert_eq!(
        Addition.evaluate(DataType::NAN, DataType::NAN),
        DataType::NAN
    );
}

#[test]
fn addition_nan_and_infinity() {
    assert_eq!(
        Addition.evaluate(DataType::NAN, DataType::Infinity),
        DataType::NAN
    );
}

#[test]
fn addition_infinity_and_string() {
    assert_eq!(
        Addition.evaluate(DataType::Infinity, DataType::String("Hello".to_string())),
        DataType::String("InfinityHello".to_string())
    );
}

#[test]
fn addition_infinity_and_float() {
    assert_eq!(
        Addition.evaluate(DataType::Infinity, DataType::Float(2.0)),
        DataType::Infinity
    );
}

#[test]
fn addition_infinity_and_integer() {
    assert_eq!(
        Addition.evaluate(DataType::Infinity, DataType::Integer(2)),
        DataType::Infinity
    );
}

#[test]
fn addition_infinity_and_boolean() {
    assert_eq!(
        Addition.evaluate(DataType::Infinity, DataType::Boolean(true)),
        DataType::Infinity
    );
}

#[test]
fn addition_infinity_and_nan() {
    assert_eq!(
        Addition.evaluate(DataType::Infinity, DataType::NAN),
        DataType::NAN
    );
}

#[test]
fn addition_infinity_and_infinity() {
    assert_eq!(
        Addition.evaluate(DataType::Infinity, DataType::Infinity),
        DataType::Infinity
    );
}
