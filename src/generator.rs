use crate::types::*;
use std::collections::{HashMap, HashSet};

pub struct LocoGenerator {
    // 生成済みのテーブルを追跡
    generated_tables: HashSet<String>,
}

impl LocoGenerator {
    pub fn new() -> Self {
        Self {
            generated_tables: HashSet::new(),
        }
    }
    
    pub fn generate_commands(&self, schema: &Schema) -> Vec<String> {
        let mut commands = Vec::new();
        
        // 1. 基本的なテーブル作成（scaffold）
        for entity in &schema.entities {
            commands.push(self.generate_scaffold_command(entity));
        }
        
        // 2. 外部キー参照の追加
        for relationship in &schema.relationships {
            match relationship.relationship_type {
                RelationshipType::OneToMany => {
                    commands.push(self.generate_reference_command(relationship));
                }
                RelationshipType::ManyToMany => {
                    commands.push(self.generate_join_table_command(relationship));
                }
                _ => {}
            }
        }
        
        commands
    }
    
    pub fn generate_scaffold_command(&self, entity: &Entity) -> String {
        let mut parts = vec![
            "cargo".to_string(),
            "loco".to_string(),
            "generate".to_string(),
            "scaffold".to_string(),
            entity.name.clone(),
        ];
        
        // 外部キー以外の属性を追加
        for attr in &entity.attributes {
			if !attr.is_primary_key {
				let loco_type = if attr.is_foreign_key {
					"references".to_string()
				} else {
					LocoDataType::from_mermaid_type(&attr.data_type).to_loco_type().to_string()
				};
				parts.push(format!("{}:{}", attr.name, loco_type));
			}
		}
		
        
        parts.join(" ")
    }
    
    fn generate_reference_command(&self, relationship: &Relationship) -> String {
		let target_table = &relationship.from_entity; // 親側
		let ref_name = target_table.to_lowercase();
		let table_name = &relationship.to_entity; // 子側
	
		format!(
			"cargo loco generate migration Add{}RefTo{} {}:references",
			capitalize(target_table),
			capitalize(table_name),
			ref_name
		)
	}
	
    
    fn generate_join_table_command(&self, relationship: &Relationship) -> String {
		let table1 = &relationship.from_entity.to_lowercase();
		let table2 = &relationship.to_entity.to_lowercase();
		let table_name = format!("{}s_{}s", table1, table2); // 例: posts_tags
	
		format!(
			"cargo loco generate migration CreateJoinTable{}And{} {}:references {}:references",
			capitalize(table1),
			capitalize(table2),
			table1,
			table2
		)
	}
	
    
    // カラム追加のコマンドを生成
    pub fn generate_add_column_command(&self, table_name: &str, columns: &[(&str, &str)]) -> String {
        let column_names: Vec<String> = columns.iter().map(|(name, _)| capitalize(name)).collect();
        let column_name_str = column_names.join("And");
        
        let mut parts = vec![
            "cargo".to_string(),
            "loco".to_string(),
            "generate".to_string(),
            "migration".to_string(),
            format!("Add{}To{}", column_name_str, capitalize(table_name)),
        ];
        
        for (name, data_type) in columns {
            let loco_type = LocoDataType::from_mermaid_type(data_type);
            parts.push(format!("{}:{}", name, loco_type.to_loco_type()));
        }
        
        parts.join(" ")
    }
    
    // カラム削除のコマンドを生成
    pub fn generate_remove_column_command(&self, table_name: &str, columns: &[(&str, &str)]) -> String {
        let column_names: Vec<String> = columns.iter().map(|(name, _)| capitalize(name)).collect();
        let column_name_str = column_names.join("And");
        
        let mut parts = vec![
            "cargo".to_string(),
            "loco".to_string(),
            "generate".to_string(),
            "migration".to_string(),
            format!("Remove{}From{}", column_name_str, capitalize(table_name)),
        ];
        
        for (name, data_type) in columns {
            let loco_type = LocoDataType::from_mermaid_type(data_type);
            parts.push(format!("{}:{}", name, loco_type.to_loco_type()));
        }
        
        parts.join(" ")
    }

	pub fn validate_relationship_entities(&self,schema: &Schema) {
		let entity_names:  HashSet<_> = schema.entities.iter().map(|e| e.name.as_str()).collect();
		for rel in &schema.relationships {
			if !entity_names.contains(rel.from_entity.as_str()) {
				eprintln!(
                    "⚠️ Warning: from_entity '{}' not found in entities",
                    rel.from_entity
                );
			}
			if !entity_names.contains(rel.to_entity.as_str()) {
                eprintln!(
                    "⚠️ Warning: to_entity '{}' not found in entities",
                    rel.to_entity
                );
            }
		}
	}
}

fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}