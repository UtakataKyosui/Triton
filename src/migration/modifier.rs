// src/migration/modifier.rs

use super::{MigrationInfo, MigrationParser, Operation};
use crate::migration::parser::ColumnInfo;
use std::collections::HashMap;
use std::fs;

pub struct MigrationModifier {
    migration_info: MigrationInfo,
    file_path: String,
}

impl MigrationModifier {
    pub fn new(file_path: String) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(&file_path)
            .map_err(|e| format!("Failed to read migration file '{}': {}", file_path, e))?;

        let migration_info = MigrationParser::parse_migration_file(&content)
            .map_err(|e| format!("Failed to parse migration file: {}", e))?;

        Ok(Self {
            migration_info,
            file_path,
        })
    }

    pub fn add_column(
        &mut self,
        table_name: &str,
        column: ColumnInfo,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // upメソッドにadd_column操作を追加
        let add_column_op = Operation::AddColumn {
            table: table_name.to_string(),
            column: column.clone(),
        };

        self.migration_info.up_operations.push(add_column_op);

        // downメソッドにdrop_column操作を追加（先頭に挿入してrollback順序を正しくする）
        let drop_column_op = Operation::DropColumn {
            table: table_name.to_string(),
            column_name: column.name.clone(),
        };

        self.migration_info
            .down_operations
            .insert(0, drop_column_op);

        self.write_to_file()?;
        Ok(())
    }

    pub fn drop_column(
        &mut self,
        table_name: &str,
        column_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // 既存のカラム情報を取得（drop用のrollback情報として）
        let column_info = self
            .find_column_info(table_name, column_name)
            .unwrap_or_else(|_| {
                // カラム情報が見つからない場合はデフォルト値を使用
                ColumnInfo {
                    name: column_name.to_string(),
                    column_type: "string".to_string(),
                    nullable: true,
                    default: None,
                }
            });

        // upメソッドにdrop_column操作を追加
        let drop_column_op = Operation::DropColumn {
            table: table_name.to_string(),
            column_name: column_name.to_string(),
        };

        self.migration_info.up_operations.push(drop_column_op);

        // downメソッドにadd_column操作を追加（rollback用）
        let add_column_op = Operation::AddColumn {
            table: table_name.to_string(),
            column: column_info,
        };

        self.migration_info.down_operations.insert(0, add_column_op);

        self.write_to_file()?;
        Ok(())
    }

    pub fn list_columns(&self, table_name: &str) -> Vec<&ColumnInfo> {
        let mut columns = Vec::new();

        // すべての操作からテーブルのカラム情報を収集
        for operation in &self.migration_info.up_operations {
            match operation {
                Operation::CreateTable {
                    name,
                    columns: table_columns,
                } if name == table_name => {
                    columns.extend(table_columns.iter());
                }
                Operation::AddColumn { table, column } if table == table_name => {
                    columns.push(column);
                }
                _ => {}
            }
        }

        columns
    }

    pub fn list_all_columns(&self) -> HashMap<String, Vec<&ColumnInfo>> {
        let mut all_columns = HashMap::new();

        for operation in &self.migration_info.up_operations {
            match operation {
                Operation::CreateTable { name, columns } => {
                    all_columns
                        .entry(name.clone())
                        .or_insert_with(Vec::new)
                        .extend(columns.iter());
                }
                Operation::AddColumn { table, column } => {
                    all_columns
                        .entry(table.clone())
                        .or_insert_with(Vec::new)
                        .push(column);
                }
                _ => {}
            }
        }

        all_columns
    }

    pub fn get_migration_info(&self) -> &MigrationInfo {
        &self.migration_info
    }

    fn find_column_info(
        &self,
        table_name: &str,
        column_name: &str,
    ) -> Result<ColumnInfo, Box<dyn std::error::Error>> {
        // 既存のマイグレーションから該当カラムの情報を検索
        for operation in &self.migration_info.up_operations {
            match operation {
                Operation::CreateTable { name, columns } if name == table_name => {
                    if let Some(column) = columns.iter().find(|col| col.name == column_name) {
                        return Ok(column.clone());
                    }
                }
                Operation::AddColumn { table, column }
                    if table == table_name && column.name == column_name =>
                {
                    return Ok(column.clone());
                }
                _ => {}
            }
        }

        Err(format!(
            "Column '{}' not found in table '{}'",
            column_name, table_name
        )
        .into())
    }

    fn write_to_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        let generated_code = self.generate_migration_code()?;
        fs::write(&self.file_path, generated_code)
            .map_err(|e| format!("Failed to write migration file: {}", e))?;
        Ok(())
    }

    fn generate_migration_code(&self) -> Result<String, Box<dyn std::error::Error>> {
        // ここではシンプルな文字列生成を行います
        // 実際の実装では、synとquoteを使ってより正確なコード生成を行う必要があります

        let mut code = String::new();
        code.push_str("use sea_orm_migration::prelude::*;\n\n");
        code.push_str("#[derive(DeriveMigrationName)]\n");
        code.push_str("pub struct Migration;\n\n");
        code.push_str("#[async_trait::async_trait]\n");
        code.push_str("impl MigrationTrait for Migration {\n");
        code.push_str("    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {\n");

        // up操作を生成
        for operation in &self.migration_info.up_operations {
            code.push_str(&self.generate_operation_code(operation, 8));
        }

        code.push_str("        Ok(())\n");
        code.push_str("    }\n\n");
        code.push_str("    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {\n");

        // down操作を生成
        for operation in &self.migration_info.down_operations {
            code.push_str(&self.generate_operation_code(operation, 8));
        }

        code.push_str("        Ok(())\n");
        code.push_str("    }\n");
        code.push_str("}\n");

        Ok(code)
    }

    fn generate_operation_code(&self, operation: &Operation, indent: usize) -> String {
        let indent_str = " ".repeat(indent);

        match operation {
            Operation::CreateTable { name, columns } => {
                let mut code = format!("{}manager\n", indent_str);
                code.push_str(&format!("{}    .create_table(\n", indent_str));
                code.push_str(&format!("{}        Table::create()\n", indent_str));
                code.push_str(&format!(
                    "{}            .table({}::Table)\n",
                    indent_str,
                    name.to_uppercase()
                ));

                for column in columns {
                    let column_def = self.generate_column_definition(column);
                    code.push_str(&format!("{}            .col({})\n", indent_str, column_def));
                }

                code.push_str(&format!("{}            .to_owned(),\n", indent_str));
                code.push_str(&format!("{}    )\n", indent_str));
                code.push_str(&format!("{}    .await?;\n\n", indent_str));
                code
            }
            Operation::AddColumn { table, column } => {
                let column_def = self.generate_column_definition(column);
                format!("{}manager\n{}    .alter_table(\n{}        Table::alter()\n{}            .table({}::Table)\n{}            .add_column({})\n{}            .to_owned(),\n{}    )\n{}    .await?;\n\n",
                    indent_str, indent_str, indent_str, indent_str, table.to_uppercase(), indent_str, column_def, indent_str, indent_str, indent_str)
            }
            Operation::DropColumn { table, column_name } => {
                format!("{}manager\n{}    .alter_table(\n{}        Table::alter()\n{}            .table({}::Table)\n{}            .drop_column({}::{})\n{}            .to_owned(),\n{}    )\n{}    .await?;\n\n",
                    indent_str, indent_str, indent_str, indent_str, table.to_uppercase(), indent_str, table.to_uppercase(), column_name.to_uppercase(), indent_str, indent_str, indent_str)
            }
            Operation::DropTable { name } => {
                format!("{}manager\n{}    .drop_table(Table::drop().table({}::Table).to_owned())\n{}    .await?;\n\n",
                    indent_str, indent_str, name.to_uppercase(), indent_str)
            }
        }
    }

    fn generate_column_definition(&self, column: &ColumnInfo) -> String {
        let mut def = format!(
            "ColumnDef::new({}::{})",
            "Alias", // テーブル名が必要な場合は調整
            column.name.to_uppercase()
        );

        // カラムタイプの追加
        match column.column_type.as_str() {
            "string" => def.push_str(".string()"),
            "integer" => def.push_str(".integer()"),
            "boolean" => def.push_str(".boolean()"),
            "text" => def.push_str(".text()"),
            "timestamp" => def.push_str(".timestamp()"),
            _ => def.push_str(".string()"), // デフォルト
        }

        // nullable設定
        if !column.nullable {
            def.push_str(".not_null()");
        }

        // デフォルト値の設定
        if let Some(default_val) = &column.default {
            def.push_str(&format!(".default(\"{}\")", default_val));
        }

        def
    }
}
