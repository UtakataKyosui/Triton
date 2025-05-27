# 要件定義書

## 制約

- Mermaid側はLocoに沿った型の決め方をしているとする。
- Mermaidで定義されているモデルのカラムは全て`型名 カラム名 [PK,FK] コメント`の順で記述されていなければならないものとする
- そのカラムがモデルにおいて必須のカラムか否かの判断については、以下の条件で判断するとする
	- 型名の後に`!`があれば必須
	- カラムの`型名 カラム名`の次に`"Require"`があれば必須とする
	- `PK`,`FK`だった場合も必須となる
- そのカラムがモデルにおいて、ユニーク制約を持つかどうかの判断は以下の条件で判断するとする。
	- 型名の後に`^`があればユニーク制約を持つ
	- カラムの`型名 カラム名`の次に`"Unique"`があれば必須とする
- 上記に沿わないものは`null許容`であるもの（要は型名だけのものはnull許容）とする。

Model定義する際のコマンドのカラム名の型記述は公式でこう指定されている
```
("uuid^", "uuid_uniq"),
("uuid", "uuid_null"),
("uuid!", "uuid"),

("string", "string_null"),
("string!", "string"),
("string^", "string_uniq"),

("text", "text_null"),
("text!", "text"),
("text^", "text_uniq"),

("small_unsigned^", "small_unsigned_uniq"),
("small_unsigned", "small_unsigned_null"),
("small_unsigned!", "small_unsigned"),

("big_unsigned^", "big_unsigned"),
("big_unsigned", "big_unsigned_null"),
("big_unsigned!", "big_unsigned_uniq"),

("small_int", "small_integer_null"),
("small_int!", "small_integer"),
("small_int^", "small_integer_uniq"),

("int", "integer_null"),
("int!", "integer"),
("int^", "integer_uniq"),

("big_int", "big_integer_null"),
("big_int!", "big_integer"),
("big_int^", "big_integer_uniq"),

("float", "float_null"),
("float!", "float"),
("float^", "float_uniq"),

("double", "double_null"),
("double!", "double"),
("double^", "double_uniq"),

("decimal", "decimal_null"),
("decimal!", "decimal"),
("decimal^", "decimal_uniq"),

("decimal_len", "decimal_len_null"),
("decimal_len!", "decimal_len"),

("bool", "boolean_null"),
("bool!", "boolean"),

("tstz", "timestamp_with_time_zone_null"),
("tstz!", "timestamp_with_time_zone"),

("date", "date_null"),
("date!", "date"),
("date^", "date_uniq"),

("date_time", "date_time_null"),
("date_time!", "date_time"),
("date_time^", "date_time_uniq"),

("blob", "blob_null"),
("blob!", "blob"),
("blob^", "blob_uniq"),

("json", "json_null"),
("json!", "json"),

("jsonb", "json_binary_null"),
("jsonb!", "json_binary"),
("jsonb^", "jsonb_uniq"),

("money", "money_null"),
("money!", "money"),
("money^", "money_uniq"),

("unsigned", "unsigned_null"),
("unsigned!", "unsigned"),
("unsigned^", "unsigned_uniq"),

("binary_len", "binary_len_null"),
("binary_len!", "binary_len"),
("binary_len^", "binary_len_uniq"),

("var_binary", "var_binary_null"),
("var_binary!", "var_binary"),

(" array", "array"),
(" array!", "array"),
(" array^", "array"),
```

つまりこうなる
```rust
#[derive(Debug)]
pub enum LocoDataType {
    // UUID
    UuidNullable,
    UuidUnique,
    Uuid,

    // String
    StringNullable,
    StringUnique,
    String,

    // Text
    TextNullable,
    TextUnique,
    Text,

    // Small Unsigned Int
    SmallUnsignedNullable,
    SmallUnsignedUnique,
    SmallUnsigned,

    // Big Unsigned Int
    BigUnsignedNullable,
    BigUnsignedUnique,
    BigUnsigned,

    // Small Int
    SmallIntNullable,
    SmallIntUnique,
    SmallInt,

    // Int
    IntNullable,
    IntUnique,
    Int,

    // Big Int
    BigIntNullable,
    BigIntUnique,
    BigInt,

    // Float
    FloatNullable,
    FloatUnique,
    Float,

    // Double
    DoubleNullable,
    DoubleUnique,
    Double,

    // Decimal
    DecimalNullable,
    DecimalUnique,
    Decimal,

    // Decimal with length
    DecimalLenNullable,
    DecimalLen,

    // Boolean
    BooleanNullable,
    Boolean,

    // Timestamp with timezone
    TimestampWithTimeZoneNullable,
    TimestampWithTimeZone,

    // Date
    DateNullable,
    DateUnique,
    Date,

    // DateTime
    DateTimeNullable,
    DateTimeUnique,
    DateTime,

    // Blob
    BlobNullable,
    BlobUnique,
    Blob,

    // JSON
    JsonNullable,
    Json,

    // JSON Binary
    JsonBinaryNullable,
    JsonBinaryUnique,
    JsonBinary,

    // Money
    MoneyNullable,
    MoneyUnique,
    Money,

    // Unsigned
    UnsignedNullable,
    UnsignedUnique,
    Unsigned,

    // Binary Length
    BinaryLenNullable,
    BinaryLenUnique,
    BinaryLen,

    // VarBinary
    VarBinaryNullable,
    VarBinary,

    // Array（変数名の先頭空白を修正）
    Array,
    ArrayUnique,
    ArrayNullable,
}
```