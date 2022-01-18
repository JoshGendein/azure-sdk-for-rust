#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessPolicy {
    #[serde(rename = "Start", default, skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    #[serde(rename = "Expiry", default, skip_serializing_if = "Option::is_none")]
    pub expiry: Option<String>,
    #[serde(rename = "Permission", default, skip_serializing_if = "Option::is_none")]
    pub permission: Option<String>,
}
impl AccessPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CorsRule {
    #[serde(rename = "AllowedOrigins")]
    pub allowed_origins: String,
    #[serde(rename = "AllowedMethods")]
    pub allowed_methods: String,
    #[serde(rename = "AllowedHeaders")]
    pub allowed_headers: String,
    #[serde(rename = "ExposedHeaders")]
    pub exposed_headers: String,
    #[serde(rename = "MaxAgeInSeconds")]
    pub max_age_in_seconds: i64,
}
impl CorsRule {
    pub fn new(
        allowed_origins: String,
        allowed_methods: String,
        allowed_headers: String,
        exposed_headers: String,
        max_age_in_seconds: i64,
    ) -> Self {
        Self {
            allowed_origins,
            allowed_methods,
            allowed_headers,
            exposed_headers,
            max_age_in_seconds,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DequeuedMessageItem {
    #[serde(rename = "MessageId")]
    pub message_id: String,
    #[serde(rename = "InsertionTime")]
    pub insertion_time: String,
    #[serde(rename = "ExpirationTime")]
    pub expiration_time: String,
    #[serde(rename = "PopReceipt")]
    pub pop_receipt: String,
    #[serde(rename = "TimeNextVisible")]
    pub time_next_visible: String,
    #[serde(rename = "DequeueCount")]
    pub dequeue_count: i64,
    #[serde(rename = "MessageText")]
    pub message_text: String,
}
impl DequeuedMessageItem {
    pub fn new(
        message_id: String,
        insertion_time: String,
        expiration_time: String,
        pop_receipt: String,
        time_next_visible: String,
        dequeue_count: i64,
        message_text: String,
    ) -> Self {
        Self {
            message_id,
            insertion_time,
            expiration_time,
            pop_receipt,
            time_next_visible,
            dequeue_count,
            message_text,
        }
    }
}
pub type DequeuedMessagesList = Vec<DequeuedMessageItem>;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnqueuedMessage {
    #[serde(rename = "MessageId")]
    pub message_id: String,
    #[serde(rename = "InsertionTime")]
    pub insertion_time: String,
    #[serde(rename = "ExpirationTime")]
    pub expiration_time: String,
    #[serde(rename = "PopReceipt")]
    pub pop_receipt: String,
    #[serde(rename = "TimeNextVisible")]
    pub time_next_visible: String,
}
impl EnqueuedMessage {
    pub fn new(
        message_id: String,
        insertion_time: String,
        expiration_time: String,
        pop_receipt: String,
        time_next_visible: String,
    ) -> Self {
        Self {
            message_id,
            insertion_time,
            expiration_time,
            pop_receipt,
            time_next_visible,
        }
    }
}
pub type EnqueuedMessageList = Vec<EnqueuedMessage>;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ErrorCode {
    AccountAlreadyExists,
    AccountBeingCreated,
    AccountIsDisabled,
    AuthenticationFailed,
    AuthorizationFailure,
    ConditionHeadersNotSupported,
    ConditionNotMet,
    EmptyMetadataKey,
    InsufficientAccountPermissions,
    InternalError,
    InvalidAuthenticationInfo,
    InvalidHeaderValue,
    InvalidHttpVerb,
    InvalidInput,
    InvalidMd5,
    InvalidMetadata,
    InvalidQueryParameterValue,
    InvalidRange,
    InvalidResourceName,
    InvalidUri,
    InvalidXmlDocument,
    InvalidXmlNodeValue,
    Md5Mismatch,
    MetadataTooLarge,
    MissingContentLengthHeader,
    MissingRequiredQueryParameter,
    MissingRequiredHeader,
    MissingRequiredXmlNode,
    MultipleConditionHeadersNotSupported,
    OperationTimedOut,
    OutOfRangeInput,
    OutOfRangeQueryParameterValue,
    RequestBodyTooLarge,
    ResourceTypeMismatch,
    RequestUrlFailedToParse,
    ResourceAlreadyExists,
    ResourceNotFound,
    ServerBusy,
    UnsupportedHeader,
    UnsupportedXmlNode,
    UnsupportedQueryParameter,
    UnsupportedHttpVerb,
    InvalidMarker,
    MessageNotFound,
    MessageTooLarge,
    PopReceiptMismatch,
    QueueAlreadyExists,
    QueueBeingDeleted,
    QueueDisabled,
    QueueNotEmpty,
    QueueNotFound,
    #[serde(rename = "AuthorizationSourceIPMismatch")]
    AuthorizationSourceIpMismatch,
    AuthorizationProtocolMismatch,
    AuthorizationPermissionMismatch,
    AuthorizationServiceMismatch,
    AuthorizationResourceTypeMismatch,
    FeatureVersionMismatch,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeoReplication {
    #[serde(rename = "Status")]
    pub status: geo_replication::Status,
    #[serde(rename = "LastSyncTime")]
    pub last_sync_time: String,
}
impl GeoReplication {
    pub fn new(status: geo_replication::Status, last_sync_time: String) -> Self {
        Self { status, last_sync_time }
    }
}
pub mod geo_replication {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "live")]
        Live,
        #[serde(rename = "bootstrap")]
        Bootstrap,
        #[serde(rename = "unavailable")]
        Unavailable,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListQueuesSegmentResponse {
    #[serde(rename = "ServiceEndpoint")]
    pub service_endpoint: String,
    #[serde(rename = "Prefix")]
    pub prefix: String,
    #[serde(rename = "Marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxResults")]
    pub max_results: i64,
    #[serde(rename = "QueueItems", default, skip_serializing_if = "Vec::is_empty")]
    pub queue_items: Vec<QueueItem>,
    #[serde(rename = "NextMarker")]
    pub next_marker: String,
}
impl ListQueuesSegmentResponse {
    pub fn new(service_endpoint: String, prefix: String, max_results: i64, next_marker: String) -> Self {
        Self {
            service_endpoint,
            prefix,
            marker: None,
            max_results,
            queue_items: Vec::new(),
            next_marker,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Logging {
    #[serde(rename = "Version")]
    pub version: String,
    #[serde(rename = "Delete")]
    pub delete: bool,
    #[serde(rename = "Read")]
    pub read: bool,
    #[serde(rename = "Write")]
    pub write: bool,
    #[serde(rename = "RetentionPolicy")]
    pub retention_policy: RetentionPolicy,
}
impl Logging {
    pub fn new(version: String, delete: bool, read: bool, write: bool, retention_policy: RetentionPolicy) -> Self {
        Self {
            version,
            delete,
            read,
            write,
            retention_policy,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Metadata {}
impl Metadata {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Metrics {
    #[serde(rename = "Version", default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "Enabled")]
    pub enabled: bool,
    #[serde(rename = "IncludeAPIs", default, skip_serializing_if = "Option::is_none")]
    pub include_ap_is: Option<bool>,
    #[serde(rename = "RetentionPolicy", default, skip_serializing_if = "Option::is_none")]
    pub retention_policy: Option<RetentionPolicy>,
}
impl Metrics {
    pub fn new(enabled: bool) -> Self {
        Self {
            version: None,
            enabled,
            include_ap_is: None,
            retention_policy: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PeekedMessageItem {
    #[serde(rename = "MessageId")]
    pub message_id: String,
    #[serde(rename = "InsertionTime")]
    pub insertion_time: String,
    #[serde(rename = "ExpirationTime")]
    pub expiration_time: String,
    #[serde(rename = "DequeueCount")]
    pub dequeue_count: i64,
    #[serde(rename = "MessageText")]
    pub message_text: String,
}
impl PeekedMessageItem {
    pub fn new(message_id: String, insertion_time: String, expiration_time: String, dequeue_count: i64, message_text: String) -> Self {
        Self {
            message_id,
            insertion_time,
            expiration_time,
            dequeue_count,
            message_text,
        }
    }
}
pub type PeekedMessagesList = Vec<PeekedMessageItem>;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueueItem {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Metadata", default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}
impl QueueItem {
    pub fn new(name: String) -> Self {
        Self { name, metadata: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueueMessage {
    #[serde(rename = "MessageText")]
    pub message_text: String,
}
impl QueueMessage {
    pub fn new(message_text: String) -> Self {
        Self { message_text }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RetentionPolicy {
    #[serde(rename = "Enabled")]
    pub enabled: bool,
    #[serde(rename = "Days", default, skip_serializing_if = "Option::is_none")]
    pub days: Option<i64>,
}
impl RetentionPolicy {
    pub fn new(enabled: bool) -> Self {
        Self { enabled, days: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SignedIdentifier {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "AccessPolicy")]
    pub access_policy: AccessPolicy,
}
impl SignedIdentifier {
    pub fn new(id: String, access_policy: AccessPolicy) -> Self {
        Self { id, access_policy }
    }
}
pub type SignedIdentifiers = Vec<SignedIdentifier>;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageError {
    #[serde(rename = "Message", default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl StorageError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageServiceProperties {
    #[serde(rename = "Logging", default, skip_serializing_if = "Option::is_none")]
    pub logging: Option<Logging>,
    #[serde(rename = "HourMetrics", default, skip_serializing_if = "Option::is_none")]
    pub hour_metrics: Option<Metrics>,
    #[serde(rename = "MinuteMetrics", default, skip_serializing_if = "Option::is_none")]
    pub minute_metrics: Option<Metrics>,
    #[serde(rename = "Cors", default, skip_serializing_if = "Vec::is_empty")]
    pub cors: Vec<CorsRule>,
}
impl StorageServiceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageServiceStats {
    #[serde(rename = "GeoReplication", default, skip_serializing_if = "Option::is_none")]
    pub geo_replication: Option<GeoReplication>,
}
impl StorageServiceStats {
    pub fn new() -> Self {
        Self::default()
    }
}
