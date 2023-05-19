/// *
/// Representation of data coming in from the mesh network
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MeshData {
    /// who created this message.
    #[prost(uint64, optional, tag = "1")]
    pub source: ::core::option::Option<u64>,
    /// network id.
    #[prost(uint64, optional, tag = "2")]
    pub network: ::core::option::Option<u64>,
    /// sink who forwarded this.
    #[prost(uint64, optional, tag = "3")]
    pub sink: ::core::option::Option<u64>,
    /// version info, if necessary.
    #[prost(float, optional, tag = "4")]
    pub version: ::core::option::Option<f32>,
    /// measurement sequence number.
    #[prost(uint32, optional, tag = "5")]
    pub sequence: ::core::option::Option<u32>,
    /// network travel time in ms
    #[prost(float, optional, tag = "6")]
    pub travel_time: ::core::option::Option<f32>,
    /// when it was sent from the node- unix epoch time.
    #[prost(uint64, optional, tag = "7")]
    pub timestamp: ::core::option::Option<u64>,
    /// measurement data.
    #[prost(message, repeated, tag = "8")]
    pub payload: ::prost::alloc::vec::Vec<MeasurementData>,
    /// number of strongest neighbors to use.
    #[prost(uint32, optional, tag = "20")]
    pub use_strongest_neighbors: ::core::option::Option<u32>,
}
/// *
/// Representation of the application data payload
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MeasurementData {
    /// measurement domain - use 0.
    #[prost(enumeration = "measurement_data::Domain", optional, tag = "1")]
    pub r#type: ::core::option::Option<i32>,
    /// to whom the measurement is done.
    #[prost(uint64, optional, tag = "2")]
    pub target: ::core::option::Option<u64>,
    /// measurement value
    #[prost(float, optional, tag = "3")]
    pub value: ::core::option::Option<f32>,
    /// amount of seconds to when measurement was done.
    #[prost(float, optional, tag = "4")]
    pub time: ::core::option::Option<f32>,
}
/// Nested message and enum types in `MeasurementData`.
pub mod measurement_data {
    /// Measurement domains.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Domain {
        /// RSS domain - subnodes scanning (values in dbm).
        PowerSr = 0,
        /// RSS domain - Not supported (values in dbm).
        PowerHr = 1,
        /// Time domain - Not supported.
        Time = 2,
        /// Space domain - Not supported.
        Space = 3,
        /// RSS domain - subnodes scanning (values in RSSi).
        PowerSrRssi = 10,
        /// RSS domain - Not supported (values in RSSi).
        PowerHrRssi = 11,
        /// no operation - ignores the measurement data
        Ignore = 240,
    }
    impl Domain {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Domain::PowerSr => "POWER_SR",
                Domain::PowerHr => "POWER_HR",
                Domain::Time => "TIME",
                Domain::Space => "SPACE",
                Domain::PowerSrRssi => "POWER_SR_RSSI",
                Domain::PowerHrRssi => "POWER_HR_RSSI",
                Domain::Ignore => "IGNORE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "POWER_SR" => Some(Self::PowerSr),
                "POWER_HR" => Some(Self::PowerHr),
                "TIME" => Some(Self::Time),
                "SPACE" => Some(Self::Space),
                "POWER_SR_RSSI" => Some(Self::PowerSrRssi),
                "POWER_HR_RSSI" => Some(Self::PowerHrRssi),
                "IGNORE" => Some(Self::Ignore),
                _ => None,
            }
        }
    }
}
/// *
/// Representation of the infrastructure data.
///
/// This message contains a set of nodes and areas, which is used as
/// an update/initialisation vector.
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigurationData {
    /// network id.
    #[prost(uint64, optional, tag = "1")]
    pub network: ::core::option::Option<u64>,
    /// node with their set of measurements.
    #[prost(message, repeated, tag = "2")]
    pub nodes: ::prost::alloc::vec::Vec<Node>,
    /// collection of geographic areas.
    #[prost(message, repeated, tag = "3")]
    pub areas: ::prost::alloc::vec::Vec<Area>,
    /// engine id who sent the answer.
    #[prost(string, optional, tag = "200")]
    pub sender: ::core::option::Option<::prost::alloc::string::String>,
}
/// *
///    Node represents an infrastruture device
///    the given address at a particular point in time.
///    It will get translated into a Node instance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Node {
    /// corresponds to the incoming sequence.
    #[prost(uint32, optional, tag = "1")]
    pub sequence: ::core::option::Option<u32>,
    /// an unique device address.
    #[prost(uint64, optional, tag = "2")]
    pub address: ::core::option::Option<u64>,
    /// the network where it belongs to.
    #[prost(uint64, optional, tag = "3")]
    pub network: ::core::option::Option<u64>,
    /// represents the node's mesh role.
    #[prost(enumeration = "node::Baserole", optional, tag = "4")]
    pub role: ::core::option::Option<i32>,
    /// Last or currently known coordinates.
    #[prost(message, optional, tag = "5")]
    pub coordinates: ::core::option::Option<Point>,
    /// list of areas where it is located.
    #[prost(uint64, repeated, packed = "false", tag = "6")]
    pub area_identifier: ::prost::alloc::vec::Vec<u64>,
    /// list of areas where it is located.
    #[prost(string, repeated, tag = "7")]
    pub geo_identifier: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// when it was sent from the node - unix epoch time.
    #[prost(uint64, optional, tag = "10")]
    pub timestamp: ::core::option::Option<u64>,
    /// map where it is located
    #[prost(string, optional, tag = "100")]
    pub map_identifier: ::core::option::Option<::prost::alloc::string::String>,
    /// engine id who sent the answer.
    #[prost(string, optional, tag = "200")]
    pub sender: ::core::option::Option<::prost::alloc::string::String>,
    /// offset added to all measurement
    #[prost(float, optional, tag = "500")]
    pub measurement_offset: ::core::option::Option<f32>,
}
/// Nested message and enum types in `Node`.
pub mod node {
    /// WM Roles.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Baserole {
        Unknown = 0,
        Sink = 1,
        Headnode = 2,
        Subnode = 3,
    }
    impl Baserole {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Baserole::Unknown => "UNKNOWN",
                Baserole::Sink => "SINK",
                Baserole::Headnode => "HEADNODE",
                Baserole::Subnode => "SUBNODE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNKNOWN" => Some(Self::Unknown),
                "SINK" => Some(Self::Sink),
                "HEADNODE" => Some(Self::Headnode),
                "SUBNODE" => Some(Self::Subnode),
                _ => None,
            }
        }
    }
}
/// *
///    Representation of a closed geographic Area.
///
///    A set of points that define the geogrphic polygon that defines a given area.
///    This area contains more metadata regarding building, floor and type of
///    security clearance required to remain in it.
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Area {
    /// logical name for the area.
    #[prost(string, optional, tag = "1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// unique id of the area.
    #[prost(uint64, optional, tag = "2")]
    pub uid: ::core::option::Option<u64>,
    /// floor identifier.
    #[prost(uint64, optional, tag = "3")]
    pub floor: ::core::option::Option<u64>,
    /// building identifier.
    #[prost(uint64, optional, tag = "4")]
    pub building: ::core::option::Option<u64>,
    /// polygon definition.
    #[prost(message, repeated, tag = "5")]
    pub coordinates: ::prost::alloc::vec::Vec<Point>,
    /// uuid
    #[prost(string, optional, tag = "6")]
    pub uuid: ::core::option::Option<::prost::alloc::string::String>,
    /// map where it is located
    #[prost(string, optional, tag = "7")]
    pub map_identifier: ::core::option::Option<::prost::alloc::string::String>,
}
/// *
///    Representation of a geographic Point.
///
///    The definition of a point in space, whose interpretation depends on the
///    GEOID type.
///
///    A point can have multiple reference frames, local and global and keep track
///    of their origin point.
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Point {
    /// to which geoid the lla corresponds to.
    #[prost(enumeration = "point::Geoid", optional, tag = "1")]
    pub geoid: ::core::option::Option<i32>,
    /// latitude, longitude, altitude.
    #[prost(float, repeated, packed = "false", tag = "2")]
    pub lla: ::prost::alloc::vec::Vec<f32>,
}
/// Nested message and enum types in `Point`.
pub mod point {
    /// Geoid definitions.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Geoid {
        /// WGS84 as default.
        Wgs84 = 0,
    }
    impl Geoid {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Geoid::Wgs84 => "WGS84",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "WGS84" => Some(Self::Wgs84),
                _ => None,
            }
        }
    }
}
/// *
/// Represnts the status of a service.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Status {
    /// service dependent status code.
    #[prost(enumeration = "status::Code", optional, tag = "1")]
    pub code: ::core::option::Option<i32>,
    /// subscriber id.
    #[prost(string, optional, tag = "2")]
    pub subscriber_id: ::core::option::Option<::prost::alloc::string::String>,
    /// subscriber id.
    #[prost(string, optional, tag = "3")]
    pub service_id: ::core::option::Option<::prost::alloc::string::String>,
    /// when it was sent - unix epoch time.
    #[prost(uint64, optional, tag = "5")]
    pub timestamp: ::core::option::Option<u64>,
    /// readable text message.
    #[prost(string, optional, tag = "10")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
    /// engine id who sent the answer.
    #[prost(string, optional, tag = "200")]
    pub sender: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `Status`.
