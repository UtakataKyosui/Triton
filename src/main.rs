use std::env;
use std::fs;
use std::io::{self, Write};

mod parser;
mod generator;
mod types;

use parser::MermaidParser;
use generator::LocoGenerator;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("使用方法: {} <mermaid_file_path> [output_file_path]", args[0]);
        std::process::exit(1);
    }
    
    let input_file = &args[1];
    let output_file = args.get(2).map(|s| s.as_str());
    
    // Mermaidファイルを読み込み
    let mermaid_content = fs::read_to_string(input_file)?;
    
    // パース
    let mut parser = MermaidParser::new();
    let schema = parser.parse(&mermaid_content)?;
    
    // Locoコマンド生成
    let generator = LocoGenerator::new();
    let commands = generator.generate_commands(&schema);
    
    // 出力
    let output = commands.join("\n");
    
    match output_file {
        Some(path) => {
            fs::write(path, &output)?;
            println!("コマンドを {}に出力しました", path);
        }
        None => {
            println!("{}", output);
        }
    }
    
    Ok(())
}
