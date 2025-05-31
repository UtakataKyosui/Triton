# Triton: Mermaid ER 図から Loco Scaffolding を生成する CLI

## 概要

Triton は、Mermaid 形式で記述された Entity-Relationship Diagram (ER 図) を解析し、Loco Framework 用の Scaffold モデルとコントローラを自動生成する CLI ツールです。Mermaid ER 図を簡単に Loco アプリケーションに変換し、開発効率を大幅に向上させます。

## 使用方法

`triton` コマンドは、Mermaid ER 図を含むファイルを引数として受け取り、Loco Scaffold の生成コマンドの文字列を出力します。

基本的な実行方法は以下の通りです。

```bash
triton <mermaid_er_diagram_file>
```

`<mermaid_er_diagram_file>` は、Mermaid 形式で記述された ER 図を含むファイルのパスです。

**例:**

`data_model.mermaid` というファイルに ER 図が記述されている場合、以下のコマンドを実行します。

```bash
triton data_model.mermaid
```

新機能

1. ヘルプコマンド
   bash# 全体のヘルプ
   triton --help

# サブコマンドのヘルプ

triton migration --help
triton migration add-column --help

2. Generate コマンド
   bash# 標準出力に表示
   triton generate --input diagram.mermaid

# ファイルに保存

triton generate --input diagram.mermaid --output commands.txt

3. Migration コマンド群
   パース・検証

   triton migration parse --file migration.rs
   カラム追加
   triton migration add-column \
    --file migration.rs \
    --table users \
    --column email \
    --column-type string \
    --nullable false \
    --default "example@email.com"
   カラム削除
   triton migration drop-column \
    --file migration.rs \
    --table users \
    --column email
   カラム一覧表示
   bash# 全テーブルのカラム一覧
   triton migration list --file migration.rs

# 特定テーブルのカラム一覧

triton migration list --file migration.rs --table users
マイグレーション情報表示
triton migration info --file migration.rs
🎯 主な改善点

ユーザビリティ向上: clap による直感的な CLI 体験
エラーハンドリング強化: 詳細なエラーメッセージ
視覚的フィードバック: 絵文字を使った分かりやすい出力
柔軟な出力: stdout または ファイル出力の選択可能
拡張性: 新機能追加が容易な構造

以前の DevContainer の設定

```json
// For format details, see https://aka.ms/devcontainer.json. For config options, see the
// README at: https://github.com/devcontainers/templates/tree/main/src/rust
{
  "name": "Triton DevContainer",
  // Or use a Dockerfile or Docker Compose file. More info: https://containers.dev/guide/dockerfile
  "image": "mcr.microsoft.com/devcontainers/rust:1-1-bullseye",
  "customizations": {
    "vscode": {
      "extensions": [
        "MermaidChart.vscode-mermaid-chart",
        "vivaxy.vscode-conventional-commits"
      ]
    }
  }

  // Use 'mounts' to make the cargo cache persistent in a Docker Volume.
  // "mounts": [
  // 	{
  // 		"source": "devcontainer-cargo-cache-${devcontainerId}",
  // 		"target": "/usr/local/cargo",
  // 		"type": "volume"
  // 	}
  // ]

  // Features to add to the dev container. More info: https://containers.dev/features.
  // "features": {},

  // Use 'forwardPorts' to make a list of ports inside the container available locally.
  // "forwardPorts": [],

  // Use 'postCreateCommand' to run commands after the container is created.
  // "postCreateCommand": "rustc --version",

  // Configure tool-specific properties.
  // "customizations": {},

  // Uncomment to connect as root instead. More info: https://aka.ms/dev-containers-non-root.
  // "remoteUser": "root"
}
```
