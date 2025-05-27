use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Schema {
    pub entities: Vec<Entity>,
    pub relationships: Vec<Relationship>,
}

#[derive(Debug, Clone)]
pub struct Entity {
    pub name: String,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug, Clone)]
pub struct Attribute {
    pub name: String,
    pub data_type: String,
    pub is_primary_key: bool,
    pub is_foreign_key: bool,
    pub is_nullable: bool,
}

#[derive(Debug, Clone)]
pub struct Relationship {
    pub from_entity: String,
    pub to_entity: String,
    pub relationship_type: RelationshipType,
    pub from_cardinality: Cardinality,
    pub to_cardinality: Cardinality,
}

#[derive(Debug, Clone)]
pub enum RelationshipType {
    OneToOne,
    OneToMany,
    ManyToMany,
}

#[derive(Debug, Clone)]
pub enum Cardinality {
    Zero,
    One,
    ZeroOrOne,
    ZeroOrMany,
    OneOrMany,
}

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

impl LocoDataType {
    pub fn from_mermaid_type(mermaid_type: &str) -> Self {
        match mermaid_type.to_lowercase().as_str() {
            // UUID
            "uuid" => LocoDataType::UuidNullable,
            "uuid!" => LocoDataType::Uuid,
            "uuid^" => LocoDataType::UuidUnique,

            // String
            "string" => LocoDataType::StringNullable,
            "string!" => LocoDataType::String,
            "string^" => LocoDataType::StringUnique,

            // Text
            "text" => LocoDataType::TextNullable,
            "text!" => LocoDataType::Text,
            "text^" => LocoDataType::TextUnique,

            // Small Unsigned
            "small_unsigned" => LocoDataType::SmallUnsignedNullable,
            "small_unsigned!" => LocoDataType::SmallUnsigned,
            "small_unsigned^" => LocoDataType::SmallUnsignedUnique,

            // Big Unsigned
            "big_unsigned" => LocoDataType::BigUnsigned,
            "big_unsigned!" => LocoDataType::BigUnsignedUnique,
            "big_unsigned^" => LocoDataType::BigUnsignedNullable,

            // Small Int
            "small_int" => LocoDataType::SmallIntNullable,
            "small_int!" => LocoDataType::SmallInt,
            "small_int^" => LocoDataType::SmallIntUnique,

            // Int
            "int" => LocoDataType::IntNullable,
            "int!" => LocoDataType::Int,
            "int^" => LocoDataType::IntUnique,

            // Big Int
            "big_int" => LocoDataType::BigIntNullable,
            "big_int!" => LocoDataType::BigInt,
            "big_int^" => LocoDataType::BigIntUnique,

            // Float
            "float" => LocoDataType::FloatNullable,
            "float!" => LocoDataType::Float,
            "float^" => LocoDataType::FloatUnique,

            // Double
            "double" => LocoDataType::DoubleNullable,
            "double!" => LocoDataType::Double,
            "double^" => LocoDataType::DoubleUnique,

            // Decimal
            "decimal" => LocoDataType::DecimalNullable,
            "decimal!" => LocoDataType::Decimal,
            "decimal^" => LocoDataType::DecimalUnique,

            // Decimal with length
            "decimal_len" => LocoDataType::DecimalLenNullable,
            "decimal_len!" => LocoDataType::DecimalLen,

            // Boolean
            "bool" => LocoDataType::BooleanNullable,
            "bool!" => LocoDataType::Boolean,

            // Timestamp with time zone
            "tstz" => LocoDataType::TimestampWithTimeZoneNullable,
            "tstz!" => LocoDataType::TimestampWithTimeZone,

            // Date
            "date" => LocoDataType::DateNullable,
            "date!" => LocoDataType::Date,
            "date^" => LocoDataType::DateUnique,

            // DateTime
            "date_time" => LocoDataType::DateTimeNullable,
            "date_time!" => LocoDataType::DateTime,
            "date_time^" => LocoDataType::DateTimeUnique,

            // Blob
            "blob" => LocoDataType::BlobNullable,
            "blob!" => LocoDataType::Blob,
            "blob^" => LocoDataType::BlobUnique,

            // JSON
            "json" => LocoDataType::JsonNullable,
            "json!" => LocoDataType::Json,

            // JSON Binary
            "jsonb" => LocoDataType::JsonBinaryNullable,
            "jsonb!" => LocoDataType::JsonBinary,
            "jsonb^" => LocoDataType::JsonBinaryUnique,

            // Money
            "money" => LocoDataType::MoneyNullable,
            "money!" => LocoDataType::Money,
            "money^" => LocoDataType::MoneyUnique,

            // Unsigned
            "unsigned" => LocoDataType::UnsignedNullable,
            "unsigned!" => LocoDataType::Unsigned,
            "unsigned^" => LocoDataType::UnsignedUnique,

            // Binary Length
            "binary_len" => LocoDataType::BinaryLenNullable,
            "binary_len!" => LocoDataType::BinaryLen,
            "binary_len^" => LocoDataType::BinaryLenUnique,

            // Var Binary
            "var_binary" => LocoDataType::VarBinaryNullable,
            "var_binary!" => LocoDataType::VarBinary,

            // Array
            "array" => LocoDataType::Array,
            "array!" => LocoDataType::ArrayUnique,
            "array^" => LocoDataType::ArrayNullable,

            // fallback
            _ => LocoDataType::StringNullable,
        }
    }

