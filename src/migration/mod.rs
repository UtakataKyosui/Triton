// src/migration/mod.rs

mod modifier;
pub mod parser;

pub use modifier::MigrationModifier;
pub use parser::{MigrationInfo, MigrationParser, Operation};

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ColumnInfo {
    pub name: String,
    pub column_type: String,
    pub nullable: bool,
    pub default: Option<String>,
}

impl ColumnInfo {
    pub fn new(name: String, column_type: String) -> Self {
        Self {
            name,
            column_type,
            nullable: false,
            default: None,
        }
    }

    pub fn nullable(mut self) -> Self {
        self.nullable = true;
        self
    }

    pub fn not_null(mut self) -> Self {
        self.nullable = false;
        self
    }

    pub fn default_value<T: ToString>(mut self, value: T) -> Self {
        self.default = Some(value.to_string());
        self
    }
}

impl std::fmt::Display for ColumnInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let nullable_str = if self.nullable {
            "nullable"
        } else {
            "not null"
        };
        let default_str = self
            .default
            .as_ref()
            .map(|d| format!(" (default: {})", d))
            .unwrap_or_default();

        write!(
            f,
            "{} ({}, {}){}",
            self.name, self.column_type, nullable_str, default_str
        )
    }
}
