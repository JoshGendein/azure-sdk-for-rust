#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessKeys {
    #[serde(rename = "primaryConnectionString", default, skip_serializing_if = "Option::is_none")]
    pub primary_connection_string: Option<String>,
    #[serde(rename = "secondaryConnectionString", default, skip_serializing_if = "Option::is_none")]
    pub secondary_connection_string: Option<String>,
    #[serde(rename = "primaryKey", default, skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<String>,
    #[serde(rename = "secondaryKey", default, skip_serializing_if = "Option::is_none")]
    pub secondary_key: Option<String>,
    #[serde(rename = "keyName", default, skip_serializing_if = "Option::is_none")]
    pub key_name: Option<String>,
}
impl AccessKeys {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthorizationRule {
    #[serde(flatten)]
    pub resource: Resource,
    pub properties: authorization_rule::Properties,
}
impl AuthorizationRule {
    pub fn new(properties: authorization_rule::Properties) -> Self {
        Self {
            resource: Resource::default(),
            properties,
        }
    }
}
pub mod authorization_rule {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        pub rights: Vec<String>,
    }
    impl Properties {
        pub fn new(rights: Vec<String>) -> Self {
            Self { rights }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AuthorizationRuleListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AuthorizationRule>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl AuthorizationRuleListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckNameAvailability {
    pub name: String,
}
impl CheckNameAvailability {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameAvailabilityResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<UnavailableReason>,
}
impl CheckNameAvailabilityResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HybridConnection {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<hybrid_connection::Properties>,
}
impl HybridConnection {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod hybrid_connection {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
        pub created_at: Option<String>,
        #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
        pub updated_at: Option<String>,
        #[serde(rename = "listenerCount", default, skip_serializing_if = "Option::is_none")]
        pub listener_count: Option<i32>,
        #[serde(rename = "requiresClientAuthorization", default, skip_serializing_if = "Option::is_none")]
        pub requires_client_authorization: Option<bool>,
        #[serde(rename = "userMetadata", default, skip_serializing_if = "Option::is_none")]
        pub user_metadata: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HybridConnectionListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<HybridConnection>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl HybridConnectionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegenerateAccessKeyParameters {
    #[serde(rename = "keyType")]
    pub key_type: regenerate_access_key_parameters::KeyType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}
impl RegenerateAccessKeyParameters {
    pub fn new(key_type: regenerate_access_key_parameters::KeyType) -> Self {
        Self { key_type, key: None }
    }
}
pub mod regenerate_access_key_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum KeyType {
        PrimaryKey,
        SecondaryKey,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RelayNamespace {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RelayNamespaceProperties>,
}
impl RelayNamespace {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            sku: None,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RelayNamespaceListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RelayNamespace>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl RelayNamespaceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RelayNamespaceProperties {
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<relay_namespace_properties::ProvisioningState>,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "serviceBusEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub service_bus_endpoint: Option<String>,
    #[serde(rename = "metricId", default, skip_serializing_if = "Option::is_none")]
    pub metric_id: Option<String>,
}
impl RelayNamespaceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod relay_namespace_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Created,
        Succeeded,
        Deleted,
        Failed,
        Updating,
        Unknown,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RelayUpdateParameters {
    #[serde(flatten)]
    pub resource_namespace_patch: ResourceNamespacePatch,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RelayNamespaceProperties>,
}
impl RelayUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceNamespacePatch {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ResourceNamespacePatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    pub name: sku::Name,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<sku::Tier>,
}
impl Sku {
    pub fn new(name: sku::Name) -> Self {
        Self { name, tier: None }
    }
}
pub mod sku {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        Standard,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Tier {
        Standard,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl TrackedResource {
    pub fn new(location: String) -> Self {
        Self {
            resource: Resource::default(),
            location,
            tags: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum UnavailableReason {
    None,
    InvalidName,
    SubscriptionIsDisabled,
    NameInUse,
    NameInLockdown,
    TooManyNamespaceInCurrentSubscription,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WcfRelay {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<wcf_relay::Properties>,
}
impl WcfRelay {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod wcf_relay {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[serde(rename = "isDynamic", default, skip_serializing_if = "Option::is_none")]
        pub is_dynamic: Option<bool>,
        #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
        pub created_at: Option<String>,
        #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
        pub updated_at: Option<String>,
        #[serde(rename = "listenerCount", default, skip_serializing_if = "Option::is_none")]
        pub listener_count: Option<i32>,
        #[serde(rename = "relayType", default, skip_serializing_if = "Option::is_none")]
        pub relay_type: Option<properties::RelayType>,
        #[serde(rename = "requiresClientAuthorization", default, skip_serializing_if = "Option::is_none")]
        pub requires_client_authorization: Option<bool>,
        #[serde(rename = "requiresTransportSecurity", default, skip_serializing_if = "Option::is_none")]
        pub requires_transport_security: Option<bool>,
        #[serde(rename = "userMetadata", default, skip_serializing_if = "Option::is_none")]
        pub user_metadata: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum RelayType {
            NetTcp,
            Http,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WcfRelaysListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<WcfRelay>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl WcfRelaysListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
