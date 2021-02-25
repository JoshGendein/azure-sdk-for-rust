#![doc = "generated by AutoRust 0.1.0"]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use crate::models::*;
use snafu::{ResultExt, Snafu};
pub mod metric_baseline {
    use crate::models::*;
    use snafu::{ResultExt, Snafu};
    pub async fn get(
        operation_config: &crate::OperationConfig,
        resource_uri: &str,
        metric_name: &str,
        timespan: Option<&str>,
        interval: Option<&str>,
        aggregation: Option<&str>,
        sensitivities: Option<&str>,
        result_type: Option<&str>,
    ) -> std::result::Result<BaselineResponse, get::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/{}/providers/Microsoft.Insights/baseline/{}",
            operation_config.base_path(),
            resource_uri,
            metric_name
        );
        let mut url = url::Url::parse(url_str).context(get::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::GET);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .context(get::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        if let Some(timespan) = timespan {
            url.query_pairs_mut().append_pair("timespan", timespan);
        }
        if let Some(interval) = interval {
            url.query_pairs_mut().append_pair("interval", interval);
        }
        if let Some(aggregation) = aggregation {
            url.query_pairs_mut().append_pair("aggregation", aggregation);
        }
        if let Some(sensitivities) = sensitivities {
            url.query_pairs_mut().append_pair("sensitivities", sensitivities);
        }
        if let Some(result_type) = result_type {
            url.query_pairs_mut().append_pair("resultType", result_type);
        }
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).context(get::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.context(get::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: BaselineResponse =
                    serde_json::from_slice(rsp_body).context(get::DeserializeError { body: rsp_body.clone() })?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: ErrorResponse =
                    serde_json::from_slice(rsp_body).context(get::DeserializeError { body: rsp_body.clone() })?;
                get::DefaultResponse {
                    status_code,
                    value: rsp_value,
                }
                .fail()
            }
        }
    }
    pub mod get {
        use crate::{models, models::*};
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::ErrorResponse,
            },
            ParseUrlError {
                source: url::ParseError,
            },
            BuildRequestError {
                source: http::Error,
            },
            ExecuteRequestError {
                source: Box<dyn std::error::Error + Sync + Send>,
            },
            SerializeError {
                source: Box<dyn std::error::Error + Sync + Send>,
            },
            DeserializeError {
                source: serde_json::Error,
                body: bytes::Bytes,
            },
            GetTokenError {
                source: azure_core::errors::AzureError,
            },
        }
    }
    pub async fn calculate_baseline(
        operation_config: &crate::OperationConfig,
        resource_uri: &str,
        time_series_information: &TimeSeriesInformation,
    ) -> std::result::Result<CalculateBaselineResponse, calculate_baseline::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/{}/providers/Microsoft.Insights/calculatebaseline",
            operation_config.base_path(),
            resource_uri
        );
        let mut url = url::Url::parse(url_str).context(calculate_baseline::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::POST);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .context(calculate_baseline::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = azure_core::to_json(time_series_information).context(calculate_baseline::SerializeError)?;
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).context(calculate_baseline::BuildRequestError)?;
        let rsp = http_client
            .execute_request(req)
            .await
            .context(calculate_baseline::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: CalculateBaselineResponse =
                    serde_json::from_slice(rsp_body).context(calculate_baseline::DeserializeError { body: rsp_body.clone() })?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: ErrorResponse =
                    serde_json::from_slice(rsp_body).context(calculate_baseline::DeserializeError { body: rsp_body.clone() })?;
                calculate_baseline::DefaultResponse {
                    status_code,
                    value: rsp_value,
                }
                .fail()
            }
        }
    }
    pub mod calculate_baseline {
        use crate::{models, models::*};
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::ErrorResponse,
            },
            ParseUrlError {
                source: url::ParseError,
            },
            BuildRequestError {
                source: http::Error,
            },
            ExecuteRequestError {
                source: Box<dyn std::error::Error + Sync + Send>,
            },
            SerializeError {
                source: Box<dyn std::error::Error + Sync + Send>,
            },
            DeserializeError {
                source: serde_json::Error,
                body: bytes::Bytes,
            },
            GetTokenError {
                source: azure_core::errors::AzureError,
            },
        }
    }
}