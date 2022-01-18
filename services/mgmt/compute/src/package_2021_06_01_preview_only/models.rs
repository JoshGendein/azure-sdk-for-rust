#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComputeDiagnosticBase {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DiagnosticProperties>,
}
impl ComputeDiagnosticBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComputeDiagnosticsList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ComputeDiagnosticBase>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl ComputeDiagnosticsList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiagnosticProperties {
    #[serde(rename = "supportedResourceTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_resource_types: Vec<String>,
}
impl DiagnosticProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDetail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorDetail>,
    #[serde(rename = "innerError", default, skip_serializing_if = "Option::is_none")]
    pub inner_error: Option<InnerError>,
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InnerError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exceptiontype: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errordetail: Option<String>,
}
impl InnerError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RunDiskInspectionAsyncOperationResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    #[serde(rename = "resultStatus", default, skip_serializing_if = "Option::is_none")]
    pub result_status: Option<run_disk_inspection_async_operation_result::ResultStatus>,
    #[serde(rename = "errorDetail", default, skip_serializing_if = "Option::is_none")]
    pub error_detail: Option<ErrorDetail>,
    #[serde(rename = "createdUTC", default, skip_serializing_if = "Option::is_none")]
    pub created_utc: Option<String>,
}
impl RunDiskInspectionAsyncOperationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod run_disk_inspection_async_operation_result {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ResultStatus {
        Success,
        Failed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RunDiskInspectionInput {
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    pub manifest: String,
    #[serde(rename = "uploadSasUri")]
    pub upload_sas_uri: String,
}
impl RunDiskInspectionInput {
    pub fn new(resource_id: String, manifest: String, upload_sas_uri: String) -> Self {
        Self {
            resource_id,
            manifest,
            upload_sas_uri,
        }
    }
}
