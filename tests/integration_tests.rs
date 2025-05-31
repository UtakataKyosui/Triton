use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::fs;
use std::process::Command;
use tempfile::TempDir;

/// テスト用のサンプルMermaid ERダイアグラム
const SAMPLE_MERMAID: &str = r#"
erDiagram
    User {
        int id PK
        string name
        string email UK
        datetime created_at
        datetime updated_at
    }

    Post {
        int id PK
        int user_id FK
        string title
        text content
        boolean published
        datetime created_at
        datetime updated_at
    }

    Comment {
        int id PK
        int post_id FK
        int user_id FK
        text content
        datetime created_at
    }

    User ||--o{ Post : "has many"
    Post ||--o{ Comment : "has many"
    User ||--o{ Comment : "has many"
"#;

/// テスト用のサンプルマイグレーションファイル
const SAMPLE_MIGRATION: &str = r#"
use loco_rs::schema::table;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigration)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table::create_table_auto(Users::Table)
                    .col(
                        ColumnDef::new(Users::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Users::Name).string().not_null())
                    .col(ColumnDef::new(Users::Email).string().not_null().unique_key())
                    .col(ColumnDef::new(Users::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Users::UpdatedAt).timestamp().not_null())
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
    CreatedAt,
    UpdatedAt,
}
"#;

#[test]
fn test_help_command() {
    let mut cmd = Command::cargo_bin("triton").unwrap();
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Triton"))
        .stdout(predicate::str::contains("generate"))
        .stdout(predicate::str::contains("migration"));
}

#[test]
fn test_generate_help() {
    let mut cmd = Command::cargo_bin("triton").unwrap();
    cmd.args(&["generate", "--help"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("--input"))
        .stdout(predicate::str::contains("--output"));
}

#[test]
fn test_migration_help() {
    let mut cmd = Command::cargo_bin("triton").unwrap();
    cmd.args(&["migration", "--help"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("parse"))
        .stdout(predicate::str::contains("add-column"))
        .stdout(predicate::str::contains("drop-column"))
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("info"));
}

#[test]
fn test_generate_from_mermaid() {
    let temp_dir = TempDir::new().unwrap();
    let mermaid_file = temp_dir.path().join("test.mermaid");
    fs::write(&mermaid_file, SAMPLE_MERMAID).unwrap();

    let mut cmd = Command::cargo_bin("triton").unwrap();
    cmd.args(&["generate", "--input", mermaid_file.to_str().unwrap()]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("loco generate scaffold"))
        .stdout(predicate::str::contains("User"))
        .stdout(predicate::str::contains("Post"))
        .stdout(predicate::str::contains("Comment"));
}

#[test]
fn test_generate_to_file() {
    let temp_dir = TempDir::new().unwrap();
    let mermaid_file = temp_dir.path().join("test.mermaid");
    let output_file = temp_dir.path().join("output.txt");

    fs::write(&mermaid_file, SAMPLE_MERMAID).unwrap();

    let mut cmd = Command::cargo_bin("triton").unwrap();
    cmd.args(&[
        "generate",
        "--input",
        mermaid_file.to_str().unwrap(),
        "--output",
        output_file.to_str().unwrap(),
    ]);

    cmd.assert().success();

    // 出力ファイルが作成されているかチェック
    assert!(output_file.exists());
    let content = fs::read_to_string(&output_file).unwrap();
    assert!(content.contains("loco generate scaffold"));
}

#[test]
fn test_migration_parse() {
    let temp_dir = TempDir::new().unwrap();
    let migration_file = temp_dir.path().join("migration.rs");
    fs::write(&migration_file, SAMPLE_MIGRATION).unwrap();

    let mut cmd = Command::cargo_bin("triton").unwrap();
    cmd.args(&[
        "migration",
        "parse",
        "--file",
        migration_file.to_str().unwrap(),
    ]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Successfully parsed"));
}

#[test]
fn test_migration_list_all_tables() {
    let temp_dir = TempDir::new().unwrap();
    let migration_file = temp_dir.path().join("migration.rs");
    fs::write(&migration_file, SAMPLE_MIGRATION).unwrap();

    let mut cmd = Command::cargo_bin("triton").unwrap();
    cmd.args(&[
        "migration",
        "list",
        "--file",
        migration_file.to_str().unwrap(),
    ]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Users"))
        .stdout(predicate::str::contains("Id"))
        .stdout(predicate::str::contains("Name"))
        .stdout(predicate::str::contains("Email"));
}

#[test]
fn test_migration_list_specific_table() {
    let temp_dir = TempDir::new().unwrap();
    let migration_file = temp_dir.path().join("migration.rs");
    fs::write(&migration_file, SAMPLE_MIGRATION).unwrap();

    let mut cmd = Command::cargo_bin("triton").unwrap();
    cmd.args(&[
        "migration",
        "list",
        "--file",
        migration_file.to_str().unwrap(),
        "--table",
        "Users",
    ]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Users"))
        .stdout(predicate::str::contains("Id"))
        .stdout(predicate::str::contains("Name"));
}

#[test]
fn test_migration_info() {
    let temp_dir = TempDir::new().unwrap();
    let migration_file = temp_dir.path().join("migration.rs");
    fs::write(&migration_file, SAMPLE_MIGRATION).unwrap();

    let mut cmd = Command::cargo_bin("triton").unwrap();
    cmd.args(&[
        "migration",
        "info",
        "--file",
        migration_file.to_str().unwrap(),
    ]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Migration"))
        .stdout(predicate::str::contains("Tables"));
}

#[test]
fn test_migration_add_column() {
    let temp_dir = TempDir::new().unwrap();
    let migration_file = temp_dir.path().join("migration.rs");
    fs::write(&migration_file, SAMPLE_MIGRATION).unwrap();

    let mut cmd = Command::cargo_bin("triton").unwrap();
    cmd.args(&[
        "migration",
        "add-column",
        "--file",
        migration_file.to_str().unwrap(),
        "--table",
        "Users",
        "--column",
        "phone",
        "--column-type",
        "string",
        "--nullable",
        "true",
    ]);

    cmd.assert().success();

    // ファイルが更新されているかチェック
    let updated_content = fs::read_to_string(&migration_file).unwrap();
    assert!(updated_content.contains("phone") || updated_content.len() > SAMPLE_MIGRATION.len());
}

#[test]
fn test_migration_add_column_with_default() {
    let temp_dir = TempDir::new().unwrap();
    let migration_file = temp_dir.path().join("migration.rs");
    fs::write(&migration_file, SAMPLE_MIGRATION).unwrap();

    let mut cmd = Command::cargo_bin("triton").unwrap();
    cmd.args(&[
        "migration",
        "add-column",
        "--file",
        migration_file.to_str().unwrap(),
        "--table",
        "Users",
        "--column",
        "status",
        "--column-type",
        "string",
        "--nullable",
        "false",
        "--default",
        "active",
    ]);

    cmd.assert().success();
}

#[test]
fn test_migration_drop_column() {
    let temp_dir = TempDir::new().unwrap();
    let migration_file = temp_dir.path().join("migration.rs");
    fs::write(&migration_file, SAMPLE_MIGRATION).unwrap();

    let mut cmd = Command::cargo_bin("triton").unwrap();
    cmd.args(&[
        "migration",
        "drop-column",
        "--file",
        migration_file.to_str().unwrap(),
        "--table",
        "Users",
        "--column",
        "email",
    ]);

    cmd.assert().success();
}

// エラーケースのテスト

#[test]
fn test_invalid_mermaid_file() {
    let mut cmd = Command::cargo_bin("triton").unwrap();
    cmd.args(&["generate", "--input", "nonexistent.mermaid"]);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("not found").or(predicate::str::contains("No such file")));
}

#[test]
fn test_invalid_migration_file() {
    let mut cmd = Command::cargo_bin("triton").unwrap();
    cmd.args(&["migration", "parse", "--file", "nonexistent.rs"]);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("not found").or(predicate::str::contains("No such file")));
}

#[test]
fn test_missing_required_args() {
    let mut cmd = Command::cargo_bin("triton").unwrap();
    cmd.args(&["generate"]);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("required"));
}

#[test]
fn test_invalid_table_name() {
    let temp_dir = TempDir::new().unwrap();
    let migration_file = temp_dir.path().join("migration.rs");
    fs::write(&migration_file, SAMPLE_MIGRATION).unwrap();

    let mut cmd = Command::cargo_bin("triton").unwrap();
    cmd.args(&[
        "migration",
        "list",
        "--file",
        migration_file.to_str().unwrap(),
        "--table",
        "NonExistentTable",
    ]);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("not found").or(predicate::str::contains("Table")));
}

// パフォーマンステスト用のサンプル

#[test]
fn test_large_mermaid_file() {
    let temp_dir = TempDir::new().unwrap();
    let mermaid_file = temp_dir.path().join("large_test.mermaid");

    // 大きなERダイアグラムを生成
    let mut large_mermaid = String::from("erDiagram\n");
    for i in 0..50 {
        large_mermaid.push_str(&format!(
            r#"
    Table{} {{
        int id PK
        string name
        datetime created_at
    }}
"#,
            i
        ));
    }

    fs::write(&mermaid_file, large_mermaid).unwrap();

    let mut cmd = Command::cargo_bin("triton").unwrap();
    cmd.args(&["generate", "--input", mermaid_file.to_str().unwrap()]);

    cmd.assert().success();
}
