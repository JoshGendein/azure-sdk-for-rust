#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BasicLoginInformation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}
impl BasicLoginInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommonSku {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dev: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
}
impl CommonSku {
    pub fn new(name: String) -> Self {
        Self {
            name,
            dev: None,
            size: None,
            family: None,
            capacity: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataControllerProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub infrastructure: Option<data_controller_properties::Infrastructure>,
    #[serde(rename = "onPremiseProperty", default, skip_serializing_if = "Option::is_none")]
    pub on_premise_property: Option<OnPremiseProperty>,
    #[serde(rename = "k8sRaw", default, skip_serializing_if = "Option::is_none")]
    pub k8s_raw: Option<serde_json::Value>,
    #[serde(rename = "uploadWatermark", default, skip_serializing_if = "Option::is_none")]
    pub upload_watermark: Option<UploadWatermark>,
    #[serde(rename = "lastUploadedDate", default, skip_serializing_if = "Option::is_none")]
    pub last_uploaded_date: Option<String>,
    #[serde(rename = "basicLoginInformation", default, skip_serializing_if = "Option::is_none")]
    pub basic_login_information: Option<BasicLoginInformation>,
    #[serde(rename = "logAnalyticsWorkspaceConfig", default, skip_serializing_if = "Option::is_none")]
    pub log_analytics_workspace_config: Option<LogAnalyticsWorkspaceConfig>,
    #[serde(rename = "uploadServicePrincipal", default, skip_serializing_if = "Option::is_none")]
    pub upload_service_principal: Option<UploadServicePrincipal>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl DataControllerProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod data_controller_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Infrastructure {
        #[serde(rename = "azure")]
        Azure,
        #[serde(rename = "gcp")]
        Gcp,
        #[serde(rename = "aws")]
        Aws,
        #[serde(rename = "alibaba")]
        Alibaba,
        #[serde(rename = "onpremises")]
        Onpremises,
        #[serde(rename = "other")]
        Other,
    }
    impl Default for Infrastructure {
        fn default() -> Self {
            Self::Other
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataControllerResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(rename = "extendedLocation", default, skip_serializing_if = "Option::is_none")]
    pub extended_location: Option<ExtendedLocation>,
    pub properties: DataControllerProperties,
}
impl DataControllerResource {
    pub fn new(tracked_resource: TrackedResource, properties: DataControllerProperties) -> Self {
        Self {
            tracked_resource,
            extended_location: None,
            properties,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataControllerUpdate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl DataControllerUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponseBody>,
}
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponseBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorResponseBody>,
}
impl ErrorResponseBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtendedLocation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<ExtendedLocationType>,
}
impl ExtendedLocation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ExtendedLocationType {
    CustomLocation,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Identity {
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<identity::Type>,
}
impl Identity {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod identity {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        SystemAssigned,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum IdentityType {
    User,
    Application,
    ManagedIdentity,
    Key,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogAnalyticsWorkspaceConfig {
    #[serde(rename = "workspaceId", default, skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    #[serde(rename = "primaryKey", default, skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<String>,
}
impl LogAnalyticsWorkspaceConfig {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ODataError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ODataError>,
}
impl ODataError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OnPremiseProperty {
    pub id: String,
    #[serde(rename = "publicSigningKey")]
    pub public_signing_key: String,
    #[serde(rename = "signingCertificateThumbprint", default, skip_serializing_if = "Option::is_none")]
    pub signing_certificate_thumbprint: Option<String>,
}
impl OnPremiseProperty {
    pub fn new(id: String, public_signing_key: String) -> Self {
        Self {
            id,
            public_signing_key,
            signing_certificate_thumbprint: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    pub name: String,
    pub display: OperationDisplay,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<operation::Origin>,
    #[serde(rename = "isDataAction")]
    pub is_data_action: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl Operation {
    pub fn new(name: String, display: OperationDisplay, is_data_action: bool) -> Self {
        Self {
            name,
            display,
            origin: None,
            is_data_action,
            properties: None,
        }
    }
}
pub mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Origin {
        #[serde(rename = "user")]
        User,
        #[serde(rename = "system")]
        System,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationDisplay {
    pub provider: String,
    pub resource: String,
    pub operation: String,
    pub description: String,
}
impl OperationDisplay {
    pub fn new(provider: String, resource: String, operation: String, description: String) -> Self {
        Self {
            provider,
            resource,
            operation,
            description,
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
pub struct PageOfDataControllerResource {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DataControllerResource>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl PageOfDataControllerResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Plan {
    pub name: String,
    pub publisher: String,
    pub product: String,
    #[serde(rename = "promotionCode", default, skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl Plan {
    pub fn new(name: String, publisher: String, product: String) -> Self {
        Self {
            name,
            publisher,
            product,
            promotion_code: None,
            version: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostgresInstance {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(rename = "extendedLocation", default, skip_serializing_if = "Option::is_none")]
    pub extended_location: Option<ExtendedLocation>,
    pub properties: PostgresInstanceProperties,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<PostgresInstanceSku>,
}
impl PostgresInstance {
    pub fn new(tracked_resource: TrackedResource, properties: PostgresInstanceProperties) -> Self {
        Self {
            tracked_resource,
            extended_location: None,
            properties,
            sku: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PostgresInstanceListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PostgresInstance>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl PostgresInstanceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PostgresInstanceProperties {
    #[serde(rename = "dataControllerId", default, skip_serializing_if = "Option::is_none")]
    pub data_controller_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub admin: Option<String>,
    #[serde(rename = "basicLoginInformation", default, skip_serializing_if = "Option::is_none")]
    pub basic_login_information: Option<BasicLoginInformation>,
    #[serde(rename = "k8sRaw", default, skip_serializing_if = "Option::is_none")]
    pub k8s_raw: Option<serde_json::Value>,
    #[serde(rename = "lastUploadedDate", default, skip_serializing_if = "Option::is_none")]
    pub last_uploaded_date: Option<String>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl PostgresInstanceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostgresInstanceSku {
    #[serde(flatten)]
    pub common_sku: CommonSku,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<postgres_instance_sku::Tier>,
}
impl PostgresInstanceSku {
    pub fn new(common_sku: CommonSku) -> Self {
        Self { common_sku, tier: None }
    }
}
pub mod postgres_instance_sku {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Tier {
        Hyperscale,
    }
    impl Default for Tier {
        fn default() -> Self {
            Self::Hyperscale
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PostgresInstanceUpdate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PostgresInstanceProperties>,
}
impl PostgresInstanceUpdate {
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
pub struct ResourceSku {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}
impl ResourceSku {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlManagedInstance {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    pub properties: SqlManagedInstanceProperties,
    #[serde(rename = "extendedLocation", default, skip_serializing_if = "Option::is_none")]
    pub extended_location: Option<ExtendedLocation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<SqlManagedInstanceSku>,
}
impl SqlManagedInstance {
    pub fn new(tracked_resource: TrackedResource, properties: SqlManagedInstanceProperties) -> Self {
        Self {
            tracked_resource,
            properties,
            extended_location: None,
            sku: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlManagedInstanceListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SqlManagedInstance>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl SqlManagedInstanceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlManagedInstanceProperties {
    #[serde(rename = "dataControllerId", default, skip_serializing_if = "Option::is_none")]
    pub data_controller_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub admin: Option<String>,
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(rename = "k8sRaw", default, skip_serializing_if = "Option::is_none")]
    pub k8s_raw: Option<serde_json::Value>,
    #[serde(rename = "basicLoginInformation", default, skip_serializing_if = "Option::is_none")]
    pub basic_login_information: Option<BasicLoginInformation>,
    #[serde(rename = "lastUploadedDate", default, skip_serializing_if = "Option::is_none")]
    pub last_uploaded_date: Option<String>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(rename = "licenseType", default, skip_serializing_if = "Option::is_none")]
    pub license_type: Option<sql_managed_instance_properties::LicenseType>,
}
impl SqlManagedInstanceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sql_managed_instance_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum LicenseType {
        BasePrice,
        LicenseIncluded,
    }
    impl Default for LicenseType {
        fn default() -> Self {
            Self::BasePrice
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlManagedInstanceSku {
    pub name: sql_managed_instance_sku::Name,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<sql_managed_instance_sku::Tier>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dev: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
}
impl SqlManagedInstanceSku {
    pub fn new(name: sql_managed_instance_sku::Name) -> Self {
        Self {
            name,
            tier: None,
            dev: None,
            size: None,
            family: None,
            capacity: None,
        }
    }
}
pub mod sql_managed_instance_sku {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        #[serde(rename = "vCore")]
        VCore,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Tier {
        GeneralPurpose,
        BusinessCritical,
    }
    impl Default for Tier {
        fn default() -> Self {
            Self::GeneralPurpose
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlManagedInstanceUpdate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl SqlManagedInstanceUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlServerInstance {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SqlServerInstanceProperties>,
}
impl SqlServerInstance {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlServerInstanceListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SqlServerInstance>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl SqlServerInstanceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlServerInstanceProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edition: Option<String>,
    #[serde(rename = "containerResourceId")]
    pub container_resource_id: String,
    #[serde(rename = "createTime", default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(rename = "vCore", default, skip_serializing_if = "Option::is_none")]
    pub v_core: Option<String>,
    pub status: String,
    #[serde(rename = "patchLevel", default, skip_serializing_if = "Option::is_none")]
    pub patch_level: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collation: Option<String>,
    #[serde(rename = "currentVersion", default, skip_serializing_if = "Option::is_none")]
    pub current_version: Option<String>,
    #[serde(rename = "instanceName", default, skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    #[serde(rename = "tcpDynamicPorts", default, skip_serializing_if = "Option::is_none")]
    pub tcp_dynamic_ports: Option<String>,
    #[serde(rename = "tcpStaticPorts", default, skip_serializing_if = "Option::is_none")]
    pub tcp_static_ports: Option<String>,
    #[serde(rename = "productId", default, skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(rename = "licenseType", default, skip_serializing_if = "Option::is_none")]
    pub license_type: Option<String>,
    #[serde(rename = "azureDefenderStatusLastUpdated", default, skip_serializing_if = "Option::is_none")]
    pub azure_defender_status_last_updated: Option<String>,
    #[serde(rename = "azureDefenderStatus", default, skip_serializing_if = "Option::is_none")]
    pub azure_defender_status: Option<String>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl SqlServerInstanceProperties {
    pub fn new(container_resource_id: String, status: String) -> Self {
        Self {
            version: None,
            edition: None,
            container_resource_id,
            create_time: None,
            v_core: None,
            status,
            patch_level: None,
            collation: None,
            current_version: None,
            instance_name: None,
            tcp_dynamic_ports: None,
            tcp_static_ports: None,
            product_id: None,
            license_type: None,
            azure_defender_status_last_updated: None,
            azure_defender_status: None,
            provisioning_state: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlServerInstanceUpdate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl SqlServerInstanceUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SystemData {
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "createdByType", default, skip_serializing_if = "Option::is_none")]
    pub created_by_type: Option<IdentityType>,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<IdentityType>,
    #[serde(rename = "lastModifiedAt", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_at: Option<String>,
}
impl SystemData {
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
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl TrackedResource {
    pub fn new(location: String) -> Self {
        Self {
            resource: Resource::default(),
            tags: None,
            location,
            system_data: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UploadServicePrincipal {
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authority: Option<String>,
    #[serde(rename = "clientSecret", default, skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
}
impl UploadServicePrincipal {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UploadWatermark {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logs: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usages: Option<String>,
}
impl UploadWatermark {
    pub fn new() -> Self {
        Self::default()
    }
}
