# ビルド
cargo build --release

# 実行（標準出力）
./target/release/triton ./sample/erDiagram.mermaid

# 実行（ファイル出力）
./target/release/triton ./sample/erDiagram.mermaid output_commands.sh

# 生成されたコマンドファイルを実行可能にして実行
chmod +x output_commands.sh
./output_commands.sh