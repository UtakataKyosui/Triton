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


以前のDevContainerの設定
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