    pub fn to_loco_type(&self) -> &'static str {
        match self {
            LocoDataType::UuidNullable => "uuid",
            LocoDataType::Uuid => "uuid!",
            LocoDataType::UuidUnique => "uuid^",

            LocoDataType::StringNullable => "string",
            LocoDataType::String => "string!",
            LocoDataType::StringUnique => "string^",

            LocoDataType::TextNullable => "text",
            LocoDataType::Text => "text!",
            LocoDataType::TextUnique => "text^",

            LocoDataType::SmallUnsignedNullable => "small_unsigned",
            LocoDataType::SmallUnsigned => "small_unsigned!",
            LocoDataType::SmallUnsignedUnique => "small_unsigned^",

            LocoDataType::BigUnsigned => "big_unsigned",
            LocoDataType::BigUnsignedUnique => "big_unsigned!",
            LocoDataType::BigUnsignedNullable => "big_unsigned^",

            LocoDataType::SmallIntNullable => "small_int",
            LocoDataType::SmallInt => "small_int!",
            LocoDataType::SmallIntUnique => "small_int^",

            LocoDataType::IntNullable => "int",
            LocoDataType::Int => "int!",
            LocoDataType::IntUnique => "int^",

            LocoDataType::BigIntNullable => "big_int",
            LocoDataType::BigInt => "big_int!",
            LocoDataType::BigIntUnique => "big_int^",

            LocoDataType::FloatNullable => "float",
            LocoDataType::Float => "float!",
            LocoDataType::FloatUnique => "float^",

            LocoDataType::DoubleNullable => "double",
            LocoDataType::Double => "double!",
            LocoDataType::DoubleUnique => "double^",

            LocoDataType::DecimalNullable => "decimal",
            LocoDataType::Decimal => "decimal!",
            LocoDataType::DecimalUnique => "decimal^",

            LocoDataType::DecimalLenNullable => "decimal_len",
            LocoDataType::DecimalLen => "decimal_len!",

            LocoDataType::BooleanNullable => "bool",
            LocoDataType::Boolean => "bool!",

            LocoDataType::TimestampWithTimeZoneNullable => "tstz",
            LocoDataType::TimestampWithTimeZone => "tstz!",

            LocoDataType::DateNullable => "date",
            LocoDataType::Date => "date!",
            LocoDataType::DateUnique => "date^",

            LocoDataType::DateTimeNullable => "date_time",
            LocoDataType::DateTime => "date_time!",
            LocoDataType::DateTimeUnique => "date_time^",

            LocoDataType::BlobNullable => "blob",
            LocoDataType::Blob => "blob!",
            LocoDataType::BlobUnique => "blob^",

            LocoDataType::JsonNullable => "json",
            LocoDataType::Json => "json!",

            LocoDataType::JsonBinaryNullable => "jsonb",
            LocoDataType::JsonBinary => "jsonb!",
            LocoDataType::JsonBinaryUnique => "jsonb^",

            LocoDataType::MoneyNullable => "money",
            LocoDataType::Money => "money!",
            LocoDataType::MoneyUnique => "money^",

            LocoDataType::UnsignedNullable => "unsigned",
            LocoDataType::Unsigned => "unsigned!",
            LocoDataType::UnsignedUnique => "unsigned^",

            LocoDataType::BinaryLenNullable => "binary_len",
            LocoDataType::BinaryLen => "binary_len!",
            LocoDataType::BinaryLenUnique => "binary_len^",

            LocoDataType::VarBinaryNullable => "var_binary",
            LocoDataType::VarBinary => "var_binary!",

            LocoDataType::Array => "array",
            LocoDataType::ArrayUnique => "array!",
            LocoDataType::ArrayNullable => "array^",
        }
    }
}
