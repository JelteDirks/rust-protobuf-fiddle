/// This is the inner options message, which basically defines options for
/// a field. When it is used in message or file scope, it applies to all
/// fields.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NanoPbOptions {
    /// Allocated size for 'bytes' and 'string' fields.
    /// For string fields, this should include the space for null terminator.
    #[prost(int32, optional, tag = "1")]
    pub max_size: ::core::option::Option<i32>,
    /// Maximum length for 'string' fields. Setting this is equivalent
    /// to setting max_size to a value of length+1.
    #[prost(int32, optional, tag = "14")]
    pub max_length: ::core::option::Option<i32>,
    /// Allocated number of entries in arrays ('repeated' fields)
    #[prost(int32, optional, tag = "2")]
    pub max_count: ::core::option::Option<i32>,
    /// Size of integer fields. Can save some memory if you don't need
    /// full 32 bits for the value.
    #[prost(enumeration = "IntSize", optional, tag = "7", default = "IsDefault")]
    pub int_size: ::core::option::Option<i32>,
    /// Force type of field (callback or static allocation)
    #[prost(enumeration = "FieldType", optional, tag = "3", default = "FtDefault")]
    pub r#type: ::core::option::Option<i32>,
    /// Use long names for enums, i.e. EnumName_EnumValue.
    #[prost(bool, optional, tag = "4", default = "true")]
    pub long_names: ::core::option::Option<bool>,
    /// Add 'packed' attribute to generated structs.
    /// Note: this cannot be used on CPUs that break on unaligned
    /// accesses to variables.
    #[prost(bool, optional, tag = "5", default = "false")]
    pub packed_struct: ::core::option::Option<bool>,
    /// Add 'packed' attribute to generated enums.
    #[prost(bool, optional, tag = "10", default = "false")]
    pub packed_enum: ::core::option::Option<bool>,
    /// Skip this message
    #[prost(bool, optional, tag = "6", default = "false")]
    pub skip_message: ::core::option::Option<bool>,
    /// Generate oneof fields as normal optional fields instead of union.
    #[prost(bool, optional, tag = "8", default = "false")]
    pub no_unions: ::core::option::Option<bool>,
    /// integer type tag for a message
    #[prost(uint32, optional, tag = "9")]
    pub msgid: ::core::option::Option<u32>,
    /// decode oneof as anonymous union
    #[prost(bool, optional, tag = "11", default = "false")]
    pub anonymous_oneof: ::core::option::Option<bool>,
    /// Proto3 singular field does not generate a "has_" flag
    #[prost(bool, optional, tag = "12", default = "false")]
    pub proto3: ::core::option::Option<bool>,
    /// Generate an enum->string mapping function (can take up lots of space).
    #[prost(bool, optional, tag = "13", default = "false")]
    pub enum_to_string: ::core::option::Option<bool>,
    /// Generate bytes arrays with fixed length
    #[prost(bool, optional, tag = "15", default = "false")]
    pub fixed_length: ::core::option::Option<bool>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FieldType {
    /// Automatically decide field type, generate static field if possible.
    FtDefault = 0,
    /// Always generate a callback field.
    FtCallback = 1,
    /// Always generate a dynamically allocated field.
    FtPointer = 4,
    /// Generate a static field or raise an exception if not possible.
    FtStatic = 2,
    /// Ignore the field completely.
    FtIgnore = 3,
    /// Legacy option, use the separate 'fixed_length' option instead
    FtInline = 5,
}
impl FieldType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            FieldType::FtDefault => "FT_DEFAULT",
            FieldType::FtCallback => "FT_CALLBACK",
            FieldType::FtPointer => "FT_POINTER",
            FieldType::FtStatic => "FT_STATIC",
            FieldType::FtIgnore => "FT_IGNORE",
            FieldType::FtInline => "FT_INLINE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "FT_DEFAULT" => Some(Self::FtDefault),
            "FT_CALLBACK" => Some(Self::FtCallback),
            "FT_POINTER" => Some(Self::FtPointer),
            "FT_STATIC" => Some(Self::FtStatic),
            "FT_IGNORE" => Some(Self::FtIgnore),
            "FT_INLINE" => Some(Self::FtInline),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum IntSize {
    /// Default, 32/64bit based on type in .proto
    IsDefault = 0,
    Is8 = 8,
    Is16 = 16,
    Is32 = 32,
    Is64 = 64,
}
impl IntSize {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            IntSize::IsDefault => "IS_DEFAULT",
            IntSize::Is8 => "IS_8",
            IntSize::Is16 => "IS_16",
            IntSize::Is32 => "IS_32",
            IntSize::Is64 => "IS_64",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "IS_DEFAULT" => Some(Self::IsDefault),
            "IS_8" => Some(Self::Is8),
            "IS_16" => Some(Self::Is16),
            "IS_32" => Some(Self::Is32),
            "IS_64" => Some(Self::Is64),
            _ => None,
        }
    }
}
