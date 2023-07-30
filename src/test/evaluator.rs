// #[cfg(test)]

// /// only for testing purposes
// fn get_evaluator(source: String) -> crate::common::datatypes::Variable {
//     use crate::common::datatypes::{Variable, DataType};

//     let mut lexer = crate::lexical_analysis::lexer::Lexer::new(source);
//     lexer.lex().unwrap();
//     let mut parser = crate::syntax_analysis::parser::Parser::new(lexer);
//     let asts = parser.parse().unwrap();
//     let mut result: Variable = Variable::from(DataType::InternalUndefined);
//     println!("asts: {:?}", asts);
//     for ast in asts {
//         let binder = crate::semantic_analysis::binder::Binder::new(ast);
//         let semantic_tree = binder.bind().unwrap();
//         let evaluator = crate::evaluator::Evaluator::new(semantic_tree);
//         result = evaluator.evaluate().unwrap()
//     }
//     result
// }

// #[test]
// fn test_evaluator() {
//     let source = String::from("1 + 2 * 3");
//     let result = get_evaluator(source);
//     assert_eq!(result, crate::common::datatypes::Variable::from(7));
// }

// #[test]
// fn test_evaluator_2() {
//     let source = String::from("(1 + 2) * 3");
//     let result = get_evaluator(source);
//     assert_eq!(result, crate::common::datatypes::Variable::from(9));
// }

// // a = 10
// #[test]
// fn test_evaluator_3() {
//     let source = String::from("a = 10");
//     let result = get_evaluator(source);
//     assert_eq!(result, crate::common::datatypes::Variable::from(10));
//     assert_eq!(
//         crate::common::symbol_table::SymbolTable::get(&"a".to_string()).unwrap(),
//         crate::common::datatypes::Variable::from(10)
//     );
// }

// // mutable b = 10
// // b = 20
// #[test]
// fn test_evaluator_4() {
//     let source = String::from("mutable b = 10");
//     let result = get_evaluator(source);
//     assert_eq!(result, crate::common::datatypes::Variable::from(10));
//     assert_eq!(
//         crate::common::symbol_table::SymbolTable::get(&"b".to_string()).unwrap(),
//         crate::common::datatypes::Variable::from(10).as_mutable()
//     );

//     let source = String::from("b = 20");
//     let result = get_evaluator(source);
//     assert_eq!(result, crate::common::datatypes::Variable::from(20));
//     assert_eq!(
//         crate::common::symbol_table::SymbolTable::get(&"b".to_string()).unwrap(),
//         crate::common::datatypes::Variable::from(20).as_mutable()
//     );
// }

// #[test]
// fn test_evaluator_5() {
//     let source = String::from("mutable b = 10");
//     let result = get_evaluator(source);
//     assert_eq!(result, crate::common::datatypes::Variable::from(10));
//     assert_eq!(
//         crate::common::symbol_table::SymbolTable::get(&"b".to_string()).unwrap(),
//         crate::common::datatypes::Variable::from(10).as_mutable()
//     );

//     let source = String::from("b += 20");
//     let result = get_evaluator(source);
//     assert_eq!(result, crate::common::datatypes::Variable::from(30));
//     assert_eq!(
//         crate::common::symbol_table::SymbolTable::get(&"b".to_string()).unwrap(),
//         crate::common::datatypes::Variable::from(30).as_mutable()
//     );
// }
