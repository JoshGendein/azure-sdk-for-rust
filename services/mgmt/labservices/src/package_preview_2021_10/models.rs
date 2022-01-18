#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutoShutdownProfile {
    #[serde(rename = "shutdownOnDisconnect", default, skip_serializing_if = "Option::is_none")]
    pub shutdown_on_disconnect: Option<EnableState>,
    #[serde(rename = "shutdownWhenNotConnected", default, skip_serializing_if = "Option::is_none")]
    pub shutdown_when_not_connected: Option<EnableState>,
    #[serde(rename = "shutdownOnIdle", default, skip_serializing_if = "Option::is_none")]
    pub shutdown_on_idle: Option<ShutdownOnIdleMode>,
    #[serde(rename = "disconnectDelay", default, skip_serializing_if = "Option::is_none")]
    pub disconnect_delay: Option<String>,
    #[serde(rename = "noConnectDelay", default, skip_serializing_if = "Option::is_none")]
    pub no_connect_delay: Option<String>,
    #[serde(rename = "idleDelay", default, skip_serializing_if = "Option::is_none")]
    pub idle_delay: Option<String>,
}
impl AutoShutdownProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectionProfile {
    #[serde(rename = "webSshAccess", default, skip_serializing_if = "Option::is_none")]
    pub web_ssh_access: Option<ConnectionType>,
    #[serde(rename = "webRdpAccess", default, skip_serializing_if = "Option::is_none")]
    pub web_rdp_access: Option<ConnectionType>,
    #[serde(rename = "clientSshAccess", default, skip_serializing_if = "Option::is_none")]
    pub client_ssh_access: Option<ConnectionType>,
    #[serde(rename = "clientRdpAccess", default, skip_serializing_if = "Option::is_none")]
    pub client_rdp_access: Option<ConnectionType>,
}
impl ConnectionProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Credentials {
    pub username: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}
impl Credentials {
    pub fn new(username: String) -> Self {
        Self { username, password: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorAdditionalInfo {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<serde_json::Value>,
}
impl ErrorAdditionalInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDetail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorDetail>,
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_info: Vec<ErrorAdditionalInfo>,
}
impl ErrorDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
}
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Image {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    pub properties: ImageProperties,
}
impl Image {
    pub fn new(properties: ImageProperties) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            system_data: None,
            properties,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageProperties {
    #[serde(flatten)]
    pub image_update_properties: ImageUpdateProperties,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "iconUrl", default, skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<OsType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
    #[serde(rename = "termsStatus", default, skip_serializing_if = "Option::is_none")]
    pub terms_status: Option<EnableState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "sharedGalleryId", default, skip_serializing_if = "Option::is_none")]
    pub shared_gallery_id: Option<Url>,
    #[serde(rename = "availableRegions", default, skip_serializing_if = "Vec::is_empty")]
    pub available_regions: Vec<String>,
    #[serde(rename = "osState", default, skip_serializing_if = "Option::is_none")]
    pub os_state: Option<OsState>,
}
impl ImageProperties {
    pub fn new() -> Self {
        Self {
            image_update_properties: ImageUpdateProperties::default(),
            provisioning_state: None,
            display_name: None,
            description: None,
            icon_url: None,
            author: None,
            os_type: None,
            plan: None,
            terms_status: None,
            offer: None,
            publisher: None,
            sku: None,
            version: None,
            shared_gallery_id: None,
            available_regions: Vec::new(),
            os_state: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageReference {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<Url>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "exactVersion", default, skip_serializing_if = "Option::is_none")]
    pub exact_version: Option<String>,
}
impl ImageReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageUpdate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ImageUpdateProperties>,
}
impl ImageUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageUpdateProperties {
    #[serde(rename = "enabledState", default, skip_serializing_if = "Option::is_none")]
    pub enabled_state: Option<EnableState>,
}
impl ImageUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum InvitationState {
    NotSent,
    Sending,
    Sent,
    Failed,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InviteBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}
