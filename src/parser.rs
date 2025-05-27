use crate::types::*;
use regex::Regex;
use std::collections::HashMap;

pub struct MermaidParser {
    entity_regex: Regex,
    attribute_regex: Regex,
    relationship_regex: Regex,
}

impl MermaidParser {
    pub fn new() -> Self {
        Self {
            entity_regex: Regex::new(r"^\s*(\w+)\s*\{").unwrap(),
            attribute_regex: Regex::new(r"^\s*(\w+)\s+(\w+)\s*(PK)?\s*(FK)?\s*(\?)?\s*$").unwrap(),
            relationship_regex: Regex::new(
				r#"(?x)
					(?P<from_entity>\w+)\s*
					(?P<symbol>\|\|--\|\||\|\|--o\{|\}o--\|\||\}o--o\{|
								 \|o--\|\||o\|--o\{|\|o--o\{|\}o--o\||o\|--\|\|)\s*
					(?P<to_entity>\w+)\s*:\s*
					"(?P<label>.+)"
				"#
			).unwrap(),
        }
    }
    
    pub fn parse(&mut self, content: &str) -> Result<Schema, Box<dyn std::error::Error>> {
        let mut entities = Vec::new();
        let mut relationships = Vec::new();
        let mut current_entity: Option<Entity> = None;
        
        for line in content.lines() {
            let line = line.trim();
            
            // エンティティの開始
            if let Some(captures) = self.entity_regex.captures(line) {
                if let Some(entity) = current_entity.take() {
                    entities.push(entity);
                }
                
                let entity_name = captures.get(1).unwrap().as_str().to_string();
                current_entity = Some(Entity {
                    name: entity_name,
                    attributes: Vec::new(),
                });
            }
            // エンティティ内の属性
            else if let Some(ref mut entity) = current_entity {
                if line == "}" {
                    entities.push(entity.clone());
                    current_entity = None;
                } else if let Some(captures) = self.attribute_regex.captures(line) {
                    let raw_type = captures.get(1).unwrap().as_str();
					let attr_type = LocoDataType::from_mermaid_type(raw_type).to_loco_type().to_string();
					let attr_name = LocoDataType::from_mermaid_type(captures.get(2).unwrap().as_str()).to_loco_type().to_string();
					let is_pk = captures.get(3).is_some();
					let is_fk = captures.get(4).is_some();
					let is_nullable = captures.get(5).is_some();
                    
                    entity.attributes.push(Attribute {
                        name: attr_name,
                        data_type: attr_type,
                        is_primary_key: is_pk,
                        is_foreign_key: is_fk,
                        is_nullable,
                    });
                }
            }
            // リレーションシップ
            else if let Some(captures) = self.relationship_regex.captures(line) {
                let from_entity = captures.get(1).unwrap().as_str().to_string();
                let relation_symbol = captures.get(2).unwrap().as_str();
                let to_entity = captures.get(3).unwrap().as_str().to_string();
                
                let (rel_type, from_card, to_card) = self.parse_relationship_symbol(relation_symbol);
                
                relationships.push(Relationship {
                    from_entity,
                    to_entity,
                    relationship_type: rel_type,
                    from_cardinality: from_card,
                    to_cardinality: to_card,
                });
            }
        }
        
        // 最後のエンティティを追加
        if let Some(entity) = current_entity {
            entities.push(entity);
        }
        
        Ok(Schema { entities, relationships })
    }
    
    fn parse_relationship_symbol(&self, symbol: &str) -> (RelationshipType, Cardinality, Cardinality) {
        match symbol {
            "||--||" => (RelationshipType::OneToOne, Cardinality::One, Cardinality::One),
            "||--o{" => (RelationshipType::OneToMany, Cardinality::One, Cardinality::ZeroOrMany),
            "o{--||" => (RelationshipType::OneToMany, Cardinality::ZeroOrMany, Cardinality::One),
            "o{--o{" => (RelationshipType::ManyToMany, Cardinality::ZeroOrMany, Cardinality::ZeroOrMany),
            _ => (RelationshipType::OneToMany, Cardinality::One, Cardinality::ZeroOrMany),
        }
    }
}
