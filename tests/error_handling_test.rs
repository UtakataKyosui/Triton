use triton::parser::MermaidParser;

#[test]
fn test_empty_input() {
    let mut parser = MermaidParser::new();
    let schema = parser.parse("").unwrap();
    
    assert_eq!(schema.entities.len(), 0);
    assert_eq!(schema.relationships.len(), 0);
}

#[test]
fn test_malformed_entity() {
    let mut parser = MermaidParser::new();
    let content = r#"
User {
    invalid_attribute_format
}
"#;
    
    let schema = parser.parse(content).unwrap();
    assert_eq!(schema.entities.len(), 1);
    
    // 不正な属性は無視される
    let user = &schema.entities[0];
    assert_eq!(user.attributes.len(), 0);
}

#[test]
fn test_missing_entity_closing_brace() {
    let mut parser = MermaidParser::new();
    let content = r#"
User {
    int id PK
    string name
"#;
    
    // パーサーは不完全なエンティティも処理できる
    let schema = parser.parse(content).unwrap();
    assert_eq!(schema.entities.len(), 1);
}

#[test]
fn test_invalid_relationship_format() {
    let mut parser = MermaidParser::new();
    let content = r#"
User {
    int id PK
}

Post {
    int id PK
}

User -- Post
"#;
    
    let schema = parser.parse(content).unwrap();
    assert_eq!(schema.entities.len(), 2);
    // 不正なリレーションシップ形式は無視される
    assert_eq!(schema.relationships.len(), 0);
}