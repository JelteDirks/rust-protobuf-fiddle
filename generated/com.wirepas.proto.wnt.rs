#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FullRole {
    #[prost(enumeration = "BaseRole", optional, tag = "1")]
    pub base_role: ::core::option::Option<i32>,
    /// True = low latency, false = low energy
    #[prost(bool, optional, tag = "2")]
    pub cb_mac: ::core::option::Option<bool>,
    /// Not supported - optional bool is_relay = 3;
    #[prost(bool, optional, tag = "4")]
    pub is_autorole: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppConfigData {
    /// Diagnostics interval in seconds
    #[prost(uint32, optional, tag = "1")]
    pub interval: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub sequence: ::core::option::Option<u32>,
    #[prost(bytes = "vec", optional, tag = "3")]
    pub app_config: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    /// Maximum length of the app config
    #[prost(uint32, optional, tag = "4")]
    pub max_length: ::core::option::Option<u32>,
    /// WNT backend will override appconfig if it is changed outside of WNT
    #[prost(bool, optional, tag = "5")]
    pub is_override_on: ::core::option::Option<bool>,
    #[prost(enumeration = "SelectionType", optional, tag = "6")]
    pub selection_type: ::core::option::Option<i32>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BaseRole {
    Subnode = 1,
    Headnode = 2,
    Sink = 4,
    RoleUnknown = 255,
}
impl BaseRole {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            BaseRole::Subnode => "SUBNODE",
            BaseRole::Headnode => "HEADNODE",
            BaseRole::Sink => "SINK",
            BaseRole::RoleUnknown => "ROLE_UNKNOWN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SUBNODE" => Some(Self::Subnode),
            "HEADNODE" => Some(Self::Headnode),
            "SINK" => Some(Self::Sink),
            "ROLE_UNKNOWN" => Some(Self::RoleUnknown),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MessageSendingType {
    Instant = 1,
    Distributed = 2,
}
impl MessageSendingType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MessageSendingType::Instant => "MESSAGE_SENDING_TYPE_INSTANT",
            MessageSendingType::Distributed => "MESSAGE_SENDING_TYPE_DISTRIBUTED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "MESSAGE_SENDING_TYPE_INSTANT" => Some(Self::Instant),
            "MESSAGE_SENDING_TYPE_DISTRIBUTED" => Some(Self::Distributed),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SelectionType {
    Network = 1,
    Sink = 2,
}
impl SelectionType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SelectionType::Network => "SELECTION_TYPE_NETWORK",
            SelectionType::Sink => "SELECTION_TYPE_SINK",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SELECTION_TYPE_NETWORK" => Some(Self::Network),
            "SELECTION_TYPE_SINK" => Some(Self::Sink),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ScratchpadAction {
    Unknown = 0,
    NoOtap = 1,
    PropagateOnly = 2,
    PropagateAndProcess = 3,
    PropagateAndProcessWithDelay = 4,
    Legacy = 5,
}
impl ScratchpadAction {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ScratchpadAction::Unknown => "SCRATCHPAD_ACTION_UNKNOWN",
            ScratchpadAction::NoOtap => "SCRATCHPAD_ACTION_NO_OTAP",
            ScratchpadAction::PropagateOnly => "SCRATCHPAD_ACTION_PROPAGATE_ONLY",
            ScratchpadAction::PropagateAndProcess => {
                "SCRATCHPAD_ACTION_PROPAGATE_AND_PROCESS"
            }
            ScratchpadAction::PropagateAndProcessWithDelay => {
                "SCRATCHPAD_ACTION_PROPAGATE_AND_PROCESS_WITH_DELAY"
            }
            ScratchpadAction::Legacy => "SCRATCHPAD_ACTION_LEGACY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SCRATCHPAD_ACTION_UNKNOWN" => Some(Self::Unknown),
            "SCRATCHPAD_ACTION_NO_OTAP" => Some(Self::NoOtap),
            "SCRATCHPAD_ACTION_PROPAGATE_ONLY" => Some(Self::PropagateOnly),
            "SCRATCHPAD_ACTION_PROPAGATE_AND_PROCESS" => Some(Self::PropagateAndProcess),
            "SCRATCHPAD_ACTION_PROPAGATE_AND_PROCESS_WITH_DELAY" => {
                Some(Self::PropagateAndProcessWithDelay)
            }
            "SCRATCHPAD_ACTION_LEGACY" => Some(Self::Legacy),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetOtapState {
    #[prost(enumeration = "OtapState", optional, tag = "1")]
    pub state: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "2")]
    pub network_id: ::core::option::Option<u32>,
    #[prost(bytes = "vec", optional, tag = "3")]
    pub scratchpad_bytes: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag = "4")]
    pub resend_interval_s: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "5")]
    pub is_sink_only: ::core::option::Option<bool>,
    #[prost(uint32, optional, tag = "6")]
    pub sequence_number: ::core::option::Option<u32>,
    #[prost(enumeration = "OtapMethod", optional, tag = "7")]
    pub method: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "8")]
    pub scratchpad_crc: ::core::option::Option<u32>,
    #[prost(enumeration = "OtapnpadDelay", optional, tag = "9")]
    pub npad_delay: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "10")]
    pub start_time_s_epoch: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "11")]
    pub loading_time_s_epoch: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "12")]
    pub activation_time_s_epoch: ::core::option::Option<u32>,
    #[prost(enumeration = "MessageSendingType", optional, tag = "20")]
    pub message_sending_type: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetOtapStateResponse {
    #[prost(enumeration = "OtapState", optional, tag = "1")]
    pub state: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "2")]
    pub start_time_s_epoch: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub activation_time_s_epoch: ::core::option::Option<u32>,
    #[prost(
        enumeration = "set_otap_state_response::OtapResponseId",
        optional,
        tag = "10"
    )]
    pub load_scratchpad_response: ::core::option::Option<i32>,
    #[prost(
        enumeration = "set_otap_state_response::OtapResponseId",
        optional,
        tag = "11"
    )]
    pub activate_scratchpad_response: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "12")]
    pub load_scratchpad_response_time_s_epoch: ::core::option::Option<u32>,
}
/// Nested message and enum types in `SetOTAPStateResponse`.
pub mod set_otap_state_response {
    /// Same as gateway ErrorCode
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
    pub enum OtapResponseId {
        UnknownErrorCode = -1,
        Ok = 0,
        InternalError = 1,
        InvalidSinkId = 2,
        InvalidRole = 3,
        InvalidNetworkAddress = 4,
        InvalidNetworkChannel = 5,
        InvalidChannelMap = 6,
        InvalidNetworkKeys = 7,
        InvalidAcRange = 8,
        InvalidSinkState = 9,
        InvalidDestAddress = 10,
        InvalidDestEndpoint = 11,
        InvalidSrcEndpoint = 12,
        InvalidQos = 13,
        InvalidDataPayload = 14,
        InvalidScratchpad = 15,
        InvalidScratchpadSize = 16,
        InvalidSequenceNumber = 17,
        InvalidRebootDelay = 18,
        InvalidDiagInterval = 19,
        InvalidAppConfig = 20,
        InvalidParam = 21,
        NoScratchpadPresent = 22,
        AccessDenied = 23,
        RequestNeedsSinkId = 24,
        InvalidMaxHopCount = 25,
        SinkOutOfMemory = 26,
        SinkTimeout = 27,
    }
    impl OtapResponseId {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                OtapResponseId::UnknownErrorCode => "UNKNOWN_ERROR_CODE",
                OtapResponseId::Ok => "OK",
                OtapResponseId::InternalError => "INTERNAL_ERROR",
                OtapResponseId::InvalidSinkId => "INVALID_SINK_ID",
                OtapResponseId::InvalidRole => "INVALID_ROLE",
                OtapResponseId::InvalidNetworkAddress => "INVALID_NETWORK_ADDRESS",
                OtapResponseId::InvalidNetworkChannel => "INVALID_NETWORK_CHANNEL",
                OtapResponseId::InvalidChannelMap => "INVALID_CHANNEL_MAP",
                OtapResponseId::InvalidNetworkKeys => "INVALID_NETWORK_KEYS",
                OtapResponseId::InvalidAcRange => "INVALID_AC_RANGE",
                OtapResponseId::InvalidSinkState => "INVALID_SINK_STATE",
                OtapResponseId::InvalidDestAddress => "INVALID_DEST_ADDRESS",
                OtapResponseId::InvalidDestEndpoint => "INVALID_DEST_ENDPOINT",
                OtapResponseId::InvalidSrcEndpoint => "INVALID_SRC_ENDPOINT",
                OtapResponseId::InvalidQos => "INVALID_QOS",
                OtapResponseId::InvalidDataPayload => "INVALID_DATA_PAYLOAD",
                OtapResponseId::InvalidScratchpad => "INVALID_SCRATCHPAD",
                OtapResponseId::InvalidScratchpadSize => "INVALID_SCRATCHPAD_SIZE",
                OtapResponseId::InvalidSequenceNumber => "INVALID_SEQUENCE_NUMBER",
                OtapResponseId::InvalidRebootDelay => "INVALID_REBOOT_DELAY",
                OtapResponseId::InvalidDiagInterval => "INVALID_DIAG_INTERVAL",
                OtapResponseId::InvalidAppConfig => "INVALID_APP_CONFIG",
                OtapResponseId::InvalidParam => "INVALID_PARAM",
                OtapResponseId::NoScratchpadPresent => "NO_SCRATCHPAD_PRESENT",
                OtapResponseId::AccessDenied => "ACCESS_DENIED",
                OtapResponseId::RequestNeedsSinkId => "REQUEST_NEEDS_SINK_ID",
                OtapResponseId::InvalidMaxHopCount => "INVALID_MAX_HOP_COUNT",
                OtapResponseId::SinkOutOfMemory => "SINK_OUT_OF_MEMORY",
                OtapResponseId::SinkTimeout => "SINK_TIMEOUT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNKNOWN_ERROR_CODE" => Some(Self::UnknownErrorCode),
                "OK" => Some(Self::Ok),
                "INTERNAL_ERROR" => Some(Self::InternalError),
                "INVALID_SINK_ID" => Some(Self::InvalidSinkId),
                "INVALID_ROLE" => Some(Self::InvalidRole),
                "INVALID_NETWORK_ADDRESS" => Some(Self::InvalidNetworkAddress),
                "INVALID_NETWORK_CHANNEL" => Some(Self::InvalidNetworkChannel),
                "INVALID_CHANNEL_MAP" => Some(Self::InvalidChannelMap),
                "INVALID_NETWORK_KEYS" => Some(Self::InvalidNetworkKeys),
                "INVALID_AC_RANGE" => Some(Self::InvalidAcRange),
                "INVALID_SINK_STATE" => Some(Self::InvalidSinkState),
                "INVALID_DEST_ADDRESS" => Some(Self::InvalidDestAddress),
                "INVALID_DEST_ENDPOINT" => Some(Self::InvalidDestEndpoint),
                "INVALID_SRC_ENDPOINT" => Some(Self::InvalidSrcEndpoint),
                "INVALID_QOS" => Some(Self::InvalidQos),
                "INVALID_DATA_PAYLOAD" => Some(Self::InvalidDataPayload),
                "INVALID_SCRATCHPAD" => Some(Self::InvalidScratchpad),
                "INVALID_SCRATCHPAD_SIZE" => Some(Self::InvalidScratchpadSize),
                "INVALID_SEQUENCE_NUMBER" => Some(Self::InvalidSequenceNumber),
                "INVALID_REBOOT_DELAY" => Some(Self::InvalidRebootDelay),
                "INVALID_DIAG_INTERVAL" => Some(Self::InvalidDiagInterval),
                "INVALID_APP_CONFIG" => Some(Self::InvalidAppConfig),
                "INVALID_PARAM" => Some(Self::InvalidParam),
                "NO_SCRATCHPAD_PRESENT" => Some(Self::NoScratchpadPresent),
                "ACCESS_DENIED" => Some(Self::AccessDenied),
                "REQUEST_NEEDS_SINK_ID" => Some(Self::RequestNeedsSinkId),
                "INVALID_MAX_HOP_COUNT" => Some(Self::InvalidMaxHopCount),
                "SINK_OUT_OF_MEMORY" => Some(Self::SinkOutOfMemory),
                "SINK_TIMEOUT" => Some(Self::SinkTimeout),
                _ => None,
            }
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OtapState {
    None = 0,
    /// For UI only
    WaitForStart = 1,
    ScratchpadStatusQuery = 2,
    LoadScratchpad = 3,
    Activate = 4,
    NpadLoadAndActivate = 5,
    /// For backend only
    WaitForActivation = 6,
    Close = 7,
}
impl OtapState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OtapState::None => "OTAP_STATE_NONE",
            OtapState::WaitForStart => "OTAP_STATE_WAIT_FOR_START",
            OtapState::ScratchpadStatusQuery => "OTAP_STATE_SCRATCHPAD_STATUS_QUERY",
            OtapState::LoadScratchpad => "OTAP_STATE_LOAD_SCRATCHPAD",
            OtapState::Activate => "OTAP_STATE_ACTIVATE",
            OtapState::NpadLoadAndActivate => "OTAP_STATE_NPAD_LOAD_AND_ACTIVATE",
            OtapState::WaitForActivation => "OTAP_STATE_WAIT_FOR_ACTIVATION",
            OtapState::Close => "OTAP_STATE_CLOSE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OTAP_STATE_NONE" => Some(Self::None),
            "OTAP_STATE_WAIT_FOR_START" => Some(Self::WaitForStart),
            "OTAP_STATE_SCRATCHPAD_STATUS_QUERY" => Some(Self::ScratchpadStatusQuery),
            "OTAP_STATE_LOAD_SCRATCHPAD" => Some(Self::LoadScratchpad),
            "OTAP_STATE_ACTIVATE" => Some(Self::Activate),
            "OTAP_STATE_NPAD_LOAD_AND_ACTIVATE" => Some(Self::NpadLoadAndActivate),
            "OTAP_STATE_WAIT_FOR_ACTIVATION" => Some(Self::WaitForActivation),
            "OTAP_STATE_CLOSE" => Some(Self::Close),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OtapMethod {
    None = 0,
    Legacy = 1,
    Npad = 2,
}
impl OtapMethod {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OtapMethod::None => "OTAP_METHOD_NONE",
            OtapMethod::Legacy => "OTAP_METHOD_LEGACY",
            OtapMethod::Npad => "OTAP_METHOD_NPAD",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OTAP_METHOD_NONE" => Some(Self::None),
            "OTAP_METHOD_LEGACY" => Some(Self::Legacy),
            "OTAP_METHOD_NPAD" => Some(Self::Npad),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OtapnpadDelay {
    OtapNpadDelayNone = 0,
    OtapNpadDelayTenMinutes = 1,
    OtapNpadDelayThirtyMinutes = 2,
    OtapNpadDelayOneHour = 3,
    OtapNpadDelaySixHours = 4,
    OtapNpadDelayOneDay = 5,
    OtapNpadDelayTwoDays = 6,
    OtapNpadDelayFiveDays = 7,
}
impl OtapnpadDelay {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OtapnpadDelay::OtapNpadDelayNone => "OTAP_NPAD_DELAY_NONE",
            OtapnpadDelay::OtapNpadDelayTenMinutes => "OTAP_NPAD_DELAY_TEN_MINUTES",
            OtapnpadDelay::OtapNpadDelayThirtyMinutes => "OTAP_NPAD_DELAY_THIRTY_MINUTES",
            OtapnpadDelay::OtapNpadDelayOneHour => "OTAP_NPAD_DELAY_ONE_HOUR",
            OtapnpadDelay::OtapNpadDelaySixHours => "OTAP_NPAD_DELAY_SIX_HOURS",
            OtapnpadDelay::OtapNpadDelayOneDay => "OTAP_NPAD_DELAY_ONE_DAY",
            OtapnpadDelay::OtapNpadDelayTwoDays => "OTAP_NPAD_DELAY_TWO_DAYS",
            OtapnpadDelay::OtapNpadDelayFiveDays => "OTAP_NPAD_DELAY_FIVE_DAYS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OTAP_NPAD_DELAY_NONE" => Some(Self::OtapNpadDelayNone),
            "OTAP_NPAD_DELAY_TEN_MINUTES" => Some(Self::OtapNpadDelayTenMinutes),
            "OTAP_NPAD_DELAY_THIRTY_MINUTES" => Some(Self::OtapNpadDelayThirtyMinutes),
            "OTAP_NPAD_DELAY_ONE_HOUR" => Some(Self::OtapNpadDelayOneHour),
            "OTAP_NPAD_DELAY_SIX_HOURS" => Some(Self::OtapNpadDelaySixHours),
            "OTAP_NPAD_DELAY_ONE_DAY" => Some(Self::OtapNpadDelayOneDay),
            "OTAP_NPAD_DELAY_TWO_DAYS" => Some(Self::OtapNpadDelayTwoDays),
            "OTAP_NPAD_DELAY_FIVE_DAYS" => Some(Self::OtapNpadDelayFiveDays),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeMetadata {
    #[prost(double, optional, tag = "1")]
    pub latitude: ::core::option::Option<f64>,
    #[prost(double, optional, tag = "2")]
    pub longitude: ::core::option::Option<f64>,
    #[prost(double, optional, tag = "3")]
    pub altitude: ::core::option::Option<f64>,
    /// Floor plan id
    #[prost(string, repeated, tag = "10")]
    pub map_uuid: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Obsolete for WNT >= 2.0
    /// optional uint32 pixel_location_x = 11;
    /// optional uint32 pixel_location_y = 12;
    #[prost(string, repeated, tag = "13")]
    pub area_uuid: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "14")]
    pub is_area_uuid_empty: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "20")]
    pub is_approved: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "21")]
    pub is_deleted: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "22")]
    pub is_map_uuid_empty: ::core::option::Option<bool>,
    /// Planning node
    #[prost(bool, optional, tag = "23")]
    pub is_virtual: ::core::option::Option<bool>,
    #[prost(enumeration = "node_metadata::PositioningRole", optional, tag = "30")]
    pub positioning_role: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "31")]
    pub rssi_offset: ::core::option::Option<i32>,
    /// Epoch ms
    #[prost(uint64, optional, tag = "40")]
    pub wpe_scan_time: ::core::option::Option<u64>,
    /// Epoch ms
    #[prost(uint64, optional, tag = "41")]
    pub wpe_calculation_time: ::core::option::Option<u64>,
    #[prost(string, optional, tag = "50")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "51")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    /// Epoch
    #[prost(uint32, optional, tag = "60")]
    pub update_time: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "255")]
    pub originator_token: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `NodeMetadata`.
