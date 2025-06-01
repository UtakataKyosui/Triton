// tests/common/mod.rs - テスト用共通ヘルパー関数

use std::fs;
use std::io::Write;
use std::path::Path;
use tempfile::TempDir;

pub mod assertions;
pub mod fixtures;

/// テスト用のMermaidファイルを作成
pub fn create_test_mermaid_file(content: &str) -> (TempDir, String) {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let file_path = temp_dir.path().join("test_diagram.mermaid");
    let mut file = fs::File::create(&file_path).expect("Failed to create test file");
    file.write_all(content.as_bytes())
        .expect("Failed to write test content");
    (temp_dir, file_path.to_string_lossy().to_string())
}

/// テスト用のマイグレーションファイルを作成
pub fn create_test_migration_file(content: &str) -> (TempDir, String) {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let file_path = temp_dir.path().join("migration.rs");
    let mut file = fs::File::create(&file_path).expect("Failed to create migration file");
    file.write_all(content.as_bytes())
        .expect("Failed to write migration content");
    (temp_dir, file_path.to_string_lossy().to_string())
}

/// テスト用の出力ファイルを作成
pub fn create_test_output_file() -> (TempDir, String) {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let file_path = temp_dir.path().join("output.txt");
    (temp_dir, file_path.to_string_lossy().to_string())
}

/// ファイルの内容を読み取り
pub fn read_file_content(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Failed to read file content")
}

/// ディレクトリ内のファイル一覧を取得
pub fn list_files_in_dir(dir_path: &Path) -> Vec<String> {
    fs::read_dir(dir_path)
        .expect("Failed to read directory")
        .filter_map(|entry| {
            entry
                .ok()
                .and_then(|e| e.file_name().to_str().map(|s| s.to_string()))
        })
        .collect()
}

/// テスト用のCLI引数を作成
pub fn create_cli_args(command: &str, args: &[&str]) -> Vec<String> {
    let mut cli_args = vec!["triton".to_string(), command.to_string()];
    cli_args.extend(args.iter().map(|s| s.to_string()));
    cli_args
}

// tests/common/fixtures.rs - テスト用フィクスチャデータ

/// シンプルなMermaid ERダイアグラム
pub const SIMPLE_MERMAID_DIAGRAM: &str = r#"
    erDiagram
        USER {
            int id PK
            string name
            string email
        }
"#;

