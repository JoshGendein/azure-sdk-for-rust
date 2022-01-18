#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Action {
    #[serde(rename = "odata.type")]
    pub odata_type: String,
}
impl Action {
    pub fn new(odata_type: String) -> Self {
        Self { odata_type }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AlertSeverity {
    #[serde(rename = "0")]
    N0,
    #[serde(rename = "1")]
    N1,
    #[serde(rename = "2")]
    N2,
    #[serde(rename = "3")]
    N3,
    #[serde(rename = "4")]
    N4,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertingAction {
    #[serde(flatten)]
    pub action: Action,
    pub severity: AlertSeverity,
    #[serde(rename = "aznsAction", default, skip_serializing_if = "Option::is_none")]
    pub azns_action: Option<AzNsActionGroup>,
    #[serde(rename = "throttlingInMin", default, skip_serializing_if = "Option::is_none")]
    pub throttling_in_min: Option<i32>,
    pub trigger: TriggerCondition,
}
impl AlertingAction {
    pub fn new(action: Action, severity: AlertSeverity, trigger: TriggerCondition) -> Self {
        Self {
            action,
            severity,
            azns_action: None,
            throttling_in_min: None,
            trigger,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzNsActionGroup {
    #[serde(rename = "actionGroup", default, skip_serializing_if = "Vec::is_empty")]
    pub action_group: Vec<String>,
    #[serde(rename = "emailSubject", default, skip_serializing_if = "Option::is_none")]
    pub email_subject: Option<String>,
    #[serde(rename = "customWebhookPayload", default, skip_serializing_if = "Option::is_none")]
    pub custom_webhook_payload: Option<String>,
}
impl AzNsActionGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ConditionalOperator {
    GreaterThanOrEqual,
    LessThanOrEqual,
    GreaterThan,
    LessThan,
    Equal,
}
impl Default for ConditionalOperator {
    fn default() -> Self {
        Self::GreaterThanOrEqual
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Criteria {
    #[serde(rename = "metricName")]
    pub metric_name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dimensions: Vec<Dimension>,
}
impl Criteria {
    pub fn new(metric_name: String) -> Self {
        Self {
            metric_name,
            dimensions: Vec::new(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Dimension {
    pub name: String,
    pub operator: dimension::Operator,
    pub values: Vec<String>,
}
impl Dimension {
    pub fn new(name: String, operator: dimension::Operator, values: Vec<String>) -> Self {
        Self { name, operator, values }
    }
}
pub mod dimension {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Operator {
        Include,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorContract {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
}
impl ErrorContract {
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
pub struct LogMetricTrigger {
    #[serde(rename = "thresholdOperator", default, skip_serializing_if = "Option::is_none")]
    pub threshold_operator: Option<ConditionalOperator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub threshold: Option<f64>,
    #[serde(rename = "metricTriggerType", default, skip_serializing_if = "Option::is_none")]
    pub metric_trigger_type: Option<MetricTriggerType>,
    #[serde(rename = "metricColumn", default, skip_serializing_if = "Option::is_none")]
    pub metric_column: Option<String>,
}
impl LogMetricTrigger {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogSearchRule {
    #[serde(rename = "createdWithApiVersion", default, skip_serializing_if = "Option::is_none")]
    pub created_with_api_version: Option<String>,
    #[serde(rename = "isLegacyLogAnalyticsRule", default, skip_serializing_if = "Option::is_none")]
    pub is_legacy_log_analytics_rule: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "autoMitigate", default, skip_serializing_if = "Option::is_none")]
    pub auto_mitigate: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<log_search_rule::Enabled>,
    #[serde(rename = "lastUpdatedTime", default, skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<String>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<log_search_rule::ProvisioningState>,
    pub source: Source,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Schedule>,
    pub action: Action,
}
impl LogSearchRule {
    pub fn new(source: Source, action: Action) -> Self {
        Self {
            created_with_api_version: None,
            is_legacy_log_analytics_rule: None,
            description: None,
            display_name: None,
            auto_mitigate: None,
            enabled: None,
            last_updated_time: None,
            provisioning_state: None,
            source,
            schedule: None,
            action,
        }
    }
}
pub mod log_search_rule {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Enabled {
        #[serde(rename = "true")]
        True,
        #[serde(rename = "false")]
        False,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Succeeded,
        Deploying,
        Canceled,
        Failed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogSearchRulePatch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<log_search_rule_patch::Enabled>,
}
impl LogSearchRulePatch {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod log_search_rule_patch {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Enabled {
        #[serde(rename = "true")]
        True,
        #[serde(rename = "false")]
        False,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogSearchRuleResource {
    #[serde(flatten)]
    pub resource: Resource,
    pub properties: LogSearchRule,
}
impl LogSearchRuleResource {
    pub fn new(resource: Resource, properties: LogSearchRule) -> Self {
        Self { resource, properties }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogSearchRuleResourceCollection {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<LogSearchRuleResource>,
}
impl LogSearchRuleResourceCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogSearchRuleResourcePatch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<LogSearchRulePatch>,
}
impl LogSearchRuleResourcePatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogToMetricAction {
    #[serde(flatten)]
    pub action: Action,
    pub criteria: Vec<Criteria>,
}
impl LogToMetricAction {
    pub fn new(action: Action, criteria: Vec<Criteria>) -> Self {
        Self { action, criteria }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum MetricTriggerType {
    Consecutive,
    Total,
}
impl Default for MetricTriggerType {
    fn default() -> Self {
        Self::Consecutive
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum QueryType {
    ResultCount,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl Resource {
    pub fn new(location: String) -> Self {
        Self {
            id: None,
            name: None,
            type_: None,
            location,
            tags: None,
            kind: None,
            etag: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Schedule {
    #[serde(rename = "frequencyInMinutes")]
    pub frequency_in_minutes: i32,
    #[serde(rename = "timeWindowInMinutes")]
    pub time_window_in_minutes: i32,
}
impl Schedule {
    pub fn new(frequency_in_minutes: i32, time_window_in_minutes: i32) -> Self {
        Self {
            frequency_in_minutes,
            time_window_in_minutes,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Source {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(rename = "authorizedResources", default, skip_serializing_if = "Vec::is_empty")]
    pub authorized_resources: Vec<String>,
    #[serde(rename = "dataSourceId")]
    pub data_source_id: String,
    #[serde(rename = "queryType", default, skip_serializing_if = "Option::is_none")]
    pub query_type: Option<QueryType>,
}
impl Source {
    pub fn new(data_source_id: String) -> Self {
        Self {
            query: None,
            authorized_resources: Vec::new(),
            data_source_id,
            query_type: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TriggerCondition {
    #[serde(rename = "thresholdOperator")]
    pub threshold_operator: ConditionalOperator,
    pub threshold: f64,
    #[serde(rename = "metricTrigger", default, skip_serializing_if = "Option::is_none")]
    pub metric_trigger: Option<LogMetricTrigger>,
}
impl TriggerCondition {
    pub fn new(threshold_operator: ConditionalOperator, threshold: f64) -> Self {
        Self {
            threshold_operator,
            threshold,
            metric_trigger: None,
        }
    }
}