pub mod status {
    /// Error code values.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Code {
        /// default value - no return provided.
        Unknown = 0,
        /// call succeeded.
        Success = 1,
        /// an error occured - see message for details.
        Error = 2,
        /// insufficient data - see message for details.
        Insdata = 3,
        /// service has started
        Started = 4,
        /// service has stopped
        Stopped = 5,
    }
    impl Code {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Code::Unknown => "UNKNOWN",
                Code::Success => "SUCCESS",
                Code::Error => "ERROR",
                Code::Insdata => "INSDATA",
                Code::Started => "STARTED",
                Code::Stopped => "STOPPED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNKNOWN" => Some(Self::Unknown),
                "SUCCESS" => Some(Self::Success),
                "ERROR" => Some(Self::Error),
                "INSDATA" => Some(Self::Insdata),
                "STARTED" => Some(Self::Started),
                "STOPPED" => Some(Self::Stopped),
                _ => None,
            }
        }
    }
}
/// *
/// Represents a question performed by the services query payload.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Query {
    /// force a data fetch if possible.
    #[prost(bool, optional, tag = "1")]
    pub force_update: ::core::option::Option<bool>,
    /// subscriber id.
    #[prost(string, optional, tag = "2")]
    pub subscriber_id: ::core::option::Option<::prost::alloc::string::String>,
    /// provide status.
    #[prost(bool, optional, tag = "3")]
    pub send_node_status: ::core::option::Option<bool>,
    /// network id to be used on server queries.
    #[prost(uint32, optional, tag = "4")]
    pub network: ::core::option::Option<u32>,
    /// node id to use in lookups.
    #[prost(uint64, repeated, packed = "false", tag = "5")]
    pub node_id: ::prost::alloc::vec::Vec<u64>,
    /// area id to use in lookups.
    #[prost(uint64, repeated, packed = "false", tag = "6")]
    pub area_id: ::prost::alloc::vec::Vec<u64>,
    /// area id to use in lookups.
    #[prost(string, repeated, tag = "7")]
    pub area_uuid: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// engine id who sent the answer.
    #[prost(string, optional, tag = "200")]
    pub sender: ::core::option::Option<::prost::alloc::string::String>,
}
/// *
/// Represents a question performed by the services query payload.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Log {
    /// other system time
    #[prost(float, optional, tag = "1")]
    pub time: ::core::option::Option<f32>,
    /// message to write.
    #[prost(string, optional, tag = "2")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
    /// identifier of send.
    #[prost(string, optional, tag = "3")]
    pub sender: ::core::option::Option<::prost::alloc::string::String>,
    /// debug level.
    #[prost(uint32, optional, tag = "4")]
    pub level: ::core::option::Option<u32>,
    /// linux epoch timestamp when message is sent.
    #[prost(uint64, optional, tag = "5")]
    pub timestamp: ::core::option::Option<u64>,
}
/// Nested message and enum types in `Log`.
pub mod log {
    /// Debug level filters.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum DebugLevel {
        Debug = 0,
        Error = 1,
        Warning = 2,
        Info = 3,
    }
    impl DebugLevel {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DebugLevel::Debug => "DEBUG",
                DebugLevel::Error => "ERROR",
                DebugLevel::Warning => "WARNING",
                DebugLevel::Info => "INFO",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DEBUG" => Some(Self::Debug),
                "ERROR" => Some(Self::Error),
                "WARNING" => Some(Self::Warning),
                "INFO" => Some(Self::Info),
                _ => None,
            }
        }
    }
}
/// *
/// Empty message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Empty {}
