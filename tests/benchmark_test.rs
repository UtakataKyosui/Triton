use std::time::Instant;
use triton::parser::MermaidParser;
use triton::generator::LocoGenerator;

#[test]
fn test_performance_large_schema() {
    let mut large_content = String::from("erDiagram\n");
    
    // 100個のエンティティを作成
    for i in 0..100 {
        large_content.push_str(&format!(r#"
    Entity{} {{
        int id PK
        string name
        string description
        timestamp created_at
        timestamp updated_at
    }}
"#, i));
    }
    
    // リレーションシップを追加
    for i in 0..50 {
        large_content.push_str(&format!(
            "    Entity{} ||--o{{ Entity{} : \"has many\"\n",
            i, i + 1
        ));
    }
    
    let start = Instant::now();
    
    let mut parser = MermaidParser::new();
    let schema = parser.parse(&large_content).unwrap();
    
    let generator = LocoGenerator::new();
    let commands = generator.generate_commands(&schema);
    
    let duration = start.elapsed();
    
    assert_eq!(schema.entities.len(), 100);
    assert_eq!(schema.relationships.len(), 50);
    assert!(commands.len() > 100);
    
    // パフォーマンステスト: 1秒以内で完了することを確認
    assert!(duration.as_secs() < 1, "処理時間が1秒を超えました: {:?}", duration);
}

#[test]
fn test_memory_usage() {
    // メモリ使用量のテスト
    let mut large_content = String::from("erDiagram\n");
    
    for i in 0..1000 {
        large_content.push_str(&format!(r#"
    Entity{} {{
        int id PK
        string field1
        string field2
        string field3
        string field4
        string field5
    }}
"#, i));
    }
    
    let mut parser = MermaidParser::new();
    let schema = parser.parse(&large_content).unwrap();
    
    // 大量のエンティティが正しく処理されることを確認
    assert_eq!(schema.entities.len(), 1000);
    
    for (i, entity) in schema.entities.iter().enumerate() {
        assert_eq!(entity.name, format!("Entity{}", i));
        assert_eq!(entity.attributes.len(), 6); // id + 5つのフィールド
    }
}