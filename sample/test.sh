# 全てのテストを実行
cargo test

# 特定のテストモジュールを実行
cargo test parser::tests

# 統合テストのみ実行
cargo test --test integration_test

# テスト結果を詳細表示
cargo test -- --nocapture

# テストカバレッジを確認（cargo-tarpaulin使用）
cargo install cargo-tarpaulin
cargo tarpaulin --out Html