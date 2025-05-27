use triton::parser::MermaidParser;
use triton::generator::LocoGenerator;
use triton::types::*;

#[test]
fn test_special_characters_in_names() {
    let mut parser = MermaidParser::new();
    let content = r#"
UserProfile {
    int id PK
    string user_name
    string first_name
    string last_name
}
"#;
    
    let schema = parser.parse(content).unwrap();
    let entity = &schema.entities[0];
    
    assert_eq!(entity.name, "UserProfile");
    assert!(entity.attributes.iter().any(|attr| attr.name == "user_name"));
    assert!(entity.attributes.iter().any(|attr| attr.name == "first_name"));
    assert!(entity.attributes.iter().any(|attr| attr.name == "last_name"));
}

#[test]
fn test_multiple_primary_keys() {
    let mut parser = MermaidParser::new();
    let content = r#"
CompositeKey {
    int user_id PK
    int post_id PK
    timestamp created_at
}
"#;
    
    let schema = parser.parse(content).unwrap();
    let entity = &schema.entities[0];
    
    let pk_count = entity.attributes.iter()
        .filter(|attr| attr.is_primary_key)
        .count();
    
    assert_eq!(pk_count, 2);
}

#[test]
fn test_mixed_case_data_types() {
    let mut parser = MermaidParser::new();
    let content = r#"
MixedCase {
    INT id PK
    STRING name
    Text description
    BOOLEAN active
}
"#;
    
    let schema = parser.parse(content).unwrap();
    let entity = &schema.entities[0];
    
    // データ型の変換が正しく動作することを確認
    let generator = LocoGenerator::new();
    let command = generator.generate_scaffold_command(entity);
    
    assert!(command.contains("name:string"));
    assert!(command.contains("description:text"));
    assert!(command.contains("active:bool"));
}

#[test]
fn test_circular_relationships() {
    let mut parser = MermaidParser::new();
    let content = r#"
User {
    int id PK
    string name
}

Friend {
    int id PK
    int user_id FK
    int friend_id FK
}

User ||--o{ Friend : "has many friends"
User ||--o{ Friend : "has many followers"
"#;
    
    let schema = parser.parse(content).unwrap();
    
    // 循環参照があっても正しく処理されることを確認
    assert_eq!(schema.entities.len(), 2);
    assert_eq!(schema.relationships.len(), 2);
    
    let generator = LocoGenerator::new();
    let commands = generator.generate_commands(&schema);
    
    // コマンドが生成されることを確認
    assert!(commands.len() > 0);
}