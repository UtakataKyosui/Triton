```rust
// 追加のカラムを後から追加する場合
let generator = LocoGenerator::new();
let add_command = generator.generate_add_column_command(
    "users", 
    &[("phone", "string"), ("age", "int")]
);
println!("{}", add_command);
// 出力: cargo loco generate migration AddPhoneAndAgeToUsers phone:string age:int

// カラムを削除する場合  
let remove_command = generator.generate_remove_column_command(
    "users",
    &[("phone", "string"), ("age", "int")]
);
println!("{}", remove_command);
// 出力: cargo loco generate migration RemovePhoneAndAgeFromUsers phone:string age:int
```