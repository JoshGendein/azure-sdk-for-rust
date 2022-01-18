use crate::clients::FileSystemClient;
use azure_core::prelude::*;
use azure_core::{
    headers::{add_mandatory_header2, add_optional_header2},
    AppendToUrlQuery, Response as HttpResponse,
};
use azure_storage::core::headers::CommonStorageResponseHeaders;
use std::convert::TryInto;

/// A future of a create file system response
type DeleteFileSystem =
    futures::future::BoxFuture<'static, crate::Result<DeleteFileSystemResponse>>;

#[derive(Debug, Clone)]
pub struct DeleteFileSystemBuilder {
    client: FileSystemClient,
    if_modified_since_condition: Option<IfModifiedSinceCondition>,
    client_request_id: Option<ClientRequestId>,
    timeout: Option<Timeout>,
}

impl DeleteFileSystemBuilder {
    pub(crate) fn new(client: FileSystemClient) -> Self {
        Self {
            client,
            if_modified_since_condition: None,
            client_request_id: None,
            timeout: None,
        }
    }

    setters! {
        if_modified_since_condition: IfModifiedSinceCondition => Some(if_modified_since_condition),
        client_request_id: ClientRequestId => Some(client_request_id),
        timeout: Timeout => Some(timeout),
    }

    pub fn into_future(self) -> DeleteFileSystem {
        let this = self.clone();
        let ctx = self.client.context.clone();

        Box::pin(async move {
            let mut url = this.client.url().clone();
            self.timeout.append_to_url_query(&mut url);
            url.query_pairs_mut().append_pair("resource", "filesystem");

            let mut request = this
                .client
                .prepare_request_pipeline(url.as_str(), http::Method::DELETE);

            add_optional_header2(&this.client_request_id, &mut request)?;
            add_optional_header2(&this.if_modified_since_condition, &mut request)?;
            add_mandatory_header2(&ContentLength::new(0), &mut request)?;

            let response = self
                .client
                .pipeline()
                .send(&mut ctx.clone(), &mut request)
                .await?;

            DeleteFileSystemResponse::try_from(response).await
        })
    }
}

#[derive(Debug, Clone)]
pub struct DeleteFileSystemResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
}

impl DeleteFileSystemResponse {
    pub async fn try_from(response: HttpResponse) -> crate::Result<Self> {
        let (_status_code, headers, _pinned_stream) = response.deconstruct();

        Ok(Self {
            common_storage_response_headers: (&headers).try_into()?,
        })
    }
}
