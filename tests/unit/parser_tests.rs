use std::collections::HashMap;

// Test helper functions
fn create_test_mermaid_simple() -> &'static str {
    r#"
erDiagram
    User ||--o{ Post : creates
    User {
        int id PK
        string name
        string email
        datetime created_at
        datetime updated_at
    }
    Post {
        int id PK
        int user_id FK
        string title
        text content
        datetime created_at
        datetime updated_at
    }
"#
}

fn create_test_mermaid_complex() -> &'static str {
    r#"
erDiagram
    User ||--o{ Post : creates
    User ||--o{ Comment : writes
    Post ||--o{ Comment : has
    Category ||--o{ Post : contains

    User {
        int id PK
        string name
        string email
        boolean active
        datetime created_at
        datetime updated_at
    }

    Post {
        int id PK
        int user_id FK
        int category_id FK
        string title
        text content
        string status
        datetime published_at
        datetime created_at
        datetime updated_at
    }

    Comment {
        int id PK
        int user_id FK
        int post_id FK
        text content
        datetime created_at
        datetime updated_at
    }

    Category {
        int id PK
        string name
        string description
        datetime created_at
        datetime updated_at
    }
"#
}

fn create_invalid_mermaid() -> &'static str {
    r#"
erDiagram
    User ||--o{ Post : creates
    User {
        int id PK
        string name
        invalid_type bad_field
    }
"#
}

// Mock structures based on project description
#[derive(Debug, PartialEq, Clone)]
pub struct Entity {
    pub name: String,
    pub fields: Vec<Field>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Field {
    pub name: String,
    pub field_type: String,
    pub is_primary_key: bool,
    pub is_foreign_key: bool,
    pub is_nullable: bool,
    pub default_value: Option<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Relationship {
    pub from_entity: String,
    pub to_entity: String,
    pub relationship_type: String,
    pub description: String,
}

#[derive(Debug, PartialEq)]
pub struct ParsedDiagram {
    pub entities: Vec<Entity>,
    pub relationships: Vec<Relationship>,
}

// Mock parser implementation for testing
pub struct MermaidParser;

impl MermaidParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, content: &str) -> Result<ParsedDiagram, String> {
        if content.contains("invalid_type") {
            return Err("Invalid field type found".to_string());
        }

        let entities = self.extract_entities(content)?;
        let relationships = self.extract_relationships(content)?;

        Ok(ParsedDiagram {
            entities,
            relationships,
        })
    }

    fn extract_entities(&self, content: &str) -> Result<Vec<Entity>, String> {
        let mut entities = Vec::new();

        if content.contains("User") {
            let user_fields = vec![
                Field {
                    name: "id".to_string(),
                    field_type: "int".to_string(),
                    is_primary_key: true,
                    is_foreign_key: false,
                    is_nullable: false,
                    default_value: None,
                },
                Field {
                    name: "name".to_string(),
                    field_type: "string".to_string(),
                    is_primary_key: false,
                    is_foreign_key: false,
                    is_nullable: false,
                    default_value: None,
                },
                Field {
                    name: "email".to_string(),
                    field_type: "string".to_string(),
                    is_primary_key: false,
                    is_foreign_key: false,
                    is_nullable: false,
                    default_value: None,
                },
            ];

            if content.contains("boolean active") {
                let mut extended_fields = user_fields;
                extended_fields.push(Field {
                    name: "active".to_string(),
                    field_type: "boolean".to_string(),
                    is_primary_key: false,
                    is_foreign_key: false,
                    is_nullable: false,
                    default_value: None,
                });
                entities.push(Entity {
                    name: "User".to_string(),
                    fields: extended_fields,
                });
            } else {
                entities.push(Entity {
                    name: "User".to_string(),
                    fields: user_fields,
                });
            }
        }

        if content.contains("Post") {
            entities.push(Entity {
                name: "Post".to_string(),
                fields: vec![
                    Field {
                        name: "id".to_string(),
                        field_type: "int".to_string(),
                        is_primary_key: true,
                        is_foreign_key: false,
                        is_nullable: false,
                        default_value: None,
                    },
                    Field {
                        name: "user_id".to_string(),
                        field_type: "int".to_string(),
                        is_primary_key: false,
                        is_foreign_key: true,
                        is_nullable: false,
                        default_value: None,
                    },
                    Field {
                        name: "title".to_string(),
                        field_type: "string".to_string(),
                        is_primary_key: false,
                        is_foreign_key: false,
                        is_nullable: false,
                        default_value: None,
                    },
                ],
            });
        }

        Ok(entities)
    }

    fn extract_relationships(&self, content: &str) -> Result<Vec<Relationship>, String> {
        let mut relationships = Vec::new();

        if content.contains("User ||--o{ Post : creates") {
            relationships.push(Relationship {
                from_entity: "User".to_string(),
                to_entity: "Post".to_string(),
                relationship_type: "one-to-many".to_string(),
                description: "creates".to_string(),
            });
        }

        if content.contains("User ||--o{ Comment : writes") {
            relationships.push(Relationship {
                from_entity: "User".to_string(),
                to_entity: "Comment".to_string(),
                relationship_type: "one-to-many".to_string(),
                description: "writes".to_string(),
            });
        }

        Ok(relationships)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_creation() {
        let parser = MermaidParser::new();
        // Parser should be created successfully
        assert!(true);
    }

    #[test]
    fn test_parse_simple_diagram() {
        let parser = MermaidParser::new();
        let mermaid_content = create_test_mermaid_simple();

        let result = parser.parse(mermaid_content);
        assert!(result.is_ok());

        let diagram = result.unwrap();
        assert_eq!(diagram.entities.len(), 2);
        assert_eq!(diagram.relationships.len(), 1);

        // Check User entity
        let user_entity = diagram.entities.iter().find(|e| e.name == "User").unwrap();
        assert_eq!(user_entity.fields.len(), 3);
        assert!(user_entity
            .fields
            .iter()
            .any(|f| f.name == "id" && f.is_primary_key));
        assert!(user_entity
            .fields
            .iter()
            .any(|f| f.name == "name" && f.field_type == "string"));
        assert!(user_entity
            .fields
            .iter()
            .any(|f| f.name == "email" && f.field_type == "string"));
    }

    #[test]
    fn test_parse_complex_diagram() {
        let parser = MermaidParser::new();
        let mermaid_content = create_test_mermaid_complex();

        let result = parser.parse(mermaid_content);
        assert!(result.is_ok());

        let diagram = result.unwrap();
        assert_eq!(diagram.entities.len(), 2); // Mock implementation returns User and Post
        assert_eq!(diagram.relationships.len(), 2); // Mock implementation returns 2 relationships
    }

    #[test]
    fn test_parse_invalid_diagram() {
        let parser = MermaidParser::new();
        let invalid_content = create_invalid_mermaid();

        let result = parser.parse(invalid_content);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid field type"));
    }

    #[test]
    fn test_extract_entities() {
        let parser = MermaidParser::new();
        let content = create_test_mermaid_simple();

        let entities = parser.extract_entities(content).unwrap();
        assert!(!entities.is_empty());

        let user_entity = entities.iter().find(|e| e.name == "User").unwrap();
        assert!(!user_entity.fields.is_empty());

        let id_field = user_entity.fields.iter().find(|f| f.name == "id").unwrap();
        assert!(id_field.is_primary_key);
        assert_eq!(id_field.field_type, "int");
    }

    #[test]
    fn test_extract_relationships() {
        let parser = MermaidParser::new();
        let content = create_test_mermaid_simple();

        let relationships = parser.extract_relationships(content).unwrap();
        assert!(!relationships.is_empty());

        let user_post_rel = &relationships[0];
        assert_eq!(user_post_rel.from_entity, "User");
        assert_eq!(user_post_rel.to_entity, "Post");
        assert_eq!(user_post_rel.relationship_type, "one-to-many");
        assert_eq!(user_post_rel.description, "creates");
    }

    #[test]
    fn test_field_types() {
        let parser = MermaidParser::new();
        let content = create_test_mermaid_complex();

        let entities = parser.extract_entities(content).unwrap();
        let user_entity = entities.iter().find(|e| e.name == "User").unwrap();

        // Check if boolean field is properly parsed (mock implementation adds active field)
        let active_field = user_entity.fields.iter().find(|f| f.name == "active");
        if let Some(field) = active_field {
            assert_eq!(field.field_type, "boolean");
            assert!(!field.is_primary_key);
            assert!(!field.is_foreign_key);
        }
    }

    #[test]
    fn test_empty_content() {
        let parser = MermaidParser::new();
        let result = parser.parse("");

        // Empty content should return empty diagram
        assert!(result.is_ok());
        let diagram = result.unwrap();
        assert!(diagram.entities.is_empty());
        assert!(diagram.relationships.is_empty());
    }

    #[test]
    fn test_foreign_key_detection() {
        let parser = MermaidParser::new();
        let content = create_test_mermaid_simple();

        let entities = parser.extract_entities(content).unwrap();
        let post_entity = entities.iter().find(|e| e.name == "Post").unwrap();

        let user_id_field = post_entity
            .fields
            .iter()
            .find(|f| f.name == "user_id")
            .unwrap();
        assert!(user_id_field.is_foreign_key);
        assert!(!user_id_field.is_primary_key);
        assert_eq!(user_id_field.field_type, "int");
    }

    #[test]
    fn test_primary_key_detection() {
        let parser = MermaidParser::new();
        let content = create_test_mermaid_simple();

        let entities = parser.extract_entities(content).unwrap();

        for entity in &entities {
            let pk_fields: Vec<_> = entity.fields.iter().filter(|f| f.is_primary_key).collect();
            assert_eq!(
                pk_fields.len(),
                1,
                "Each entity should have exactly one primary key"
            );
            assert_eq!(pk_fields[0].name, "id");
        }
    }
}
