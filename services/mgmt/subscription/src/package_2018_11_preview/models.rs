#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdPrincipal {
    #[serde(rename = "objectId")]
    pub object_id: String,
}
impl AdPrincipal {
    pub fn new(object_id: String) -> Self {
        Self { object_id }
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
pub struct Location {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latitude: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub longitude: Option<String>,
}
impl Location {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LocationListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Location>,
}
impl LocationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ModernCspSubscriptionCreationParameters {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "skuId")]
    pub sku_id: String,
    #[serde(rename = "resellerId", default, skip_serializing_if = "Option::is_none")]
    pub reseller_id: Option<String>,
    #[serde(rename = "serviceProviderId", default, skip_serializing_if = "Option::is_none")]
    pub service_provider_id: Option<String>,
}
impl ModernCspSubscriptionCreationParameters {
    pub fn new(display_name: String, sku_id: String) -> Self {
        Self {
            display_name,
            sku_id,
            reseller_id: None,
            service_provider_id: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ModernSubscriptionCreationParameters {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "billingProfileId")]
    pub billing_profile_id: String,
    #[serde(rename = "skuId")]
    pub sku_id: String,
    #[serde(rename = "costCenter", default, skip_serializing_if = "Option::is_none")]
    pub cost_center: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<AdPrincipal>,
    #[serde(rename = "managementGroupId", default, skip_serializing_if = "Option::is_none")]
    pub management_group_id: Option<String>,
    #[serde(rename = "additionalParameters", default, skip_serializing_if = "Option::is_none")]
    pub additional_parameters: Option<serde_json::Value>,
}
impl ModernSubscriptionCreationParameters {
    pub fn new(display_name: String, billing_profile_id: String, sku_id: String) -> Self {
        Self {
            display_name,
            billing_profile_id,
            sku_id,
            cost_center: None,
            owner: None,
            management_group_id: None,
            additional_parameters: None,
        }
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Subscription {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<subscription::State>,
    #[serde(rename = "subscriptionPolicies", default, skip_serializing_if = "Option::is_none")]
    pub subscription_policies: Option<SubscriptionPolicies>,
    #[serde(rename = "authorizationSource", default, skip_serializing_if = "Option::is_none")]
    pub authorization_source: Option<String>,
}
impl Subscription {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod subscription {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Enabled,
        Warned,
        PastDue,
        Disabled,
        Deleted,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionCreationResult {
    #[serde(rename = "subscriptionLink", default, skip_serializing_if = "Option::is_none")]
    pub subscription_link: Option<String>,
}
impl SubscriptionCreationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubscriptionListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Subscription>,
    #[serde(rename = "nextLink")]
    pub next_link: String,
}
impl SubscriptionListResult {
    pub fn new(next_link: String) -> Self {
        Self {
            value: Vec::new(),
            next_link,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionOperation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusDetail", default, skip_serializing_if = "Option::is_none")]
    pub status_detail: Option<String>,
}
impl SubscriptionOperation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionOperationListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SubscriptionOperation>,
}
impl SubscriptionOperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionPolicies {
    #[serde(rename = "locationPlacementId", default, skip_serializing_if = "Option::is_none")]
    pub location_placement_id: Option<String>,
    #[serde(rename = "quotaId", default, skip_serializing_if = "Option::is_none")]
    pub quota_id: Option<String>,
    #[serde(rename = "spendingLimit", default, skip_serializing_if = "Option::is_none")]
    pub spending_limit: Option<subscription_policies::SpendingLimit>,
}
impl SubscriptionPolicies {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod subscription_policies {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SpendingLimit {
        On,
        Off,
        CurrentPeriodOff,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TenantIdDescription {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
impl TenantIdDescription {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TenantListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<TenantIdDescription>,
    #[serde(rename = "nextLink")]
    pub next_link: String,
}
impl TenantListResult {
    pub fn new(next_link: String) -> Self {
        Self {
            value: Vec::new(),
            next_link,
        }
    }
}