pub mod node_metadata {
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
    pub enum PositioningRole {
        Unknown = 0,
        Anchor = 1,
        Tag = 2,
    }
    impl PositioningRole {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PositioningRole::Unknown => "UNKNOWN",
                PositioningRole::Anchor => "ANCHOR",
                PositioningRole::Tag => "TAG",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNKNOWN" => Some(Self::Unknown),
                "ANCHOR" => Some(Self::Anchor),
                "TAG" => Some(Self::Tag),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RtSituationMetadata {
    /// Cluster cell number
    #[prost(uint32, optional, tag = "1")]
    pub cluster_no: ::core::option::Option<u32>,
    /// Total count of cells in the cluster
    #[prost(uint32, optional, tag = "2")]
    pub cluster_size: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub node_count: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "10")]
    pub backend_version: ::core::option::Option<::prost::alloc::string::String>,
}
/// Used for backend internal messaging
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BackendMessage {
    #[prost(enumeration = "backend_message::MessageType", optional, tag = "1")]
    pub message: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "2")]
    pub client_id: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "255")]
    pub originator_token: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `BackendMessage`.
pub mod backend_message {
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
    pub enum MessageType {
        Delta = 1,
        Full = 2,
        RtSituationMetadata = 3,
        RtSituationDeleteNode = 4,
        RtSituationRemoteApiConfigurationStarted = 5,
        RtSituationRemoteApiActivationStarted = 6,
        RtSituationRemoteApiCancellationStarted = 7,
        RtSituationMetadataUpdate = 8,
        RtSituationSendPositioningData = 9,
        RtSituationComponentInformation = 10,
        RtSituationQueryNodeMetadata = 11,
        RtSituationQueryNetworkMetadata = 12,
        RtSituationQueryBuildingAndFloorPlanMetadata = 13,
        RtSituationQueryAreaMetadata = 14,
    }
    impl MessageType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                MessageType::Delta => "Delta",
                MessageType::Full => "Full",
                MessageType::RtSituationMetadata => "RTSituationMetadata",
                MessageType::RtSituationDeleteNode => "RTSituationDeleteNode",
                MessageType::RtSituationRemoteApiConfigurationStarted => {
                    "RTSituationRemoteApiConfigurationStarted"
                }
                MessageType::RtSituationRemoteApiActivationStarted => {
                    "RTSituationRemoteApiActivationStarted"
                }
                MessageType::RtSituationRemoteApiCancellationStarted => {
                    "RTSituationRemoteApiCancellationStarted"
                }
                MessageType::RtSituationMetadataUpdate => "RTSituationMetadataUpdate",
                MessageType::RtSituationSendPositioningData => {
                    "RTSituationSendPositioningData"
                }
                MessageType::RtSituationComponentInformation => {
                    "RTSituationComponentInformation"
                }
                MessageType::RtSituationQueryNodeMetadata => {
                    "RTSituationQueryNodeMetadata"
                }
                MessageType::RtSituationQueryNetworkMetadata => {
                    "RTSituationQueryNetworkMetadata"
                }
                MessageType::RtSituationQueryBuildingAndFloorPlanMetadata => {
                    "RTSituationQueryBuildingAndFloorPlanMetadata"
                }
                MessageType::RtSituationQueryAreaMetadata => {
                    "RTSituationQueryAreaMetadata"
                }
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "Delta" => Some(Self::Delta),
                "Full" => Some(Self::Full),
                "RTSituationMetadata" => Some(Self::RtSituationMetadata),
                "RTSituationDeleteNode" => Some(Self::RtSituationDeleteNode),
                "RTSituationRemoteApiConfigurationStarted" => {
                    Some(Self::RtSituationRemoteApiConfigurationStarted)
                }
                "RTSituationRemoteApiActivationStarted" => {
                    Some(Self::RtSituationRemoteApiActivationStarted)
                }
                "RTSituationRemoteApiCancellationStarted" => {
                    Some(Self::RtSituationRemoteApiCancellationStarted)
                }
                "RTSituationMetadataUpdate" => Some(Self::RtSituationMetadataUpdate),
                "RTSituationSendPositioningData" => {
                    Some(Self::RtSituationSendPositioningData)
                }
                "RTSituationComponentInformation" => {
                    Some(Self::RtSituationComponentInformation)
                }
                "RTSituationQueryNodeMetadata" => {
                    Some(Self::RtSituationQueryNodeMetadata)
                }
                "RTSituationQueryNetworkMetadata" => {
                    Some(Self::RtSituationQueryNetworkMetadata)
                }
                "RTSituationQueryBuildingAndFloorPlanMetadata" => {
                    Some(Self::RtSituationQueryBuildingAndFloorPlanMetadata)
                }
                "RTSituationQueryAreaMetadata" => {
                    Some(Self::RtSituationQueryAreaMetadata)
                }
                _ => None,
            }
        }
    }
}
/// Node online status
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OnlineStatus {
    #[prost(enumeration = "online_status::Status", optional, tag = "1")]
    pub status: ::core::option::Option<i32>,
    /// Epoch
    #[prost(uint32, optional, tag = "2")]
    pub update_time: ::core::option::Option<u32>,
    /// Epoch
    #[prost(uint32, optional, tag = "3")]
    pub last_time_series_write: ::core::option::Option<u32>,
    #[prost(enumeration = "online_status::Status", optional, tag = "4")]
    pub previous_status: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "10")]
    pub is_sink_online_in_gateway: ::core::option::Option<bool>,
}
/// Nested message and enum types in `OnlineStatus`.
pub mod online_status {
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
    pub enum Status {
        Offline = 0,
        Uncertain = 1,
        Online = 2,
    }
    impl Status {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Status::Offline => "OFFLINE",
                Status::Uncertain => "UNCERTAIN",
                Status::Online => "ONLINE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "OFFLINE" => Some(Self::Offline),
                "UNCERTAIN" => Some(Self::Uncertain),
                "ONLINE" => Some(Self::Online),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TraveltimeKpi {
    #[prost(float, optional, tag = "1")]
    pub qos0_minimum: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "2")]
    pub qos0_average: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "3")]
    pub qos0_maximum: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "11")]
    pub qos1_minimum: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "12")]
    pub qos1_average: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "13")]
    pub qos1_maximum: ::core::option::Option<f32>,
    #[prost(int32, optional, tag = "20")]
    pub window_duration: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BackendHeartbeat {
    #[prost(enumeration = "BackendComponent", optional, tag = "1")]
    pub sender: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "2")]
    pub cluster_no: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3", default = "60")]
    pub interval_s: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BackendComponentStatus {
    #[prost(enumeration = "BackendComponent", optional, tag = "1")]
    pub component: ::core::option::Option<i32>,
    #[prost(enumeration = "RunningStatus", optional, tag = "2")]
    pub status: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Network {
    #[prost(uint32, optional, tag = "1")]
    pub id: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "3")]
    pub update_time: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "4")]
    pub is_delete_nodes: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct User {
    #[prost(enumeration = "user::Role", optional, tag = "1")]
    pub role: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub user_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub full_name: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `User`.
pub mod user {
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
    pub enum Role {
        Admin = 1,
        User = 2,
    }
    impl Role {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Role::Admin => "Admin",
                Role::User => "User",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "Admin" => Some(Self::Admin),
                "User" => Some(Self::User),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Building {
    #[prost(string, optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "3")]
    pub update_time: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FloorplanCorner {
    #[prost(double, optional, tag = "1")]
    pub latitude: ::core::option::Option<f64>,
    #[prost(double, optional, tag = "2")]
    pub longitude: ::core::option::Option<f64>,
    #[prost(double, optional, tag = "3")]
    pub altitude: ::core::option::Option<f64>,
    #[prost(double, optional, tag = "4")]
    pub x_anchor: ::core::option::Option<f64>,
    #[prost(double, optional, tag = "5")]
    pub y_anchor: ::core::option::Option<f64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct XyPoint {
    #[prost(double, optional, tag = "1")]
    pub x: ::core::option::Option<f64>,
    #[prost(double, optional, tag = "2")]
    pub y: ::core::option::Option<f64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct XyzPoint {
    #[prost(double, optional, tag = "1")]
    pub x: ::core::option::Option<f64>,
    #[prost(double, optional, tag = "2")]
    pub y: ::core::option::Option<f64>,
    #[prost(double, optional, tag = "3")]
    pub z: ::core::option::Option<f64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Matrix {
    #[prost(double, optional, tag = "1")]
    pub m11: ::core::option::Option<f64>,
    #[prost(double, optional, tag = "2")]
    pub m12: ::core::option::Option<f64>,
    #[prost(double, optional, tag = "3")]
    pub m13: ::core::option::Option<f64>,
    #[prost(double, optional, tag = "4")]
    pub m21: ::core::option::Option<f64>,
    #[prost(double, optional, tag = "5")]
    pub m22: ::core::option::Option<f64>,
    #[prost(double, optional, tag = "6")]
    pub m23: ::core::option::Option<f64>,
    #[prost(double, optional, tag = "7")]
    pub m31: ::core::option::Option<f64>,
    #[prost(double, optional, tag = "8")]
    pub m32: ::core::option::Option<f64>,
    #[prost(double, optional, tag = "9")]
    pub m33: ::core::option::Option<f64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FloorPlan {
    #[prost(string, optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub building_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "4")]
    pub level: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "11")]
    pub image_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "12")]
    pub image_thumbnail_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "13")]
    pub image_width: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "14")]
    pub image_height: ::core::option::Option<u32>,
    #[prost(message, optional, tag = "21")]
    pub left_top_corner: ::core::option::Option<FloorplanCorner>,
    #[prost(message, optional, tag = "22")]
    pub right_top_corner: ::core::option::Option<FloorplanCorner>,
    #[prost(message, optional, tag = "23")]
    pub left_bottom_corner: ::core::option::Option<FloorplanCorner>,
    #[prost(message, optional, tag = "24")]
    pub right_bottom_corner: ::core::option::Option<FloorplanCorner>,
    #[prost(message, optional, tag = "31")]
    pub distance_point_1: ::core::option::Option<XyPoint>,
    #[prost(message, optional, tag = "32")]
    pub distance_point_2: ::core::option::Option<XyPoint>,
    #[prost(double, optional, tag = "33")]
    pub distance_in_m: ::core::option::Option<f64>,
    #[prost(message, optional, tag = "40")]
    pub rotation_matrix: ::core::option::Option<Matrix>,
    #[prost(message, optional, tag = "41")]
    pub offset_ecef_to_local: ::core::option::Option<XyzPoint>,
    #[prost(message, optional, tag = "42")]
    pub offset_local_to_ecef: ::core::option::Option<XyzPoint>,
    #[prost(double, optional, tag = "43")]
    pub pixels_per_meter: ::core::option::Option<f64>,
    #[prost(uint32, optional, tag = "50")]
    pub update_time: ::core::option::Option<u32>,
}
/// Alpha, red, green and blue
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Argb {
    #[prost(uint32, optional, tag = "1")]
    pub a: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub r: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub g: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub b: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Lla {
    #[prost(double, optional, tag = "1")]
    pub latitude: ::core::option::Option<f64>,
    #[prost(double, optional, tag = "2")]
    pub longitude: ::core::option::Option<f64>,
    #[prost(double, optional, tag = "3")]
    pub altitude: ::core::option::Option<f64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Area {
    #[prost(string, optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub floor_plan_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "4")]
    pub color: ::core::option::Option<Argb>,
    #[prost(message, repeated, tag = "10")]
    pub polygon_points: ::prost::alloc::vec::Vec<Lla>,
    #[prost(uint32, optional, tag = "20")]
    pub update_time: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetadataUpdateMessage {
    #[prost(message, repeated, tag = "1")]
    pub added_or_changed_networks: ::prost::alloc::vec::Vec<Network>,
    #[prost(message, repeated, tag = "2")]
    pub deleted_networks: ::prost::alloc::vec::Vec<Network>,
    #[prost(message, repeated, tag = "11")]
    pub added_or_changed_users: ::prost::alloc::vec::Vec<User>,
    #[prost(message, repeated, tag = "12")]
    pub deleted_users: ::prost::alloc::vec::Vec<User>,
    #[prost(message, repeated, tag = "21")]
    pub added_or_changed_buildings: ::prost::alloc::vec::Vec<Building>,
    #[prost(message, repeated, tag = "22")]
    pub deleted_buildings: ::prost::alloc::vec::Vec<Building>,
    #[prost(message, repeated, tag = "31")]
    pub added_or_changed_floor_plans: ::prost::alloc::vec::Vec<FloorPlan>,
    #[prost(message, repeated, tag = "32")]
    pub deleted_floor_plans: ::prost::alloc::vec::Vec<FloorPlan>,
    #[prost(message, repeated, tag = "41")]
    pub added_or_changed_areas: ::prost::alloc::vec::Vec<Area>,
    #[prost(message, repeated, tag = "42")]
    pub deleted_areas: ::prost::alloc::vec::Vec<Area>,
    #[prost(string, optional, tag = "255")]
    pub originator_token: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SinkPseudoIdMap {
    #[prost(string, optional, tag = "1")]
    pub pseudo_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "10")]
    pub network_id: ::core::option::Option<u32>,
    /// Node id
    #[prost(uint32, optional, tag = "11")]
    pub address: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "12")]
    pub gateway_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "13")]
    pub gateway_sink_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "20")]
    pub app_config: ::core::option::Option<AppConfigData>,
    #[prost(uint32, optional, tag = "21")]
    pub stored_scratchpad_sequence: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag = "22")]
    pub stored_scratchpad_sequence_update_time_ms_epoch: ::core::option::Option<u64>,
    #[prost(enumeration = "ScratchpadAction", optional, tag = "23")]
    pub scratchpad_action: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "24")]
    pub target_sequence: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "25")]
    pub target_crc: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "26")]
    pub target_delay_minutes: ::core::option::Option<u32>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BackendComponent {
    AuthManager = 1,
    DiagnosticsInjector = 2,
    Ferouter = 3,
    MetadataManager = 4,
    Parser = 5,
    Rtsituation = 6,
    TimeseriesManager = 7,
    GatewayCommunicator = 8,
}
impl BackendComponent {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            BackendComponent::AuthManager => "AUTH_MANAGER",
            BackendComponent::DiagnosticsInjector => "DIAGNOSTICS_INJECTOR",
            BackendComponent::Ferouter => "FEROUTER",
            BackendComponent::MetadataManager => "METADATA_MANAGER",
            BackendComponent::Parser => "PARSER",
            BackendComponent::Rtsituation => "RTSITUATION",
            BackendComponent::TimeseriesManager => "TIMESERIES_MANAGER",
            BackendComponent::GatewayCommunicator => "GATEWAY_COMMUNICATOR",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "AUTH_MANAGER" => Some(Self::AuthManager),
            "DIAGNOSTICS_INJECTOR" => Some(Self::DiagnosticsInjector),
            "FEROUTER" => Some(Self::Ferouter),
            "METADATA_MANAGER" => Some(Self::MetadataManager),
            "PARSER" => Some(Self::Parser),
            "RTSITUATION" => Some(Self::Rtsituation),
            "TIMESERIES_MANAGER" => Some(Self::TimeseriesManager),
            "GATEWAY_COMMUNICATOR" => Some(Self::GatewayCommunicator),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RunningStatus {
    Starting = 1,
    Running = 10,
    RunningAndAcknowledged = 11,
    Closed = 20,
    Unknown = 255,
}
impl RunningStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RunningStatus::Starting => "STARTING",
            RunningStatus::Running => "RUNNING",
            RunningStatus::RunningAndAcknowledged => "RUNNING_AND_ACKNOWLEDGED",
            RunningStatus::Closed => "CLOSED",
            RunningStatus::Unknown => "UNKNOWN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "STARTING" => Some(Self::Starting),
            "RUNNING" => Some(Self::Running),
            "RUNNING_AND_ACKNOWLEDGED" => Some(Self::RunningAndAcknowledged),
            "CLOSED" => Some(Self::Closed),
            "UNKNOWN" => Some(Self::Unknown),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoteApiRequest {
    /// To which devices this request stands for
    #[prost(uint32, repeated, packed = "false", tag = "1")]
    pub addresses: ::prost::alloc::vec::Vec<u32>,
    /// Sink address to use when sendind data via unicast
    #[prost(uint32, optional, tag = "2")]
    pub sink_address: ::core::option::Option<u32>,
    /// Changing the role
    #[prost(enumeration = "BaseRole", optional, tag = "10")]
    pub role: ::core::option::Option<i32>,
    /// True = low latency, false = low energy
    #[prost(bool, optional, tag = "11")]
    pub is_cb_mac: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "12")]
    pub is_autorole: ::core::option::Option<bool>,
    #[prost(uint32, optional, tag = "13")]
    pub network_address: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "14")]
    pub network_channel: ::core::option::Option<u32>,
    #[prost(bytes = "vec", optional, tag = "15")]
    pub cipher_key: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes = "vec", optional, tag = "16")]
    pub authentication_key: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    /// optional uint32 channel_map = 17;  Not supported anymore as for WM FW 3.x
    #[prost(uint32, optional, tag = "18")]
    pub node_address: ::core::option::Option<u32>,
}
/// This is a collection of remote messages currently undergoing
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoteApiRequestCollection {
    /// UUID
    #[prost(string, optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(enumeration = "RemoteApiRequestType", optional, tag = "2")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "3")]
    pub messages: ::prost::alloc::vec::Vec<RemoteApiRequest>,
    /// Interval to resend the requests
    #[prost(uint32, optional, tag = "4")]
    pub resend_interval_s: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "5")]
    pub is_cancel: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "6")]
    pub is_close: ::core::option::Option<bool>,
    #[prost(enumeration = "MessageSendingType", optional, tag = "7")]
    pub message_sending_type: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "10")]
    pub start_time_s_epoch: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "11")]
    pub activation_time_s_epoch: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "12")]
    pub cancellation_time_s_epoch: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoteApiResponse {
    #[prost(uint32, optional, tag = "1")]
    pub start_time_s_epoch: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub activation_time_s_epoch: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub cancellation_time_s_epoch: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "11")]
    pub metadata_update_time_s_epoch: ::core::option::Option<u32>,
    #[prost(enumeration = "RemoteApiResponseId", optional, tag = "21")]
    pub configure_result: ::core::option::Option<i32>,
    #[prost(enumeration = "RemoteApiResponseId", optional, tag = "22")]
    pub activate_result: ::core::option::Option<i32>,
    #[prost(enumeration = "RemoteApiResponseId", optional, tag = "23")]
    pub cancel_result: ::core::option::Option<i32>,
    #[prost(enumeration = "RemoteApiResponseId", optional, tag = "31")]
    pub metadata_update_result: ::core::option::Option<i32>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RemoteApiRequestType {
    RemoteApiTypeBroadcast = 0,
    RemoteApiTypeUnicast = 1,
}
impl RemoteApiRequestType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RemoteApiRequestType::RemoteApiTypeBroadcast => "REMOTE_API_TYPE_BROADCAST",
            RemoteApiRequestType::RemoteApiTypeUnicast => "REMOTE_API_TYPE_UNICAST",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "REMOTE_API_TYPE_BROADCAST" => Some(Self::RemoteApiTypeBroadcast),
            "REMOTE_API_TYPE_UNICAST" => Some(Self::RemoteApiTypeUnicast),
            _ => None,
        }
    }
}
/// This is a response for remote api responses
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RemoteApiResponseId {
    Ok = 0,
    AccessDenied = 1,
    WriteOnlyAttribute = 2,
    InvalidBcRequest = 3,
    InvalidBegin = 4,
    NoSpaceForResponse = 5,
    InvalidValue = 6,
    InvalidLength = 7,
    UnknownRequest = 8,
    UnknownError = 255,
}
impl RemoteApiResponseId {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RemoteApiResponseId::Ok => "OK",
            RemoteApiResponseId::AccessDenied => "ACCESS_DENIED",
            RemoteApiResponseId::WriteOnlyAttribute => "WRITE_ONLY_ATTRIBUTE",
            RemoteApiResponseId::InvalidBcRequest => "INVALID_BC_REQUEST",
            RemoteApiResponseId::InvalidBegin => "INVALID_BEGIN",
            RemoteApiResponseId::NoSpaceForResponse => "NO_SPACE_FOR_RESPONSE",
            RemoteApiResponseId::InvalidValue => "INVALID_VALUE",
            RemoteApiResponseId::InvalidLength => "INVALID_LENGTH",
            RemoteApiResponseId::UnknownRequest => "UNKNOWN_REQUEST",
            RemoteApiResponseId::UnknownError => "UNKNOWN_ERROR",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OK" => Some(Self::Ok),
            "ACCESS_DENIED" => Some(Self::AccessDenied),
            "WRITE_ONLY_ATTRIBUTE" => Some(Self::WriteOnlyAttribute),
            "INVALID_BC_REQUEST" => Some(Self::InvalidBcRequest),
            "INVALID_BEGIN" => Some(Self::InvalidBegin),
            "NO_SPACE_FOR_RESPONSE" => Some(Self::NoSpaceForResponse),
            "INVALID_VALUE" => Some(Self::InvalidValue),
            "INVALID_LENGTH" => Some(Self::InvalidLength),
            "UNKNOWN_REQUEST" => Some(Self::UnknownRequest),
            "UNKNOWN_ERROR" => Some(Self::UnknownError),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Neighbor {
    /// This equals neighbor diagnostics
    #[prost(uint32, optional, tag = "1")]
    pub address: ::core::option::Option<u32>,
    #[prost(float, optional, tag = "2")]
    pub cluster_channel_m_hz: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "3")]
    pub radio_power_d_b: ::core::option::Option<f32>,
    #[prost(enumeration = "neighbor::NeighborType", optional, tag = "4")]
    pub neighbor_type: ::core::option::Option<i32>,
    #[prost(float, optional, tag = "5")]
    pub rssi_d_bm: ::core::option::Option<f32>,
    /// This is special value to create 1-size list when no neighbors exist
    #[prost(bool, optional, tag = "6")]
    pub no_neighbors: ::core::option::Option<bool>,
    #[prost(uint32, optional, tag = "7")]
    pub cluster_channel: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "8")]
    pub amount_of_cluster_beacon_reception_attempts: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "9")]
    pub amount_of_failed_cluster_beacon_reception_attempts: ::core::option::Option<u32>,
}
/// Nested message and enum types in `Neighbor`.
pub mod neighbor {
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
    pub enum NeighborType {
        Member = 1,
        /// Alternative route
        SyncCluster = 2,
        /// Next hop
        AssociatedCluster = 3,
        /// Heard from scan/SNDP
        Unsynced = 4,
        Unknown = 255,
    }
    impl NeighborType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                NeighborType::Member => "MEMBER",
                NeighborType::SyncCluster => "SYNC_CLUSTER",
                NeighborType::AssociatedCluster => "ASSOCIATED_CLUSTER",
                NeighborType::Unsynced => "UNSYNCED",
                NeighborType::Unknown => "NEIGHBOR_TYPE_UNKNOWN",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "MEMBER" => Some(Self::Member),
                "SYNC_CLUSTER" => Some(Self::SyncCluster),
                "ASSOCIATED_CLUSTER" => Some(Self::AssociatedCluster),
                "UNSYNCED" => Some(Self::Unsynced),
                "NEIGHBOR_TYPE_UNKNOWN" => Some(Self::Unknown),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CostInfo {
    /// Cost info structure, used in node diagnostics
    #[prost(uint32, optional, tag = "2")]
    pub next_hop_address: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub cost: ::core::option::Option<u32>,
    /// Percentage
    #[prost(float, optional, tag = "4")]
    pub quality: ::core::option::Option<f32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CalculatedValuesFromDiagnostics {
    #[prost(float, optional, tag = "1")]
    pub low_energy_transmission_load_percentage: ::core::option::Option<f32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiagnosticsData {
    /// This message is general purpose container.
    /// This does NOT distinguish different types of diagnostics packets!
    /// Info currently sent in neighbor diagnostics
    #[prost(message, repeated, tag = "1")]
    pub neighbors: ::prost::alloc::vec::Vec<Neighbor>,
    /// Info currently sent in node diagnostics
    #[prost(float, optional, tag = "2")]
    pub access_cycle_ms: ::core::option::Option<f32>,
    #[prost(enumeration = "BaseRole", optional, tag = "3")]
    pub role: ::core::option::Option<i32>,
    /// True = low latency, false = low energy
    #[prost(bool, optional, tag = "4")]
    pub cb_mac: ::core::option::Option<bool>,
    /// Not supported - optional bool is_relay = 5;
    #[prost(bool, optional, tag = "6")]
    pub is_autorole: ::core::option::Option<bool>,
    /// Volts
    #[prost(float, optional, tag = "7")]
    pub voltage: ::core::option::Option<f32>,
    /// Percentage
    #[prost(float, optional, tag = "8")]
    pub max_buffer_usage: ::core::option::Option<f32>,
    /// Percentage
    #[prost(float, optional, tag = "9")]
    pub average_buffer_usage: ::core::option::Option<f32>,
    #[prost(uint32, optional, tag = "10")]
    pub mem_alloc_fails: ::core::option::Option<u32>,
    /// WM FW 3.x not supported anymore - repeated float buf_delay_ms = 11 [packed=true, (nanopb).max_count = 2]; // By qos
    ///
    /// WM FW 3.x not supported anymore - repeated float dl_average_ms = 13 [packed=true, (nanopb).max_count = 2]; // By qos
    /// WM FW 3.x not supported anymore - repeated float dl_minimum_ms = 14 [packed=true, (nanopb).max_count = 2]; // By qos
    /// WM FW 3.x not supported anymore - repeated float dl_maximum_ms = 15 [packed=true, (nanopb).max_count = 2]; // By qos
    /// WM FW 3.x not supported anymore - repeated uint32 dl_samples = 16 [packed=true, (nanopb).max_count = 2]; // By qos
    #[prost(uint32, optional, tag = "12")]
    pub scans: ::core::option::Option<u32>,
    /// Not valid for WM version >= 5.0 as there is one combined value for dropped packets
    ///
    /// By qos
    #[prost(uint32, repeated, tag = "17")]
    pub dropped: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, optional, tag = "49")]
    pub sink_address: ::core::option::Option<u32>,
    /// By qos
    #[prost(message, repeated, tag = "18")]
    pub cost: ::prost::alloc::vec::Vec<CostInfo>,
    #[prost(enumeration = "diagnostics_data::Events", repeated, tag = "19")]
    pub events: ::prost::alloc::vec::Vec<i32>,
    /// Percentage
    #[prost(float, optional, tag = "20")]
    pub duty_cycle: ::core::option::Option<f32>,
    #[prost(uint32, optional, tag = "21")]
    pub antenna: ::core::option::Option<u32>,
    /// WM FW 3.x not supported anymore - optional uint32 access_cycles = 22;
    #[prost(float, optional, tag = "23")]
    pub cluster_channel_m_hz: ::core::option::Option<f32>,
    /// Percentage
    #[prost(float, optional, tag = "24")]
    pub channel_reliability: ::core::option::Option<f32>,
    /// WM FW 3.x not supported anymore - optional uint32 rx_amount = 25;
    /// WM FW 3.x not supported anymore - optional uint32 tx_amount = 26;
    ///
    /// Percentage
    #[prost(float, optional, tag = "27")]
    pub aloha_rx_ratio: ::core::option::Option<f32>,
    /// WM FW 3.x not supported anymore - optional float reserved_rx_success_ratio = 28; // Percentage
    /// WM FW 3.x not supported anymore - optional float data_rx_ratio = 47; // Percentage
    /// WM FW 3.x not supported anymore - optional float rx_duplicate_ratio = 29; // Percentage
    /// WM FW 3.x not supported anymore - optional float cca_success_ratio = 30; // Percentage
    /// WM FW 3.x not supported anymore - optional float broadcast_ratio = 31; // Percentage
    /// WM FW 3.x not supported anymore - optional float failed_unicast_ratio = 32; // Percentage
    ///
    /// Percentage
    #[prost(float, optional, tag = "33")]
    pub max_reserved_slot_usage: ::core::option::Option<f32>,
    /// Percentage
    #[prost(float, optional, tag = "48")]
    pub avg_reserved_slot_usage: ::core::option::Option<f32>,
    /// Percentage
    #[prost(float, optional, tag = "34")]
    pub max_aloha_slot_usage: ::core::option::Option<f32>,
    /// Info currently in boot diagnostics
    #[prost(uint32, optional, tag = "35")]
    pub boot_count: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "36")]
    pub sw_version_devel: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "37")]
    pub sw_version_maintenance: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "38")]
    pub sw_version_minor: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "39")]
    pub sw_version_major: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "40")]
    pub scratchpad_sequence: ::core::option::Option<u32>,
    #[prost(enumeration = "diagnostics_data::HwMagic", optional, tag = "41")]
    pub hw_magic: ::core::option::Option<i32>,
    #[prost(enumeration = "StackProfile", optional, tag = "42")]
    pub stack_profile: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "43")]
    pub otap_enabled: ::core::option::Option<bool>,
    #[prost(uint32, optional, tag = "44")]
    pub boot_line_number: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "45")]
    pub file_hash: ::core::option::Option<u32>,
    #[prost(uint32, repeated, tag = "46")]
    pub stack_trace: ::prost::alloc::vec::Vec<u32>,
    /// If WM version >= 3.4 Buffering Delay QoS1 is replaced by Routers in Neighborhood
    #[prost(uint32, optional, tag = "50")]
    pub routers_in_neighborhood: ::core::option::Option<u32>,
    /// If WM version >= 3.4 Broadcast Ratio is replaced by CB-MAC load
    #[prost(float, optional, tag = "51")]
    pub cb_mac_load: ::core::option::Option<f32>,
    /// If WM version >= 4.0 Access cycles from traffic diagnostics
    /// is replaced by cluster node amounts, but is activated again in WM FW 5.2
    #[prost(uint32, optional, tag = "52")]
    pub cluster_member_amount: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "53")]
    pub cluster_router_node_amount: ::core::option::Option<u32>,
    /// If WM version >= 4.0 cost array in node diagnostics will only contain one cost info
    /// The freed area is filled with blacklisting information
    #[prost(uint32, optional, tag = "54")]
    pub blacklisted_radio_channels_1_8: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "55")]
    pub blacklisted_radio_channels_9_16: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "56")]
    pub blacklisted_radio_channels_17_24: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "57")]
    pub blacklisted_radio_channels_25_32: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "58")]
    pub blacklisted_radio_channels_33_40: ::core::option::Option<u32>,
    /// Added in WM version 4.0
    #[prost(uint32, optional, tag = "59")]
    pub scratchpad_firmware_sequence: ::core::option::Option<u32>,
    /// New in WM 5.0
    #[prost(uint32, optional, tag = "60")]
    pub broadcast_ll_members_pending: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "61")]
    pub broadcast_le_members_pending: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "62")]
    pub broadcast_next_hop_pending: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "63")]
    pub broadcast_unack_pending: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "64")]
    pub packets_expired_pending: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "65")]
    pub packets_reroute_pending: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "66")]
    pub unicast_cluster_pending: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "67")]
    pub unicast_members_pending: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "68")]
    pub dropped_packets_combined: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "69")]
    pub cbmac_rx_messages_ack: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "70")]
    pub cbmac_rx_messages_unack: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "71")]
    pub cbmac_rx_ack_other_reasons: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "72")]
    pub cbmac_tx_ack_cca_fail: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "73")]
    pub cbmac_tx_ack_not_received: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "74")]
    pub cbmac_tx_messages_ack: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "75")]
    pub cbmac_tx_messages_unack: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "76")]
    pub cbmac_tx_cca_unack_fail: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "77")]
    pub cfmac_messages_ack: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "78")]
    pub cfmac_pending_broadcast_le_member: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "79")]
    pub next_hop_radio_power_d_b: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "80")]
    pub next_hop_rssi_d_bm: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "81")]
    pub blacklisted_radio_channels_count: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "82")]
    pub application_version_major: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "83")]
    pub application_version_minor: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "84")]
    pub application_version_maintenance: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "85")]
    pub application_version_devel: ::core::option::Option<u32>,
    #[prost(enumeration = "diagnostics_data::TraceType", optional, tag = "86")]
    pub trace_type: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "87")]
    pub packet_sequence_number: ::core::option::Option<u32>,
    /// New in 5.1
    #[prost(uint32, optional, tag = "88")]
    pub cluster_channel: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "89")]
    pub sleep_time_s: ::core::option::Option<u32>,
    #[prost(enumeration = "diagnostics_data::BootReason", optional, tag = "90")]
    pub boot_reason: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "91")]
    pub rx_gain: ::core::option::Option<i32>,
    #[prost(int32, repeated, tag = "92")]
    pub tx_powers: ::prost::alloc::vec::Vec<i32>,
    #[prost(uint32, optional, tag = "93")]
    pub scratchpad_transfers: ::core::option::Option<u32>,
    #[prost(float, optional, tag = "94")]
    pub installation_quality: ::core::option::Option<f32>,
    /// Max_count should be the same as InstallationQualityError enum item count
    #[prost(
        enumeration = "diagnostics_data::InstallationQualityError",
        repeated,
        tag = "95"
    )]
    pub installation_quality_errors: ::prost::alloc::vec::Vec<i32>,
    #[prost(uint32, optional, tag = "96")]
    pub dropped_unack_broadcast_packets: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "97")]
    pub unacknowledged_broadcast_transmissions_channel_count: ::core::option::Option<
        u32,
    >,
    #[prost(int32, optional, tag = "98")]
    pub wide_band_noise_rssi_correction_d_bm: ::core::option::Option<i32>,
    #[prost(float, optional, tag = "99")]
    pub network_channel_packet_error_rate_percent: ::core::option::Option<f32>,
    /// Old value before 5.1
    #[prost(uint64, optional, tag = "100")]
    pub boot_diagnostics_tx_time_ms_epoch: ::core::option::Option<u64>,
    /// New in 5.1 continues
    #[prost(int32, optional, tag = "101")]
    pub cca_limit_d_bm: ::core::option::Option<i32>,
    /// New in 5.2
    #[prost(uint32, optional, tag = "102")]
    pub address_clashes: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "103")]
    pub ll_device_count: ::core::option::Option<u32>,
    /// New in 5.3
    #[prost(bool, optional, tag = "104")]
    pub is_ble_scan_active: ::core::option::Option<bool>,
    #[prost(uint32, optional, tag = "105")]
    pub cost_compensation_throughput: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "106")]
    pub cost_compensation_bad_link: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "107")]
    pub cost_compensation_bad_channel: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "108")]
    pub cost_compensation_price_of_association: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "109")]
    pub cost_compensation_high_buffer_usage: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "110")]
    pub cost_compensation_high_energy_consumption: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "111")]
    pub is_joining_beacon_active: ::core::option::Option<bool>,
    #[prost(uint32, optional, tag = "112")]
    pub amount_of_flooding_messages: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "113")]
    pub amount_of_dropped_reassembled_packets: ::core::option::Option<u32>,
    /// DECT
    #[prost(uint32, optional, tag = "114")]
    pub modem_version_devel: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "115")]
    pub modem_version_maintenance: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "116")]
    pub modem_version_minor: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "117")]
    pub modem_version_major: ::core::option::Option<u32>,
}
/// Nested message and enum types in `DiagnosticsData`.
pub mod diagnostics_data {
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
    pub enum Events {
        /// This is special value to create 1-size list when no events occurred
        RoleNoEvents = 0,
        RoleChangeToSubnode = 8,
        RoleChangeToHeadnode = 9,
        RouteChange = 16,
        ScanningReqByApplication = 23,
        ScanningNoChannelSelected = 24,
        ScanningFtdmaConfWithNeighbor = 25,
        ScanningFtdmaConfWithNbNeighbor = 26,
        ScanningTimingConfWithNeighbor = 27,
        ScanningTimingConfWithMultipleNeighbors = 28,
        ScanningNeedMoreNeighbors = 29,
        ScanningPeriodic = 30,
        ScanningRoleChange = 31,
        BootPoweron = 32,
        BootIntentional = 33,
        BootSwFailure = 34,
        BootProcessorFailure = 35,
        BootWatchdogExpire = 36,
        BootUnindentifiedReason = 37,
        SynclostAlternativeRoute = 40,
        SynclostPrimaryRoute = 41,
        FtdmaAdjMinorBoundary = 48,
        FtdmaAdjNotInSlotBoundary = 49,
        FtdmaAdjConflictWithPrimaryRoute = 50,
        FtdmaAdjConflictWithAlternativeRoute = 51,
        FtdmaAdjConflictWithNeighbor = 52,
        FtdmaAdjNoChannelSelected = 53,
        FtdmaAdjChannelBlacklisted = 54,
        FtdmaAdjOtherReason = 55,
        SinkChanged = 56,
        FhmaAdjust = 57,
        TdmaAdjustCcstat = 58,
        RoutingLoop = 64,
        DenseRemoveSubnode = 72,
        DownlinkTxFailCouldNotSend = 73,
        DownlinkTxFailNextHopCouldNotSend = 74,
        UplinkTxFailCouldNotSend = 75,
        TooManyRoutersDuringScan = 76,
        TimingLateForSchedulingMacOperation = 77,
        EventUnknown = 255,
    }
    impl Events {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Events::RoleNoEvents => "ROLE_NO_EVENTS",
                Events::RoleChangeToSubnode => "ROLE_CHANGE_TO_SUBNODE",
                Events::RoleChangeToHeadnode => "ROLE_CHANGE_TO_HEADNODE",
                Events::RouteChange => "ROUTE_CHANGE",
                Events::ScanningReqByApplication => "SCANNING_REQ_BY_APPLICATION",
                Events::ScanningNoChannelSelected => "SCANNING_NO_CHANNEL_SELECTED",
                Events::ScanningFtdmaConfWithNeighbor => {
                    "SCANNING_FTDMA_CONF_WITH_NEIGHBOR"
                }
                Events::ScanningFtdmaConfWithNbNeighbor => {
                    "SCANNING_FTDMA_CONF_WITH_NB_NEIGHBOR"
                }
                Events::ScanningTimingConfWithNeighbor => {
                    "SCANNING_TIMING_CONF_WITH_NEIGHBOR"
                }
                Events::ScanningTimingConfWithMultipleNeighbors => {
                    "SCANNING_TIMING_CONF_WITH_MULTIPLE_NEIGHBORS"
                }
                Events::ScanningNeedMoreNeighbors => "SCANNING_NEED_MORE_NEIGHBORS",
                Events::ScanningPeriodic => "SCANNING_PERIODIC",
                Events::ScanningRoleChange => "SCANNING_ROLE_CHANGE",
                Events::BootPoweron => "BOOT_POWERON",
                Events::BootIntentional => "BOOT_INTENTIONAL",
                Events::BootSwFailure => "BOOT_SW_FAILURE",
                Events::BootProcessorFailure => "BOOT_PROCESSOR_FAILURE",
                Events::BootWatchdogExpire => "BOOT_WATCHDOG_EXPIRE",
                Events::BootUnindentifiedReason => "BOOT_UNINDENTIFIED_REASON",
                Events::SynclostAlternativeRoute => "SYNCLOST_ALTERNATIVE_ROUTE",
                Events::SynclostPrimaryRoute => "SYNCLOST_PRIMARY_ROUTE",
                Events::FtdmaAdjMinorBoundary => "FTDMA_ADJ_MINOR_BOUNDARY",
                Events::FtdmaAdjNotInSlotBoundary => "FTDMA_ADJ_NOT_IN_SLOT_BOUNDARY",
                Events::FtdmaAdjConflictWithPrimaryRoute => {
                    "FTDMA_ADJ_CONFLICT_WITH_PRIMARY_ROUTE"
                }
                Events::FtdmaAdjConflictWithAlternativeRoute => {
                    "FTDMA_ADJ_CONFLICT_WITH_ALTERNATIVE_ROUTE"
                }
                Events::FtdmaAdjConflictWithNeighbor => {
                    "FTDMA_ADJ_CONFLICT_WITH_NEIGHBOR"
                }
                Events::FtdmaAdjNoChannelSelected => "FTDMA_ADJ_NO_CHANNEL_SELECTED",
                Events::FtdmaAdjChannelBlacklisted => "FTDMA_ADJ_CHANNEL_BLACKLISTED",
                Events::FtdmaAdjOtherReason => "FTDMA_ADJ_OTHER_REASON",
                Events::SinkChanged => "SINK_CHANGED",
                Events::FhmaAdjust => "FHMA_ADJUST",
                Events::TdmaAdjustCcstat => "TDMA_ADJUST_CCSTAT",
                Events::RoutingLoop => "ROUTING_LOOP",
                Events::DenseRemoveSubnode => "DENSE_REMOVE_SUBNODE",
                Events::DownlinkTxFailCouldNotSend => "DOWNLINK_TX_FAIL_COULD_NOT_SEND",
                Events::DownlinkTxFailNextHopCouldNotSend => {
                    "DOWNLINK_TX_FAIL_NEXT_HOP_COULD_NOT_SEND"
                }
                Events::UplinkTxFailCouldNotSend => "UPLINK_TX_FAIL_COULD_NOT_SEND",
                Events::TooManyRoutersDuringScan => "TOO_MANY_ROUTERS_DURING_SCAN",
                Events::TimingLateForSchedulingMacOperation => {
                    "TIMING_LATE_FOR_SCHEDULING_MAC_OPERATION"
                }
                Events::EventUnknown => "EVENT_UNKNOWN",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ROLE_NO_EVENTS" => Some(Self::RoleNoEvents),
                "ROLE_CHANGE_TO_SUBNODE" => Some(Self::RoleChangeToSubnode),
                "ROLE_CHANGE_TO_HEADNODE" => Some(Self::RoleChangeToHeadnode),
                "ROUTE_CHANGE" => Some(Self::RouteChange),
                "SCANNING_REQ_BY_APPLICATION" => Some(Self::ScanningReqByApplication),
                "SCANNING_NO_CHANNEL_SELECTED" => Some(Self::ScanningNoChannelSelected),
                "SCANNING_FTDMA_CONF_WITH_NEIGHBOR" => {
                    Some(Self::ScanningFtdmaConfWithNeighbor)
                }
                "SCANNING_FTDMA_CONF_WITH_NB_NEIGHBOR" => {
                    Some(Self::ScanningFtdmaConfWithNbNeighbor)
                }
                "SCANNING_TIMING_CONF_WITH_NEIGHBOR" => {
                    Some(Self::ScanningTimingConfWithNeighbor)
                }
                "SCANNING_TIMING_CONF_WITH_MULTIPLE_NEIGHBORS" => {
                    Some(Self::ScanningTimingConfWithMultipleNeighbors)
                }
                "SCANNING_NEED_MORE_NEIGHBORS" => Some(Self::ScanningNeedMoreNeighbors),
                "SCANNING_PERIODIC" => Some(Self::ScanningPeriodic),
                "SCANNING_ROLE_CHANGE" => Some(Self::ScanningRoleChange),
                "BOOT_POWERON" => Some(Self::BootPoweron),
                "BOOT_INTENTIONAL" => Some(Self::BootIntentional),
                "BOOT_SW_FAILURE" => Some(Self::BootSwFailure),
                "BOOT_PROCESSOR_FAILURE" => Some(Self::BootProcessorFailure),
                "BOOT_WATCHDOG_EXPIRE" => Some(Self::BootWatchdogExpire),
                "BOOT_UNINDENTIFIED_REASON" => Some(Self::BootUnindentifiedReason),
                "SYNCLOST_ALTERNATIVE_ROUTE" => Some(Self::SynclostAlternativeRoute),
                "SYNCLOST_PRIMARY_ROUTE" => Some(Self::SynclostPrimaryRoute),
                "FTDMA_ADJ_MINOR_BOUNDARY" => Some(Self::FtdmaAdjMinorBoundary),
                "FTDMA_ADJ_NOT_IN_SLOT_BOUNDARY" => Some(Self::FtdmaAdjNotInSlotBoundary),
                "FTDMA_ADJ_CONFLICT_WITH_PRIMARY_ROUTE" => {
                    Some(Self::FtdmaAdjConflictWithPrimaryRoute)
                }
                "FTDMA_ADJ_CONFLICT_WITH_ALTERNATIVE_ROUTE" => {
                    Some(Self::FtdmaAdjConflictWithAlternativeRoute)
                }
                "FTDMA_ADJ_CONFLICT_WITH_NEIGHBOR" => {
                    Some(Self::FtdmaAdjConflictWithNeighbor)
                }
                "FTDMA_ADJ_NO_CHANNEL_SELECTED" => Some(Self::FtdmaAdjNoChannelSelected),
                "FTDMA_ADJ_CHANNEL_BLACKLISTED" => Some(Self::FtdmaAdjChannelBlacklisted),
                "FTDMA_ADJ_OTHER_REASON" => Some(Self::FtdmaAdjOtherReason),
                "SINK_CHANGED" => Some(Self::SinkChanged),
                "FHMA_ADJUST" => Some(Self::FhmaAdjust),
                "TDMA_ADJUST_CCSTAT" => Some(Self::TdmaAdjustCcstat),
                "ROUTING_LOOP" => Some(Self::RoutingLoop),
                "DENSE_REMOVE_SUBNODE" => Some(Self::DenseRemoveSubnode),
                "DOWNLINK_TX_FAIL_COULD_NOT_SEND" => {
                    Some(Self::DownlinkTxFailCouldNotSend)
                }
                "DOWNLINK_TX_FAIL_NEXT_HOP_COULD_NOT_SEND" => {
                    Some(Self::DownlinkTxFailNextHopCouldNotSend)
                }
                "UPLINK_TX_FAIL_COULD_NOT_SEND" => Some(Self::UplinkTxFailCouldNotSend),
                "TOO_MANY_ROUTERS_DURING_SCAN" => Some(Self::TooManyRoutersDuringScan),
                "TIMING_LATE_FOR_SCHEDULING_MAC_OPERATION" => {
                    Some(Self::TimingLateForSchedulingMacOperation)
                }
                "EVENT_UNKNOWN" => Some(Self::EventUnknown),
                _ => None,
            }
        }
    }
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
    pub enum HwMagic {
        Nrf51 = 1,
        Ezr32 = 2,
        Nrf52 = 3,
        Cc2650 = 4,
        Efr32xg121024kb128kb = 5,
        Nrf528401024kb256kb = 6,
        Efr32xg12512kb64kb = 7,
        Efr32xg13512kb = 8,
        Nrf52833 = 9,
        Efr32xg21 = 10,
        Efr32xg22 = 11,
        Bgm210pa22jia = 12,
        Bgm220pc22hna = 13,
        Bgm220sc22hna = 14,
        Nrf9160 = 15,
        Efr32xg23 = 16,
        Efr32xg24 = 17,
        HwmagicUnknown = 255,
    }
    impl HwMagic {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                HwMagic::Nrf51 => "NRF51",
                HwMagic::Ezr32 => "EZR32",
                HwMagic::Nrf52 => "NRF52",
                HwMagic::Cc2650 => "CC2650",
                HwMagic::Efr32xg121024kb128kb => "EFR32XG12_1024KB_128KB",
                HwMagic::Nrf528401024kb256kb => "NRF52840_1024KB_256KB",
                HwMagic::Efr32xg12512kb64kb => "EFR32XG12_512KB_64KB",
                HwMagic::Efr32xg13512kb => "EFR32XG13_512KB",
                HwMagic::Nrf52833 => "NRF52833",
                HwMagic::Efr32xg21 => "EFR32XG21",
                HwMagic::Efr32xg22 => "EFR32XG22",
                HwMagic::Bgm210pa22jia => "BGM210PA22JIA",
                HwMagic::Bgm220pc22hna => "BGM220PC22HNA",
                HwMagic::Bgm220sc22hna => "BGM220SC22HNA",
                HwMagic::Nrf9160 => "NRF9160",
                HwMagic::Efr32xg23 => "EFR32XG23",
                HwMagic::Efr32xg24 => "EFR32XG24",
                HwMagic::HwmagicUnknown => "HWMAGIC_UNKNOWN",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "NRF51" => Some(Self::Nrf51),
                "EZR32" => Some(Self::Ezr32),
                "NRF52" => Some(Self::Nrf52),
                "CC2650" => Some(Self::Cc2650),
                "EFR32XG12_1024KB_128KB" => Some(Self::Efr32xg121024kb128kb),
                "NRF52840_1024KB_256KB" => Some(Self::Nrf528401024kb256kb),
                "EFR32XG12_512KB_64KB" => Some(Self::Efr32xg12512kb64kb),
                "EFR32XG13_512KB" => Some(Self::Efr32xg13512kb),
                "NRF52833" => Some(Self::Nrf52833),
                "EFR32XG21" => Some(Self::Efr32xg21),
                "EFR32XG22" => Some(Self::Efr32xg22),
                "BGM210PA22JIA" => Some(Self::Bgm210pa22jia),
                "BGM220PC22HNA" => Some(Self::Bgm220pc22hna),
                "BGM220SC22HNA" => Some(Self::Bgm220sc22hna),
                "NRF9160" => Some(Self::Nrf9160),
                "EFR32XG23" => Some(Self::Efr32xg23),
                "EFR32XG24" => Some(Self::Efr32xg24),
                "HWMAGIC_UNKNOWN" => Some(Self::HwmagicUnknown),
                _ => None,
            }
        }
    }
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
    pub enum TraceType {
        NodeDiagnostics = 1,
        NeighborDiagnostics = 2,
        ScanDiagnostics = 3,
        BootDiagnostics = 4,
        BootDiagnosticsWmFw50 = 7,
    }
    impl TraceType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                TraceType::NodeDiagnostics => "NODE_DIAGNOSTICS",
                TraceType::NeighborDiagnostics => "NEIGHBOR_DIAGNOSTICS",
                TraceType::ScanDiagnostics => "SCAN_DIAGNOSTICS",
                TraceType::BootDiagnostics => "BOOT_DIAGNOSTICS",
                TraceType::BootDiagnosticsWmFw50 => "BOOT_DIAGNOSTICS_WM_FW_5_0",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "NODE_DIAGNOSTICS" => Some(Self::NodeDiagnostics),
                "NEIGHBOR_DIAGNOSTICS" => Some(Self::NeighborDiagnostics),
                "SCAN_DIAGNOSTICS" => Some(Self::ScanDiagnostics),
                "BOOT_DIAGNOSTICS" => Some(Self::BootDiagnostics),
                "BOOT_DIAGNOSTICS_WM_FW_5_0" => Some(Self::BootDiagnosticsWmFw50),
                _ => None,
            }
        }
    }
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
    pub enum BootReason {
        NormalPowerOnResetStartup = 0,
        RebootRequestedAsPartOfNormalOperation = 1,
        SoftwareAssertFailure = 2,
        McuFault = 3,
        WatchdogReset = 4,
        UnknownResetReason = 5,
    }
    impl BootReason {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                BootReason::NormalPowerOnResetStartup => "NORMAL_POWER_ON_RESET_STARTUP",
                BootReason::RebootRequestedAsPartOfNormalOperation => {
                    "REBOOT_REQUESTED_AS_PART_OF_NORMAL_OPERATION"
                }
                BootReason::SoftwareAssertFailure => "SOFTWARE_ASSERT_FAILURE",
                BootReason::McuFault => "MCU_FAULT",
                BootReason::WatchdogReset => "WATCHDOG_RESET",
                BootReason::UnknownResetReason => "UNKNOWN_RESET_REASON",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "NORMAL_POWER_ON_RESET_STARTUP" => Some(Self::NormalPowerOnResetStartup),
                "REBOOT_REQUESTED_AS_PART_OF_NORMAL_OPERATION" => {
                    Some(Self::RebootRequestedAsPartOfNormalOperation)
                }
                "SOFTWARE_ASSERT_FAILURE" => Some(Self::SoftwareAssertFailure),
                "MCU_FAULT" => Some(Self::McuFault),
                "WATCHDOG_RESET" => Some(Self::WatchdogReset),
                "UNKNOWN_RESET_REASON" => Some(Self::UnknownResetReason),
                _ => None,
            }
        }
    }
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
    pub enum InstallationQualityError {
        NoRouteToSink = 0,
        NotEnoughNeighbors = 1,
        BadRssi = 2,
    }
    impl InstallationQualityError {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                InstallationQualityError::NoRouteToSink => "NO_ROUTE_TO_SINK",
                InstallationQualityError::NotEnoughNeighbors => "NOT_ENOUGH_NEIGHBORS",
                InstallationQualityError::BadRssi => "BAD_RSSI",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "NO_ROUTE_TO_SINK" => Some(Self::NoRouteToSink),
                "NOT_ENOUGH_NEIGHBORS" => Some(Self::NotEnoughNeighbors),
                "BAD_RSSI" => Some(Self::BadRssi),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RxData {
    #[prost(uint32, optional, tag = "2")]
    pub source_endpoint: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub destination_endpoint: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "5", default = "0")]
    pub qos: ::core::option::Option<u32>,
    #[prost(bytes = "vec", optional, tag = "7")]
    pub payload: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag = "8")]
    pub payload_size: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "9")]
    pub sink_address: ::core::option::Option<u32>,
    /// If WM FW version >= 4.0 QoS is divided to QoS and Hop count
    #[prost(uint32, optional, tag = "10")]
    pub hop_count: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag = "11")]
    pub gateway_rx_time_ms_epoch: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "12")]
    pub backend_rx_time_ms_epoch: ::core::option::Option<u64>,
    #[prost(int32, optional, tag = "13")]
    pub gateway_backend_rx_time_delta_ms: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppConfigResponse {
    #[prost(enumeration = "app_config_response::AppConfigResult", optional, tag = "1")]
    pub result: ::core::option::Option<i32>,
}
/// Nested message and enum types in `AppConfigResponse`.
pub mod app_config_response {
    /// Same as gateway ErrorCode
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
    pub enum AppConfigResult {
        UnknownErrorCode = -1,
        Ok = 0,
        InternalError = 1,
        InvalidSinkId = 2,
        InvalidRole = 3,
        InvalidNetworkAddress = 4,
        InvalidNetworkChannel = 5,
        InvalidChannelMap = 6,
        InvalidNetworkKeys = 7,
        InvalidAcRange = 8,
        InvalidSinkState = 9,
        InvalidDestAddress = 10,
        InvalidDestEndpoint = 11,
        InvalidSrcEndpoint = 12,
        InvalidQos = 13,
        InvalidDataPayload = 14,
        InvalidScratchpad = 15,
        InvalidScratchpadSize = 16,
        InvalidSequenceNumber = 17,
        InvalidRebootDelay = 18,
        InvalidDiagInterval = 19,
        InvalidAppConfig = 20,
        InvalidParam = 21,
        NoScratchpadPresent = 22,
        AccessDenied = 23,
        RequestNeedsSinkId = 24,
        InvalidMaxHopCount = 25,
        SinkOutOfMemory = 26,
        SinkTimeout = 27,
        FirstInvalidEnumValue = 28,
    }
    impl AppConfigResult {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AppConfigResult::UnknownErrorCode => "UNKNOWN_ERROR_CODE",
                AppConfigResult::Ok => "OK",
                AppConfigResult::InternalError => "INTERNAL_ERROR",
                AppConfigResult::InvalidSinkId => "INVALID_SINK_ID",
                AppConfigResult::InvalidRole => "INVALID_ROLE",
                AppConfigResult::InvalidNetworkAddress => "INVALID_NETWORK_ADDRESS",
                AppConfigResult::InvalidNetworkChannel => "INVALID_NETWORK_CHANNEL",
                AppConfigResult::InvalidChannelMap => "INVALID_CHANNEL_MAP",
                AppConfigResult::InvalidNetworkKeys => "INVALID_NETWORK_KEYS",
                AppConfigResult::InvalidAcRange => "INVALID_AC_RANGE",
                AppConfigResult::InvalidSinkState => "INVALID_SINK_STATE",
                AppConfigResult::InvalidDestAddress => "INVALID_DEST_ADDRESS",
                AppConfigResult::InvalidDestEndpoint => "INVALID_DEST_ENDPOINT",
                AppConfigResult::InvalidSrcEndpoint => "INVALID_SRC_ENDPOINT",
                AppConfigResult::InvalidQos => "INVALID_QOS",
                AppConfigResult::InvalidDataPayload => "INVALID_DATA_PAYLOAD",
                AppConfigResult::InvalidScratchpad => "INVALID_SCRATCHPAD",
                AppConfigResult::InvalidScratchpadSize => "INVALID_SCRATCHPAD_SIZE",
                AppConfigResult::InvalidSequenceNumber => "INVALID_SEQUENCE_NUMBER",
                AppConfigResult::InvalidRebootDelay => "INVALID_REBOOT_DELAY",
                AppConfigResult::InvalidDiagInterval => "INVALID_DIAG_INTERVAL",
                AppConfigResult::InvalidAppConfig => "INVALID_APP_CONFIG",
                AppConfigResult::InvalidParam => "INVALID_PARAM",
                AppConfigResult::NoScratchpadPresent => "NO_SCRATCHPAD_PRESENT",
                AppConfigResult::AccessDenied => "ACCESS_DENIED",
                AppConfigResult::RequestNeedsSinkId => "REQUEST_NEEDS_SINK_ID",
                AppConfigResult::InvalidMaxHopCount => "INVALID_MAX_HOP_COUNT",
                AppConfigResult::SinkOutOfMemory => "SINK_OUT_OF_MEMORY",
                AppConfigResult::SinkTimeout => "SINK_TIMEOUT",
                AppConfigResult::FirstInvalidEnumValue => "FIRST_INVALID_ENUM_VALUE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNKNOWN_ERROR_CODE" => Some(Self::UnknownErrorCode),
                "OK" => Some(Self::Ok),
                "INTERNAL_ERROR" => Some(Self::InternalError),
                "INVALID_SINK_ID" => Some(Self::InvalidSinkId),
                "INVALID_ROLE" => Some(Self::InvalidRole),
                "INVALID_NETWORK_ADDRESS" => Some(Self::InvalidNetworkAddress),
                "INVALID_NETWORK_CHANNEL" => Some(Self::InvalidNetworkChannel),
                "INVALID_CHANNEL_MAP" => Some(Self::InvalidChannelMap),
                "INVALID_NETWORK_KEYS" => Some(Self::InvalidNetworkKeys),
                "INVALID_AC_RANGE" => Some(Self::InvalidAcRange),
                "INVALID_SINK_STATE" => Some(Self::InvalidSinkState),
                "INVALID_DEST_ADDRESS" => Some(Self::InvalidDestAddress),
                "INVALID_DEST_ENDPOINT" => Some(Self::InvalidDestEndpoint),
                "INVALID_SRC_ENDPOINT" => Some(Self::InvalidSrcEndpoint),
                "INVALID_QOS" => Some(Self::InvalidQos),
                "INVALID_DATA_PAYLOAD" => Some(Self::InvalidDataPayload),
                "INVALID_SCRATCHPAD" => Some(Self::InvalidScratchpad),
                "INVALID_SCRATCHPAD_SIZE" => Some(Self::InvalidScratchpadSize),
                "INVALID_SEQUENCE_NUMBER" => Some(Self::InvalidSequenceNumber),
                "INVALID_REBOOT_DELAY" => Some(Self::InvalidRebootDelay),
                "INVALID_DIAG_INTERVAL" => Some(Self::InvalidDiagInterval),
                "INVALID_APP_CONFIG" => Some(Self::InvalidAppConfig),
                "INVALID_PARAM" => Some(Self::InvalidParam),
                "NO_SCRATCHPAD_PRESENT" => Some(Self::NoScratchpadPresent),
                "ACCESS_DENIED" => Some(Self::AccessDenied),
                "REQUEST_NEEDS_SINK_ID" => Some(Self::RequestNeedsSinkId),
                "INVALID_MAX_HOP_COUNT" => Some(Self::InvalidMaxHopCount),
                "SINK_OUT_OF_MEMORY" => Some(Self::SinkOutOfMemory),
                "SINK_TIMEOUT" => Some(Self::SinkTimeout),
                "FIRST_INVALID_ENUM_VALUE" => Some(Self::FirstInvalidEnumValue),
                _ => None,
            }
        }
    }
}
/// This is a device measurement result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Measurement {
    /// Id of the sensor, used to distinguish different measurements in device
    #[prost(uint32, optional, tag = "1")]
    pub sensor_id: ::core::option::Option<u32>,
    /// Actual measurement value
    #[prost(float, optional, tag = "2")]
    pub value: ::core::option::Option<f32>,
    /// Unit of the measurement
    #[prost(string, optional, tag = "4")]
    pub unit: ::core::option::Option<::prost::alloc::string::String>,
    /// This is special value to create 1-size list when no measurements exist
    #[prost(bool, optional, tag = "5")]
    pub no_measurements: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GatewayHeartbeat {
    /// MAC address of the gateway
    #[prost(string, optional, tag = "1")]
    pub mac_address: ::core::option::Option<::prost::alloc::string::String>,
    /// Hostname of the gateway
    #[prost(string, optional, tag = "3")]
    pub hostname: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GatewayInfo {
    #[prost(uint32, optional, tag = "1")]
    pub current_time_s_epoch: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "2")]
    pub model: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub version: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "4")]
    pub api_version: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BackendComponentInfo {
    #[prost(uint32, optional, tag = "1")]
    pub current_time_s_epoch: ::core::option::Option<u32>,
    #[prost(enumeration = "BackendComponent", optional, tag = "2")]
    pub component: ::core::option::Option<i32>,
}
/// Mapping of channel id to frequency
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelIdToFreqMap {
    #[prost(uint32, optional, tag = "1")]
    pub id: ::core::option::Option<u32>,
    #[prost(float, optional, tag = "2")]
    pub freq_m_hz: ::core::option::Option<f32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessCycleLimits {
    #[prost(uint32, optional, tag = "1")]
    pub min_ms: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub max_ms: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeMessage {
    #[prost(uint32, optional, tag = "1")]
    pub destination_address: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub network_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub source_end_point: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub destination_end_point: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub qos: ::core::option::Option<u32>,
    #[prost(bytes = "vec", optional, tag = "6")]
    pub payload: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetScratchpadStatus {
    #[prost(uint32, optional, tag = "1")]
    pub network_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub resend_interval_s: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "3")]
    pub is_close: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "4")]
    pub is_sink_only: ::core::option::Option<bool>,
    #[prost(enumeration = "MessageSendingType", optional, tag = "5")]
    pub message_sending_type: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "6")]
    pub message_distribution_interval_s: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScratchpadStatus {
    #[prost(bool, optional, tag = "1")]
    pub is_continuous_query_on: ::core::option::Option<bool>,
    /// Not updated when is_continuous_query_on changes
    #[prost(uint64, optional, tag = "2")]
    pub tx_time_ms_epoch: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag = "10")]
    pub stored_scratchpad_size: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "11")]
    pub stored_scratchpad_crc: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "12")]
    pub stored_scratchpad_sequence: ::core::option::Option<u32>,
    #[prost(enumeration = "ScratchpadType", optional, tag = "13")]
    pub stored_scratchpad_type: ::core::option::Option<i32>,
    #[prost(enumeration = "ScratchpadWriteStatus", optional, tag = "14")]
    pub stored_scratchpad_status: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "20")]
    pub processed_fw_scratchpad_size: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "21")]
    pub processed_fw_scratchpad_crc: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "22")]
    pub processed_fw_scratchpad_sequence: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "23")]
    pub processed_fw_area_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "24")]
    pub processed_fw_version_major: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "25")]
    pub processed_fw_version_minor: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "26")]
    pub processed_fw_version_maintenance: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "27")]
    pub processed_fw_version_devel: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "40")]
    pub processed_application_scratchpad_size: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "41")]
    pub processed_application_scratchpad_crc: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "42")]
    pub processed_application_scratchpad_sequence: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "43")]
    pub processed_application_area_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "44")]
    pub processed_application_version_major: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "45")]
    pub processed_application_version_minor: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "46")]
    pub processed_application_version_maintenance: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "47")]
    pub processed_application_version_devel: ::core::option::Option<u32>,
    #[prost(enumeration = "ScratchpadAction", optional, tag = "60")]
    pub scratchpad_action: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "61")]
    pub target_sequence: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "62")]
    pub target_crc: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "63")]
    pub target_delay_minutes: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "64")]
    pub remaining_time_minutes: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetScratchpadAction {
    #[prost(uint32, optional, tag = "1")]
    pub network_id: ::core::option::Option<u32>,
    #[prost(enumeration = "ScratchpadAction", optional, tag = "2")]
    pub scratchpad_action: ::core::option::Option<i32>,
}
/// This is the main message that base of every Protocol Buffer message
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Message {
    /// Message overhead
    ///
    /// Message ID. Not kept as uint64!
    #[prost(uint32, repeated, tag = "1")]
    pub id: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub network_id: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "3")]
    pub gateway_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Generation time of the message: epoch ms
    #[prost(uint64, optional, tag = "4")]
    pub tx_time: ::core::option::Option<u64>,
    /// Reception time of the message; epoch ms
    #[prost(uint64, optional, tag = "5")]
    pub rx_time: ::core::option::Option<u64>,
    /// Node ID
    #[prost(uint32, optional, tag = "7")]
    pub source_address: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "8")]
    pub destination_address: ::core::option::Option<u32>,
    #[prost(float, optional, tag = "9")]
    pub travel_time_ms: ::core::option::Option<f32>,
    /// Messages, uni- or bidirectional
    #[prost(message, optional, tag = "50")]
    pub diagnostics: ::core::option::Option<DiagnosticsData>,
    #[prost(message, optional, tag = "51")]
    pub rx_data: ::core::option::Option<RxData>,
    #[prost(message, optional, tag = "52")]
    pub app_config: ::core::option::Option<AppConfigData>,
    #[prost(enumeration = "StackProfile", optional, tag = "53")]
    pub stack_profile: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "54")]
    pub measurement: ::prost::alloc::vec::Vec<Measurement>,
    #[prost(message, optional, tag = "55")]
    pub gw_heartbeat: ::core::option::Option<GatewayHeartbeat>,
    #[prost(message, optional, tag = "56")]
    pub tx_data: ::core::option::Option<RxData>,
    /// optional uint32 channel_map = 57;  Not supported anymore as for WM FW 3.x
    #[prost(uint32, optional, tag = "58")]
    pub network_channel: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "59")]
    pub security_enabled: ::core::option::Option<bool>,
    #[prost(message, repeated, tag = "60")]
    pub channel_info: ::prost::alloc::vec::Vec<ChannelIdToFreqMap>,
    #[prost(message, optional, tag = "61")]
    pub app_config_response: ::core::option::Option<AppConfigResponse>,
    #[prost(message, optional, tag = "62")]
    pub access_cycle_limits: ::core::option::Option<AccessCycleLimits>,
    #[prost(message, optional, tag = "63")]
    pub gateway_info: ::core::option::Option<GatewayInfo>,
    #[prost(message, optional, tag = "64")]
    pub backend_component_info: ::core::option::Option<BackendComponentInfo>,
    #[prost(message, optional, tag = "65")]
    pub scratchpad_status: ::core::option::Option<ScratchpadStatus>,
    /// Backend internal and backend -> client
    #[prost(message, optional, tag = "70")]
    pub backend_message: ::core::option::Option<BackendMessage>,
    #[prost(message, optional, tag = "71")]
    pub online_status: ::core::option::Option<OnlineStatus>,
    #[prost(message, optional, tag = "72")]
    pub traveltime_kpi: ::core::option::Option<TraveltimeKpi>,
    #[prost(message, optional, tag = "73")]
    pub node_metadata: ::core::option::Option<NodeMetadata>,
    #[prost(message, optional, tag = "74")]
    pub rtsituation_metadata: ::core::option::Option<RtSituationMetadata>,
    #[prost(message, optional, tag = "75")]
    pub backend_heartbeat: ::core::option::Option<BackendHeartbeat>,
    #[prost(message, optional, tag = "76")]
    pub metadata_update_message: ::core::option::Option<MetadataUpdateMessage>,
    #[prost(message, repeated, tag = "77")]
    pub sink_pseudo_id_map: ::prost::alloc::vec::Vec<SinkPseudoIdMap>,
    #[prost(message, optional, tag = "78")]
    pub backend_component_status: ::core::option::Option<BackendComponentStatus>,
    /// Remote API procedures
    #[prost(message, optional, tag = "80")]
    pub remoteapi_requests: ::core::option::Option<RemoteApiRequestCollection>,
    #[prost(message, optional, tag = "81")]
    pub remoteapi_response: ::core::option::Option<RemoteApiResponse>,
    /// Messages to network
    #[prost(message, repeated, tag = "90")]
    pub node_messages: ::prost::alloc::vec::Vec<NodeMessage>,
    #[prost(message, repeated, tag = "91")]
    pub get_scratchpad_status: ::prost::alloc::vec::Vec<GetScratchpadStatus>,
    /// OTAP
    #[prost(message, optional, tag = "100")]
    pub set_otap_state: ::core::option::Option<SetOtapState>,
    #[prost(message, optional, tag = "101")]
    pub set_otap_state_response: ::core::option::Option<SetOtapStateResponse>,
    #[prost(message, optional, tag = "102")]
    pub set_scratchpad_action: ::core::option::Option<SetScratchpadAction>,
    /// Positioning
    #[prost(message, optional, tag = "130")]
    pub positioning_mesh_data: ::core::option::Option<super::wpe::MeshData>,
    #[prost(message, optional, tag = "131")]
    pub positioning_status_data: ::core::option::Option<super::wpe::Status>,
    /// Other
    #[prost(message, optional, tag = "160")]
    pub calculated_values_from_diagnostics: ::core::option::Option<
        CalculatedValuesFromDiagnostics,
    >,
}
/// This is container of multiple messages at the same time
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageCollection {
    #[prost(message, repeated, tag = "1")]
    pub message_collection: ::prost::alloc::vec::Vec<Message>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum StackProfile {
    /// Stack profile definition
    Profile24 = 1,
    Profile868 = 2,
    Profile915 = 3,
    Profile870 = 4,
    Profile917 = 5,
    ProfileReserved1 = 6,
    ProfileReserved2 = 7,
    Profile865 = 8,
    ProfileEfr3224 = 9,
    ProfileReserved3 = 10,
    ProfileEfr32915Australia = 11,
    ProfileDectNr = 12,
    ProfileNrf52244dbm = 13,
    ProfileNrf52248dbm = 14,
    ProfileBleEfr32Dmp = 15,
    ProfileUnknown = 255,
}
impl StackProfile {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            StackProfile::Profile24 => "PROFILE_24",
            StackProfile::Profile868 => "PROFILE_868",
            StackProfile::Profile915 => "PROFILE_915",
            StackProfile::Profile870 => "PROFILE_870",
            StackProfile::Profile917 => "PROFILE_917",
            StackProfile::ProfileReserved1 => "PROFILE_RESERVED_1",
            StackProfile::ProfileReserved2 => "PROFILE_RESERVED_2",
            StackProfile::Profile865 => "PROFILE_865",
            StackProfile::ProfileEfr3224 => "PROFILE_EFR32_24",
            StackProfile::ProfileReserved3 => "PROFILE_RESERVED_3",
            StackProfile::ProfileEfr32915Australia => "PROFILE_EFR32_915_AUSTRALIA",
            StackProfile::ProfileDectNr => "PROFILE_DECT_NR",
            StackProfile::ProfileNrf52244dbm => "PROFILE_NRF52_24_4DBM",
            StackProfile::ProfileNrf52248dbm => "PROFILE_NRF52_24_8DBM",
            StackProfile::ProfileBleEfr32Dmp => "PROFILE_BLE_EFR32_DMP",
            StackProfile::ProfileUnknown => "PROFILE_UNKNOWN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PROFILE_24" => Some(Self::Profile24),
            "PROFILE_868" => Some(Self::Profile868),
            "PROFILE_915" => Some(Self::Profile915),
            "PROFILE_870" => Some(Self::Profile870),
            "PROFILE_917" => Some(Self::Profile917),
            "PROFILE_RESERVED_1" => Some(Self::ProfileReserved1),
            "PROFILE_RESERVED_2" => Some(Self::ProfileReserved2),
            "PROFILE_865" => Some(Self::Profile865),
            "PROFILE_EFR32_24" => Some(Self::ProfileEfr3224),
            "PROFILE_RESERVED_3" => Some(Self::ProfileReserved3),
            "PROFILE_EFR32_915_AUSTRALIA" => Some(Self::ProfileEfr32915Australia),
            "PROFILE_DECT_NR" => Some(Self::ProfileDectNr),
            "PROFILE_NRF52_24_4DBM" => Some(Self::ProfileNrf52244dbm),
            "PROFILE_NRF52_24_8DBM" => Some(Self::ProfileNrf52248dbm),
            "PROFILE_BLE_EFR32_DMP" => Some(Self::ProfileBleEfr32Dmp),
            "PROFILE_UNKNOWN" => Some(Self::ProfileUnknown),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ScratchpadType {
    Blank = 0,
    Present = 1,
    Process = 2,
    Unknown = 3,
}
impl ScratchpadType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ScratchpadType::Blank => "SCRATCHPAD_TYPE_BLANK",
            ScratchpadType::Present => "SCRATCHPAD_TYPE_PRESENT",
            ScratchpadType::Process => "SCRATCHPAD_TYPE_PROCESS",
            ScratchpadType::Unknown => "SCRATCHPAD_TYPE_UNKNOWN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SCRATCHPAD_TYPE_BLANK" => Some(Self::Blank),
            "SCRATCHPAD_TYPE_PRESENT" => Some(Self::Present),
            "SCRATCHPAD_TYPE_PROCESS" => Some(Self::Process),
            "SCRATCHPAD_TYPE_UNKNOWN" => Some(Self::Unknown),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ScratchpadWriteStatus {
    Ok = 0,
    CompletedOk = 1,
    CompletedError = 2,
    NotOngoing = 3,
    InvalidStart = 4,
    InvalidNumBytes = 5,
    InvalidHeader = 6,
    InvalidNullBytes = 7,
    FlashError = 8,
    Unknown = 9,
}
impl ScratchpadWriteStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ScratchpadWriteStatus::Ok => "SCRATCHPAD_WRITE_STATUS_OK",
            ScratchpadWriteStatus::CompletedOk => "SCRATCHPAD_WRITE_STATUS_COMPLETED_OK",
            ScratchpadWriteStatus::CompletedError => {
                "SCRATCHPAD_WRITE_STATUS_COMPLETED_ERROR"
            }
            ScratchpadWriteStatus::NotOngoing => "SCRATCHPAD_WRITE_STATUS_NOT_ONGOING",
            ScratchpadWriteStatus::InvalidStart => {
                "SCRATCHPAD_WRITE_STATUS_INVALID_START"
            }
            ScratchpadWriteStatus::InvalidNumBytes => {
                "SCRATCHPAD_WRITE_STATUS_INVALID_NUM_BYTES"
            }
            ScratchpadWriteStatus::InvalidHeader => {
                "SCRATCHPAD_WRITE_STATUS_INVALID_HEADER"
            }
            ScratchpadWriteStatus::InvalidNullBytes => {
                "SCRATCHPAD_WRITE_STATUS_INVALID_NULL_BYTES"
            }
            ScratchpadWriteStatus::FlashError => "SCRATCHPAD_WRITE_STATUS_FLASH_ERROR",
            ScratchpadWriteStatus::Unknown => "SCRATCHPAD_WRITE_STATUS_UNKNOWN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SCRATCHPAD_WRITE_STATUS_OK" => Some(Self::Ok),
            "SCRATCHPAD_WRITE_STATUS_COMPLETED_OK" => Some(Self::CompletedOk),
            "SCRATCHPAD_WRITE_STATUS_COMPLETED_ERROR" => Some(Self::CompletedError),
            "SCRATCHPAD_WRITE_STATUS_NOT_ONGOING" => Some(Self::NotOngoing),
            "SCRATCHPAD_WRITE_STATUS_INVALID_START" => Some(Self::InvalidStart),
            "SCRATCHPAD_WRITE_STATUS_INVALID_NUM_BYTES" => Some(Self::InvalidNumBytes),
            "SCRATCHPAD_WRITE_STATUS_INVALID_HEADER" => Some(Self::InvalidHeader),
            "SCRATCHPAD_WRITE_STATUS_INVALID_NULL_BYTES" => Some(Self::InvalidNullBytes),
            "SCRATCHPAD_WRITE_STATUS_FLASH_ERROR" => Some(Self::FlashError),
            "SCRATCHPAD_WRITE_STATUS_UNKNOWN" => Some(Self::Unknown),
            _ => None,
        }
    }
}
