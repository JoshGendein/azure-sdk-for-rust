#![doc = "generated by AutoRust"]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use super::models;
#[derive(Clone)]
pub struct Client {
    endpoint: String,
    credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
    scopes: Vec<String>,
    pipeline: azure_core::Pipeline,
}
#[derive(Clone)]
pub struct ClientBuilder {
    credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
    endpoint: Option<String>,
    scopes: Option<Vec<String>>,
}
pub const DEFAULT_ENDPOINT: &str = azure_core::resource_manager_endpoint::AZURE_PUBLIC_CLOUD;
impl ClientBuilder {
    pub fn new(credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>) -> Self {
        Self {
            credential,
            endpoint: None,
            scopes: None,
        }
    }
    pub fn endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoint = Some(endpoint.into());
        self
    }
    pub fn scopes(mut self, scopes: &[&str]) -> Self {
        self.scopes = Some(scopes.iter().map(|scope| (*scope).to_owned()).collect());
        self
    }
    pub fn build(self) -> Client {
        let endpoint = self.endpoint.unwrap_or_else(|| DEFAULT_ENDPOINT.to_owned());
        let scopes = self.scopes.unwrap_or_else(|| vec![format!("{}/", endpoint)]);
        Client::new(endpoint, self.credential, scopes)
    }
}
impl Client {
    pub(crate) fn endpoint(&self) -> &str {
        self.endpoint.as_str()
    }
    pub(crate) fn token_credential(&self) -> &dyn azure_core::auth::TokenCredential {
        self.credential.as_ref()
    }
    pub(crate) fn scopes(&self) -> Vec<&str> {
        self.scopes.iter().map(String::as_str).collect()
    }
    pub(crate) async fn send(&self, request: impl Into<azure_core::Request>) -> Result<azure_core::Response, azure_core::Error> {
        let mut context = azure_core::Context::default();
        let mut request = request.into();
        self.pipeline.send(&mut context, &mut request).await
    }
    pub fn new(
        endpoint: impl Into<String>,
        credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
        scopes: Vec<String>,
    ) -> Self {
        let endpoint = endpoint.into();
        let pipeline = azure_core::Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            azure_core::ClientOptions::default(),
            Vec::new(),
            Vec::new(),
        );
        Self {
            endpoint,
            credential,
            scopes,
            pipeline,
        }
    }
    pub fn metric_definitions(&self) -> metric_definitions::Client {
        metric_definitions::Client(self.clone())
    }
    pub fn metrics(&self) -> metrics::Client {
        metrics::Client(self.clone())
    }
}
#[non_exhaustive]
#[derive(Debug, thiserror :: Error)]
#[allow(non_camel_case_types)]
pub enum Error {
    #[error(transparent)]
    MetricDefinitions_List(#[from] metric_definitions::list::Error),
    #[error(transparent)]
    Metrics_List(#[from] metrics::list::Error),
}
pub mod metric_definitions {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        pub fn list(&self, resource_uri: impl Into<String>) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                resource_uri: resource_uri.into(),
                metricnamespace: None,
            }
        }
    }
    pub mod list {
        use super::models;
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::ErrorResponse,
            },
            #[error("Failed to parse request URL")]
            ParseUrl(#[source] url::ParseError),
            #[error("Failed to build request")]
            BuildRequest(#[source] http::Error),
            #[error("Failed to serialize request body")]
            Serialize(#[source] serde_json::Error),
            #[error("Failed to get access token")]
            GetToken(#[source] azure_core::Error),
            #[error("Failed to execute request")]
            SendRequest(#[source] azure_core::Error),
            #[error("Failed to get response bytes")]
            ResponseBytes(#[source] azure_core::StreamError),
            #[error("Failed to deserialize response, body: {1:?}")]
            Deserialize(#[source] serde_json::Error, bytes::Bytes),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_uri: String,
            pub(crate) metricnamespace: Option<String>,
        }
        impl Builder {
            pub fn metricnamespace(mut self, metricnamespace: impl Into<String>) -> Self {
                self.metricnamespace = Some(metricnamespace.into());
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, std::result::Result<models::MetricDefinitionCollection, Error>> {
                Box::pin(async move {
                    let url_str = &format!(
                        "{}/{}/providers/Microsoft.Insights/metricDefinitions",
                        self.client.endpoint(),
                        &self.resource_uri
                    );
                    let mut url = url::Url::parse(url_str).map_err(Error::ParseUrl)?;
                    let mut req_builder = http::request::Builder::new();
                    req_builder = req_builder.method(http::Method::GET);
                    let credential = self.client.token_credential();
                    let token_response = credential
                        .get_token(&self.client.scopes().join(" "))
                        .await
                        .map_err(Error::GetToken)?;
                    req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
                    url.query_pairs_mut().append_pair("api-version", "2018-01-01");
                    if let Some(metricnamespace) = &self.metricnamespace {
                        url.query_pairs_mut().append_pair("metricnamespace", metricnamespace);
                    }
                    let req_body = azure_core::EMPTY_BODY;
                    req_builder = req_builder.uri(url.as_str());
                    let req = req_builder.body(req_body).map_err(Error::BuildRequest)?;
                    let rsp = self.client.send(req).await.map_err(Error::SendRequest)?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        http::StatusCode::OK => {
                            let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await.map_err(Error::ResponseBytes)?;
                            let rsp_value: models::MetricDefinitionCollection =
                                serde_json::from_slice(&rsp_body).map_err(|source| Error::Deserialize(source, rsp_body.clone()))?;
                            Ok(rsp_value)
                        }
                        status_code => {
                            let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await.map_err(Error::ResponseBytes)?;
                            let rsp_value: models::ErrorResponse =
                                serde_json::from_slice(&rsp_body).map_err(|source| Error::Deserialize(source, rsp_body.clone()))?;
                            Err(Error::DefaultResponse {
                                status_code,
                                value: rsp_value,
                            })
                        }
                    }
                })
            }
        }
    }
}
pub mod metrics {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        pub fn list(&self, resource_uri: impl Into<String>) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                resource_uri: resource_uri.into(),
                timespan: None,
                interval: None,
                metricnames: None,
                aggregation: None,
                top: None,
                orderby: None,
                filter: None,
                result_type: None,
                metricnamespace: None,
            }
        }
    }
    pub mod list {
        use super::models;
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::ErrorResponse,
            },
            #[error("Failed to parse request URL")]
            ParseUrl(#[source] url::ParseError),
            #[error("Failed to build request")]
            BuildRequest(#[source] http::Error),
            #[error("Failed to serialize request body")]
            Serialize(#[source] serde_json::Error),
            #[error("Failed to get access token")]
            GetToken(#[source] azure_core::Error),
            #[error("Failed to execute request")]
            SendRequest(#[source] azure_core::Error),
            #[error("Failed to get response bytes")]
            ResponseBytes(#[source] azure_core::StreamError),
            #[error("Failed to deserialize response, body: {1:?}")]
            Deserialize(#[source] serde_json::Error, bytes::Bytes),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_uri: String,
            pub(crate) timespan: Option<String>,
            pub(crate) interval: Option<String>,
            pub(crate) metricnames: Option<String>,
            pub(crate) aggregation: Option<String>,
            pub(crate) top: Option<i32>,
            pub(crate) orderby: Option<String>,
            pub(crate) filter: Option<String>,
            pub(crate) result_type: Option<String>,
            pub(crate) metricnamespace: Option<String>,
        }
        impl Builder {
            pub fn timespan(mut self, timespan: impl Into<String>) -> Self {
                self.timespan = Some(timespan.into());
                self
            }
            pub fn interval(mut self, interval: impl Into<String>) -> Self {
                self.interval = Some(interval.into());
                self
            }
            pub fn metricnames(mut self, metricnames: impl Into<String>) -> Self {
                self.metricnames = Some(metricnames.into());
                self
            }
            pub fn aggregation(mut self, aggregation: impl Into<String>) -> Self {
                self.aggregation = Some(aggregation.into());
                self
            }
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            pub fn orderby(mut self, orderby: impl Into<String>) -> Self {
                self.orderby = Some(orderby.into());
                self
            }
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn result_type(mut self, result_type: impl Into<String>) -> Self {
                self.result_type = Some(result_type.into());
                self
            }
            pub fn metricnamespace(mut self, metricnamespace: impl Into<String>) -> Self {
                self.metricnamespace = Some(metricnamespace.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, std::result::Result<models::Response, Error>> {
                Box::pin(async move {
                    let url_str = &format!(
                        "{}/{}/providers/Microsoft.Insights/metrics",
                        self.client.endpoint(),
                        &self.resource_uri
                    );
                    let mut url = url::Url::parse(url_str).map_err(Error::ParseUrl)?;
                    let mut req_builder = http::request::Builder::new();
                    req_builder = req_builder.method(http::Method::GET);
                    let credential = self.client.token_credential();
                    let token_response = credential
                        .get_token(&self.client.scopes().join(" "))
                        .await
                        .map_err(Error::GetToken)?;
                    req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
                    url.query_pairs_mut().append_pair("api-version", "2018-01-01");
                    if let Some(timespan) = &self.timespan {
                        url.query_pairs_mut().append_pair("timespan", timespan);
                    }
                    if let Some(interval) = &self.interval {
                        url.query_pairs_mut().append_pair("interval", interval);
                    }
                    if let Some(metricnames) = &self.metricnames {
                        url.query_pairs_mut().append_pair("metricnames", metricnames);
                    }
                    if let Some(aggregation) = &self.aggregation {
                        url.query_pairs_mut().append_pair("aggregation", aggregation);
                    }
                    if let Some(top) = &self.top {
                        url.query_pairs_mut().append_pair("top", &top.to_string());
                    }
                    if let Some(orderby) = &self.orderby {
                        url.query_pairs_mut().append_pair("orderby", orderby);
                    }
                    if let Some(filter) = &self.filter {
                        url.query_pairs_mut().append_pair("$filter", filter);
                    }
                    if let Some(result_type) = &self.result_type {
                        url.query_pairs_mut().append_pair("resultType", result_type);
                    }
                    if let Some(metricnamespace) = &self.metricnamespace {
                        url.query_pairs_mut().append_pair("metricnamespace", metricnamespace);
                    }
                    let req_body = azure_core::EMPTY_BODY;
                    req_builder = req_builder.uri(url.as_str());
                    let req = req_builder.body(req_body).map_err(Error::BuildRequest)?;
                    let rsp = self.client.send(req).await.map_err(Error::SendRequest)?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        http::StatusCode::OK => {
                            let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await.map_err(Error::ResponseBytes)?;
                            let rsp_value: models::Response =
                                serde_json::from_slice(&rsp_body).map_err(|source| Error::Deserialize(source, rsp_body.clone()))?;
                            Ok(rsp_value)
                        }
                        status_code => {
                            let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await.map_err(Error::ResponseBytes)?;
                            let rsp_value: models::ErrorResponse =
                                serde_json::from_slice(&rsp_body).map_err(|source| Error::Deserialize(source, rsp_body.clone()))?;
                            Err(Error::DefaultResponse {
                                status_code,
                                value: rsp_value,
                            })
                        }
                    }
                })
            }
        }
    }
}
