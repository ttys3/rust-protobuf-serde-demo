/// A snazzy new shirt!
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Shirt {
    #[prost(string, tag="1")]
    pub color: ::prost::alloc::string::String,
    #[prost(enumeration="shirt::Size", tag="2")]
    pub size: i32,
    #[prost(int64, tag="3")]
    pub ts: i64,
}
/// Nested message and enum types in `Shirt`.
pub mod shirt {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Size {
        Small = 0,
        Medium = 1,
        Large = 2,
    }
    impl Size {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Size::Small => "SMALL",
                Size::Medium => "MEDIUM",
                Size::Large => "LARGE",
            }
        }
    }
}
