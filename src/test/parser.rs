#[cfg(test)]

fn parse(source: String) -> std::sync::Arc<std::sync::RwLock<crate::parsing::block::Block>> {
    let mut lexer = crate::lexing::lexer::Lexer::new(source);
    lexer.lex().unwrap();
    let mut parser = crate::parsing::parser::Parser::new(lexer);
    parser.parse().unwrap()
}

#[test]
fn test_parser() {
    let source = String::from("a = 1 + 2 * 3\n");
    let global_block = parse(source);
    assert_eq!(global_block.read().unwrap().statements.len(), 1);
}

#[test]
fn test_parser2() {
    let source = String::from("a = 1 + 2 * 3\nb = 1 + 2 * 3\n");
    let global_block = parse(source);
    assert_eq!(global_block.read().unwrap().statements.len(), 2);
}