/// 複数エンティティを持つMermaid ERダイアグラム
pub const COMPLEX_MERMAID_DIAGRAM: &str = r#"
    erDiagram
        USER {
            int id PK
            string name
            string email
            datetime created_at
            datetime updated_at
        }
        POST {
            int id PK
            int user_id FK
            string title
            text content
            boolean published
            datetime created_at
            datetime updated_at
        }
        COMMENT {
            int id PK
            int post_id FK
            int user_id FK
            text content
            datetime created_at
        }

        USER ||--o{ POST : "has"
        POST ||--o{ COMMENT : "has"
        USER ||--o{ COMMENT : "writes"
"#;

/// リレーションシップを含むMermaid ERダイアグラム
pub const RELATIONSHIP_MERMAID_DIAGRAM: &str = r#"
    erDiagram
        CUSTOMER {
            int customer_id PK
            string first_name
            string last_name
            string email
        }
        ORDER {
            int order_id PK
            int customer_id FK
            datetime order_date
            decimal total_amount
        }
        ORDER_ITEM {
            int item_id PK
            int order_id FK
            int product_id FK
            int quantity
            decimal unit_price
        }
        PRODUCT {
            int product_id PK
            string name
            text description
            decimal price
        }

        CUSTOMER ||--o{ ORDER : "places"
        ORDER ||--o{ ORDER_ITEM : "contains"
        PRODUCT ||--o{ ORDER_ITEM : "ordered_as"
"#;

/// 基本的なSeaORMマイグレーションファイル
pub const BASIC_MIGRATION_FILE: &str = r#"
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Users::Name).string().not_null())
                    .col(ColumnDef::new(Users::Email).string())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Users {
    Table,
    Id,
    Name,
    Email,
}
"#;

/// 複数テーブルを持つマイグレーションファイル
pub const MULTI_TABLE_MIGRATION_FILE: &str = r#"
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create Users table
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Users::Name).string().not_null())
                    .col(ColumnDef::new(Users::Email).string().unique_key())
                    .col(ColumnDef::new(Users::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Users::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await?;

        // Create Posts table
        manager
            .create_table(
                Table::create()
                    .table(Posts::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Posts::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Posts::UserId).integer().not_null())
                    .col(ColumnDef::new(Posts::Title).string().not_null())
                    .col(ColumnDef::new(Posts::Content).text())
                    .col(ColumnDef::new(Posts::Published).boolean().default(false))
                    .col(ColumnDef::new(Posts::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Posts::UpdatedAt).timestamp().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_posts_user_id")
                            .from(Posts::Table, Posts::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Posts::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Users {
    Table,
    Id,
    Name,
    Email,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Posts {
    Table,
    Id,
    UserId,
    Title,
    Content,
    Published,
    CreatedAt,
    UpdatedAt,
}
"#;

/// 無効なMermaidダイアグラム
pub const INVALID_MERMAID_DIAGRAM: &str = r#"
    invalid mermaid syntax
    this is not a valid diagram
    USER {
        missing closing brace
"#;

/// 無効なマイグレーションファイル
pub const INVALID_MIGRATION_FILE: &str = r#"
    invalid rust syntax
    this is not a valid migration file
    missing semicolons and proper structure
"#;

// tests/common/assertions.rs - テスト用アサーション関数

use std::path::Path;

/// ファイルが存在することをアサート
pub fn assert_file_exists(file_path: &str) {
    assert!(
        Path::new(file_path).exists(),
        "File does not exist: {}",
        file_path
    );
}

/// ファイルが存在しないことをアサート
pub fn assert_file_not_exists(file_path: &str) {
    assert!(
        !Path::new(file_path).exists(),
        "File should not exist: {}",
        file_path
    );
}

/// ファイルの内容に特定の文字列が含まれることをアサート
pub fn assert_file_contains(file_path: &str, expected_content: &str) {
    let content = super::read_file_content(file_path);
    assert!(
        content.contains(expected_content),
        "File '{}' does not contain expected content: '{}'",
        file_path,
        expected_content
    );
}

/// ファイルの内容に特定の文字列が含まれないことをアサート
pub fn assert_file_not_contains(file_path: &str, unexpected_content: &str) {
    let content = super::read_file_content(file_path);
    assert!(
        !content.contains(unexpected_content),
        "File '{}' contains unexpected content: '{}'",
        file_path,
        unexpected_content
    );
}

/// 文字列が有効なScaffoldコマンドであることをアサート
pub fn assert_valid_scaffold_command(command: &str) {
    assert!(command.contains("loco generate scaffold"));
    assert!(command.matches(':').count() >= 2); // 最低2つのフィールドがある
}

/// エンティティリストが期待される数のエンティティを含むことをアサート
pub fn assert_entity_count(entities: &[crate::Entity], expected_count: usize) {
    assert_eq!(
        entities.len(),
        expected_count,
        "Expected {} entities, but found {}",
        expected_count,
        entities.len()
    );
}

/// エンティティが期待される属性を持つことをアサート
pub fn assert_entity_has_attribute(entity: &crate::Entity, attribute_name: &str) {
    assert!(
        entity
            .attributes
            .iter()
            .any(|attr| attr.name == attribute_name),
        "Entity '{}' does not have attribute '{}'",
        entity.name,
        attribute_name
    );
}

/// コマンドの実行結果が成功であることをアサート
pub fn assert_command_success(result: &Result<(), Box<dyn std::error::Error>>) {
    assert!(result.is_ok(), "Command failed: {:?}", result);
}

/// コマンドの実行結果がエラーであることをアサート
pub fn assert_command_error(result: &Result<(), Box<dyn std::error::Error>>) {
    assert!(result.is_err(), "Command should have failed but succeeded");
}

/// 出力に期待されるメッセージが含まれることをアサート
pub fn assert_output_contains(output: &str, expected_message: &str) {
    assert!(
        output.contains(expected_message),
        "Output does not contain expected message: '{}'\nActual output: '{}'",
        expected_message,
        output
    );
}
