# Triton: Mermaid ER図からLoco Scaffoldingを生成するCLI

## 概要

Tritonは、Mermaid形式で記述されたEntity-Relationship Diagram (ER図) を解析し、Loco Framework用のScaffoldモデルとコントローラを自動生成するCLIツールです。Mermaid ER図を簡単にLocoアプリケーションに変換し、開発効率を大幅に向上させます。

## 使用方法

`triton` コマンドは、Mermaid ER図を含むファイルを引数として受け取り、Loco Scaffoldの生成コマンドの文字列を出力します。

基本的な実行方法は以下の通りです。

```bash
triton <mermaid_er_diagram_file>
```

`<mermaid_er_diagram_file>` は、Mermaid形式で記述されたER図を含むファイルのパスです。

**例:**

`data_model.mermaid` というファイルにER図が記述されている場合、以下のコマンドを実行します。

```bash
triton data_model.mermaid
```