impl InviteBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Lab {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    pub properties: LabProperties,
}
impl Lab {
    pub fn new(tracked_resource: TrackedResource, properties: LabProperties) -> Self {
        Self {
            tracked_resource,
            system_data: None,
            properties,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LabNetworkProfile {
    #[serde(rename = "subnetId", default, skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<Url>,
    #[serde(rename = "loadBalancerId", default, skip_serializing_if = "Option::is_none")]
    pub load_balancer_id: Option<Url>,
    #[serde(rename = "publicIpId", default, skip_serializing_if = "Option::is_none")]
    pub public_ip_id: Option<Url>,
}
impl LabNetworkProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LabPlan {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    pub properties: LabPlanProperties,
}
impl LabPlan {
    pub fn new(tracked_resource: TrackedResource, properties: LabPlanProperties) -> Self {
        Self {
            tracked_resource,
            system_data: None,
            properties,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LabPlanNetworkProfile {
    #[serde(rename = "subnetId", default, skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<Url>,
}
impl LabPlanNetworkProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LabPlanProperties {
    #[serde(flatten)]
    pub lab_plan_update_properties: LabPlanUpdateProperties,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl LabPlanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LabPlanUpdate {
    #[serde(flatten)]
    pub tracked_resource_update: TrackedResourceUpdate,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<LabPlanUpdateProperties>,
}
impl LabPlanUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LabPlanUpdateProperties {
    #[serde(rename = "defaultConnectionProfile", default, skip_serializing_if = "Option::is_none")]
    pub default_connection_profile: Option<ConnectionProfile>,
    #[serde(rename = "defaultAutoShutdownProfile", default, skip_serializing_if = "Option::is_none")]
    pub default_auto_shutdown_profile: Option<AutoShutdownProfile>,
    #[serde(rename = "defaultNetworkProfile", default, skip_serializing_if = "Option::is_none")]
    pub default_network_profile: Option<LabPlanNetworkProfile>,
    #[serde(rename = "allowedRegions", default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_regions: Vec<String>,
    #[serde(rename = "sharedGalleryId", default, skip_serializing_if = "Option::is_none")]
    pub shared_gallery_id: Option<Url>,
    #[serde(rename = "supportInfo", default, skip_serializing_if = "Option::is_none")]
    pub support_info: Option<SupportInfo>,
    #[serde(rename = "linkedLmsInstance", default, skip_serializing_if = "Option::is_none")]
    pub linked_lms_instance: Option<Url>,
}
impl LabPlanUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LabProperties {
    #[serde(flatten)]
    pub lab_update_properties: LabUpdateProperties,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[serde(rename = "networkProfile", default, skip_serializing_if = "Option::is_none")]
    pub network_profile: Option<LabNetworkProfile>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<LabState>,
}
impl LabProperties {
    pub fn new() -> Self {
        Self {
            lab_update_properties: LabUpdateProperties::default(),
            provisioning_state: None,
            network_profile: None,
            state: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum LabState {
    Draft,
    Publishing,
    Scaling,
    Syncing,
    Published,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LabUpdate {
    #[serde(flatten)]
    pub tracked_resource_update: TrackedResourceUpdate,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<LabUpdateProperties>,
}
impl LabUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LabUpdateProperties {
    #[serde(rename = "autoShutdownProfile", default, skip_serializing_if = "Option::is_none")]
    pub auto_shutdown_profile: Option<AutoShutdownProfile>,
    #[serde(rename = "connectionProfile", default, skip_serializing_if = "Option::is_none")]
    pub connection_profile: Option<ConnectionProfile>,
    #[serde(rename = "virtualMachineProfile", default, skip_serializing_if = "Option::is_none")]
    pub virtual_machine_profile: Option<VirtualMachineProfile>,
    #[serde(rename = "securityProfile", default, skip_serializing_if = "Option::is_none")]
    pub security_profile: Option<SecurityProfile>,
    #[serde(rename = "rosterProfile", default, skip_serializing_if = "Option::is_none")]
    pub roster_profile: Option<RosterProfile>,
    #[serde(rename = "labPlanId", default, skip_serializing_if = "Option::is_none")]
    pub lab_plan_id: Option<Url>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl LabUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<operation::Origin>,
    #[serde(rename = "actionType", default, skip_serializing_if = "Option::is_none")]
    pub action_type: Option<operation::ActionType>,
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
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Origin {
        #[serde(rename = "user")]
        User,
        #[serde(rename = "system")]
        System,
        #[serde(rename = "user,system")]
        UserSystem,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ActionType {
        Internal,
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
pub struct OperationResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub status: operation_result::Status,
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(rename = "percentComplete", default, skip_serializing_if = "Option::is_none")]
    pub percent_complete: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
}
impl OperationResult {
    pub fn new(status: operation_result::Status) -> Self {
        Self {
            id: None,
            name: None,
            status,
            start_time: None,
            end_time: None,
            percent_complete: None,
            error: None,
        }
    }
}
pub mod operation_result {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        NotStarted,
        InProgress,
        Succeeded,
        Failed,
        Canceled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PagedImages {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Image>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl PagedImages {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PagedLabPlans {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<LabPlan>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl PagedLabPlans {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PagedLabs {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Lab>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl PagedLabs {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PagedSchedules {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Schedule>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl PagedSchedules {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PagedUsers {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<User>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl PagedUsers {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PagedVirtualMachines {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<VirtualMachine>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl PagedVirtualMachines {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProxyResource {
    #[serde(flatten)]
    pub resource: Resource,
}
impl ProxyResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum RecurrenceFrequency {
    Daily,
    Weekly,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecurrencePattern {
    pub frequency: RecurrenceFrequency,
    #[serde(rename = "weekDays", default, skip_serializing_if = "Vec::is_empty")]
    pub week_days: Vec<WeekDay>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<i32>,
    #[serde(rename = "expirationDate")]
    pub expiration_date: String,
}
impl RecurrencePattern {
    pub fn new(frequency: RecurrenceFrequency, expiration_date: String) -> Self {
        Self {
            frequency,
            week_days: Vec::new(),
            interval: None,
            expiration_date,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum RegistrationState {
    NotRegistered,
    Registered,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResetPasswordBody {
    pub username: String,
    pub password: String,
}
impl ResetPasswordBody {
    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
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
pub struct RosterProfile {
    #[serde(rename = "activeDirectoryGroupId", default, skip_serializing_if = "Option::is_none")]
    pub active_directory_group_id: Option<String>,
    #[serde(rename = "ltiContextId", default, skip_serializing_if = "Option::is_none")]
    pub lti_context_id: Option<String>,
    #[serde(rename = "lmsInstance", default, skip_serializing_if = "Option::is_none")]
    pub lms_instance: Option<String>,
    #[serde(rename = "ltiClientId", default, skip_serializing_if = "Option::is_none")]
    pub lti_client_id: Option<String>,
    #[serde(rename = "ltiRosterEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub lti_roster_endpoint: Option<String>,
}
impl RosterProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SaveImageBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "labVirtualMachineId", default, skip_serializing_if = "Option::is_none")]
    pub lab_virtual_machine_id: Option<Url>,
}
impl SaveImageBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Schedule {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    pub properties: ScheduleProperties,
}
impl Schedule {
    pub fn new(properties: ScheduleProperties) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            system_data: None,
            properties,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScheduleProperties {
    #[serde(flatten)]
    pub schedule_update_properties: ScheduleUpdateProperties,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl ScheduleProperties {
    pub fn new() -> Self {
        Self {
            schedule_update_properties: ScheduleUpdateProperties::default(),
            provisioning_state: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScheduleUpdate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ScheduleUpdateProperties>,
}
impl ScheduleUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScheduleUpdateProperties {
    #[serde(rename = "startAt", default, skip_serializing_if = "Option::is_none")]
    pub start_at: Option<String>,
    #[serde(rename = "stopAt", default, skip_serializing_if = "Option::is_none")]
    pub stop_at: Option<String>,
    #[serde(rename = "recurrencePattern", default, skip_serializing_if = "Option::is_none")]
    pub recurrence_pattern: Option<RecurrencePattern>,
    #[serde(rename = "timeZoneId", default, skip_serializing_if = "Option::is_none")]
    pub time_zone_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}
impl ScheduleUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecurityProfile {
    #[serde(rename = "registrationCode", default, skip_serializing_if = "Option::is_none")]
    pub registration_code: Option<String>,
    #[serde(rename = "openAccess", default, skip_serializing_if = "Option::is_none")]
    pub open_access: Option<EnableState>,
}
impl SecurityProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<sku::Tier>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
}
impl Sku {
    pub fn new(name: String) -> Self {
        Self {
            name,
            tier: None,
            size: None,
            family: None,
            capacity: None,
        }
    }
}
pub mod sku {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Tier {
        Free,
        Basic,
        Standard,
        Premium,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SupportInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<Url>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<EmailAddress>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<PhoneNumber>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
}
impl SupportInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    pub location: String,
}
impl TrackedResource {
    pub fn new(location: String) -> Self {
        Self {
            resource: Resource::default(),
            tags: None,
            location,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TrackedResourceUpdate {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
}
impl TrackedResourceUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    pub properties: UserProperties,
}
impl User {
    pub fn new(properties: UserProperties) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            system_data: None,
            properties,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserProperties {
    #[serde(flatten)]
    pub user_update_properties: UserUpdateProperties,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    pub email: EmailAddress,
    #[serde(rename = "registrationState", default, skip_serializing_if = "Option::is_none")]
    pub registration_state: Option<RegistrationState>,
    #[serde(rename = "invitationState", default, skip_serializing_if = "Option::is_none")]
    pub invitation_state: Option<InvitationState>,
    #[serde(rename = "invitationSent", default, skip_serializing_if = "Option::is_none")]
    pub invitation_sent: Option<String>,
    #[serde(rename = "totalUsage", default, skip_serializing_if = "Option::is_none")]
    pub total_usage: Option<String>,
}
impl UserProperties {
    pub fn new(email: EmailAddress) -> Self {
        Self {
            user_update_properties: UserUpdateProperties::default(),
            provisioning_state: None,
            display_name: None,
            email,
            registration_state: None,
            invitation_state: None,
            invitation_sent: None,
            total_usage: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserUpdate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<UserUpdateProperties>,
}
impl UserUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserUpdateProperties {
    #[serde(rename = "additionalUsageQuota", default, skip_serializing_if = "Option::is_none")]
    pub additional_usage_quota: Option<String>,
}
impl UserUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachine {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    pub properties: VirtualMachineProperties,
}
impl VirtualMachine {
    pub fn new(properties: VirtualMachineProperties) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            system_data: None,
            properties,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineAdditionalCapabilities {
    #[serde(rename = "installGpuDrivers", default, skip_serializing_if = "Option::is_none")]
    pub install_gpu_drivers: Option<EnableState>,
}
impl VirtualMachineAdditionalCapabilities {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineConnectionProfile {
    #[serde(rename = "privateIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<String>,
    #[serde(rename = "sshAuthority", default, skip_serializing_if = "Option::is_none")]
    pub ssh_authority: Option<String>,
    #[serde(rename = "sshInBrowserUrl", default, skip_serializing_if = "Option::is_none")]
    pub ssh_in_browser_url: Option<Url>,
    #[serde(rename = "rdpAuthority", default, skip_serializing_if = "Option::is_none")]
    pub rdp_authority: Option<String>,
    #[serde(rename = "rdpInBrowserUrl", default, skip_serializing_if = "Option::is_none")]
    pub rdp_in_browser_url: Option<Url>,
    #[serde(rename = "adminUsername", default, skip_serializing_if = "Option::is_none")]
    pub admin_username: Option<String>,
    #[serde(rename = "nonAdminUsername", default, skip_serializing_if = "Option::is_none")]
    pub non_admin_username: Option<String>,
}
impl VirtualMachineConnectionProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineProfile {
    #[serde(rename = "createOption")]
    pub create_option: virtual_machine_profile::CreateOption,
    #[serde(rename = "imageReference")]
    pub image_reference: ImageReference,
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<OsType>,
    pub sku: Sku,
    #[serde(rename = "additionalCapabilities", default, skip_serializing_if = "Option::is_none")]
    pub additional_capabilities: Option<VirtualMachineAdditionalCapabilities>,
    #[serde(rename = "usageQuota")]
    pub usage_quota: String,
    #[serde(rename = "useSharedPassword", default, skip_serializing_if = "Option::is_none")]
    pub use_shared_password: Option<EnableState>,
    #[serde(rename = "adminUser")]
    pub admin_user: Credentials,
    #[serde(rename = "nonAdminUser", default, skip_serializing_if = "Option::is_none")]
    pub non_admin_user: Option<Credentials>,
}
impl VirtualMachineProfile {
    pub fn new(
        create_option: virtual_machine_profile::CreateOption,
        image_reference: ImageReference,
        sku: Sku,
        usage_quota: String,
        admin_user: Credentials,
    ) -> Self {
        Self {
            create_option,
            image_reference,
            os_type: None,
            sku,
            additional_capabilities: None,
            usage_quota,
            use_shared_password: None,
            admin_user,
            non_admin_user: None,
        }
    }
}
pub mod virtual_machine_profile {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CreateOption {
        Image,
        #[serde(rename = "TemplateVM")]
        TemplateVm,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineProperties {
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<VirtualMachineState>,
    #[serde(rename = "connectionProfile", default, skip_serializing_if = "Option::is_none")]
    pub connection_profile: Option<VirtualMachineConnectionProfile>,
    #[serde(rename = "claimedByUserId", default, skip_serializing_if = "Option::is_none")]
    pub claimed_by_user_id: Option<String>,
    #[serde(rename = "vmType", default, skip_serializing_if = "Option::is_none")]
    pub vm_type: Option<VirtualMachineType>,
}
impl VirtualMachineProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum VirtualMachineState {
    Stopped,
    Starting,
    Running,
    Stopping,
    ResettingPassword,
    Reimaging,
    Redeploying,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum VirtualMachineType {
    User,
    Template,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum WeekDay {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ConnectionType {
    Public,
    Private,
    None,
}
pub type EmailAddress = String;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum EnableState {
    Enabled,
    Disabled,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum OsState {
    Generalized,
    Specialized,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum OsType {
    Windows,
    Linux,
}
pub type PhoneNumber = String;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ProvisioningState {
    Creating,
    Updating,
    Deleting,
    Succeeded,
    Failed,
    Locked,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ShutdownOnIdleMode {
    None,
    UserAbsence,
    LowUsage,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SystemData {
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "createdByType", default, skip_serializing_if = "Option::is_none")]
    pub created_by_type: Option<system_data::CreatedByType>,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_data::LastModifiedByType>,
    #[serde(rename = "lastModifiedAt", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_at: Option<String>,
}
impl SystemData {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod system_data {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CreatedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum LastModifiedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
    }
}
pub type Url = String;
