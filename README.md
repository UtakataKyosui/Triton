# 🔱 Triton

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![Build Status](https://img.shields.io/badge/build-passing-green.svg)]()

**Triton** は、Mermaid 形式で記述された Entity-Relationship Diagram (ER 図) を解析し、SeaORM マイグレーションファイルの生成・管理を行う強力な CLI ツールです。データベーススキーマ設計から実装まで、開発ワークフローを大幅に効率化します。

## ✨ 主な機能

- 🎨 **Mermaid ER → SeaORM マイグレーション変換**: Mermaid 形式の ER 図から SeaORM マイグレーションファイルを自動生成
- 🔧 **マイグレーション管理**: 既存のマイグレーションファイルに対するカラムの追加・削除
- 📊 **スキーマ可視化**: マイグレーションファイルの内容をわかりやすく表示
- 🚀 **直感的な CLI**: clap ベースの使いやすいコマンドライン体験
- 🛡️ **堅牢なエラーハンドリング**: 詳細なエラーメッセージと視覚的フィードバック

## 🚀 インストール

### Cargo 経由でのインストール

```bash
cargo install triton
```

### ソースからのビルド

```bash
git clone https://github.com/UtakataKyosui/Triton.git
cd Triton
cargo build --release
```

## 📖 使用方法

### 基本的な使用方法

Triton は複数のサブコマンドを提供し、それぞれが特定の機能を担当します：

```bash
triton --help
```

### 🎨 Generate コマンド - Mermaid ER からマイグレーション生成

Mermaid 形式の ER 図から SeaORM マイグレーションファイルを生成します。

```bash
# 標準出力に表示
triton generate --input diagram.mermaid

# ファイルに出力
triton generate --input diagram.mermaid --output migration.rs
```

#### Mermaid ER 図の例

```mermaid
erDiagram
    users {
        id uuid PK
        name string
        email string
        created_at timestamp
        updated_at timestamp
    }

    posts {
        id uuid PK
        title string
        content text
        user_id uuid FK
        published_at timestamp
        created_at timestamp
        updated_at timestamp
    }

    users ||--o{ posts : "has many"
```

### 🔧 Migration コマンド群 - マイグレーション管理

既存の SeaORM マイグレーションファイルを操作するための包括的なコマンドセットです。

#### パース・検証

マイグレーションファイルの構文チェックと構造解析：

```bash
triton migration parse --file src/migrations/m20240101_000001_create_users.rs
```

#### カラム追加

既存のテーブルに新しいカラムを追加：

```bash
triton migration add-column \
    --file src/migrations/m20240101_000001_create_users.rs \
    --table users \
    --column email \
    --column-type string \
    --nullable false \
    --default "example@email.com"
```

**オプション:**

- `--nullable`: カラムが NULL 値を許可するか (`true`/`false`)
- `--default`: デフォルト値を設定
- `--unique`: UNIQUE 制約を追加
- `--index`: インデックスを作成

#### カラム削除

テーブルから指定されたカラムを削除：

```bash
triton migration drop-column \
    --file src/migrations/m20240101_000001_create_users.rs \
    --table users \
    --column email
```

#### カラム一覧表示

マイグレーションファイル内のテーブル構造を表示：

```bash
# 全テーブルのカラム一覧
triton migration list --file src/migrations/m20240101_000001_create_users.rs

# 特定テーブルのカラム一覧
triton migration list --file src/migrations/m20240101_000001_create_users.rs --table users
```

#### マイグレーション情報表示

マイグレーションファイルの詳細情報を表示：

```bash
triton migration info --file src/migrations/m20240101_000001_create_users.rs
```

## 📚 サポートする型

Triton は SeaORM でサポートされている主要なデータ型をすべてサポートします：

| Mermaid 型  | SeaORM 型    | 説明           |
| ----------- | ------------ | -------------- |
| `string`    | `String`     | 可変長文字列   |
| `text`      | `Text`       | 長いテキスト   |
| `int`       | `Integer`    | 整数           |
| `bigint`    | `BigInteger` | 大きな整数     |
| `decimal`   | `Decimal`    | 10 進数        |
| `float`     | `Float`      | 浮動小数点数   |
| `boolean`   | `Boolean`    | 真偽値         |
| `date`      | `Date`       | 日付           |
| `datetime`  | `DateTime`   | 日時           |
| `timestamp` | `Timestamp`  | タイムスタンプ |
| `uuid`      | `Uuid`       | UUID           |
| `json`      | `Json`       | JSON           |
| `binary`    | `Binary`     | バイナリデータ |

## 🎯 主な改善点

- **📱 ユーザビリティ向上**: clap による直感的な CLI 体験
- **🛡️ エラーハンドリング強化**: 詳細なエラーメッセージと解決策の提示
- **🎨 視覚的フィードバック**: 絵文字とカラー出力による分かりやすい操作体験
- **⚡ 柔軟な出力**: 標準出力またはファイル出力の選択可能
- **🔧 拡張性**: 新機能追加が容易なモジュラー構造

## 🔗 ヘルプコマンド

各コマンドの詳細なヘルプは以下で確認できます：

```bash
# 全体のヘルプ
triton --help

# サブコマンドのヘルプ
triton migration --help
triton migration add-column --help
triton generate --help
```

## 🛠️ 開発環境

### 前提条件

- Rust 1.70 以上
- Cargo

### DevContainer 設定

プロジェクトには開発用の DevContainer が設定済みです：

```json
{
  "name": "Triton DevContainer",
  "image": "mcr.microsoft.com/devcontainers/rust:1-1-bullseye",
  "customizations": {
    "vscode": {
      "extensions": [
        "MermaidChart.vscode-mermaid-chart",
        "vivaxy.vscode-conventional-commits"
      ]
    }
  }
}
```

### 開発用コマンド

```bash
# 開発版のビルド
cargo build

# テストの実行
cargo test

# リンターの実行
cargo clippy

# フォーマット
cargo fmt
```

## 🤝 コントリビューション

プロジェクトへの貢献を歓迎します！以下の手順でコントリビュートしてください：

1. このリポジトリをフォーク
2. フィーチャーブランチを作成 (`git checkout -b feature/amazing-feature`)
3. 変更をコミット (`git commit -m 'Add some amazing feature'`)
4. ブランチにプッシュ (`git push origin feature/amazing-feature`)
5. プルリクエストを作成

### コミット規約

このプロジェクトでは[Conventional Commits](https://www.conventionalcommits.org/)を採用しています：

```
feat: 新機能の追加
fix: バグ修正
docs: ドキュメントの更新
style: コードスタイルの変更
refactor: リファクタリング
test: テストの追加・修正
chore: その他の変更
```

## 📄 ライセンス

このプロジェクトは MIT ライセンスの下で公開されています。詳細は[LICENSE](LICENSE)ファイルを参照してください。

## 🙏 謝辞

- [SeaORM](https://github.com/SeaQL/sea-orm) - 優れた Rust ORM
- [Mermaid](https://mermaid.js.org/) - 美しい図表作成ツール
- [clap](https://github.com/clap-rs/clap) - Rust の素晴らしい CLI ライブラリ

---

**Triton**で、データベース設計から実装までの開発体験を向上させましょう！ 🚀
