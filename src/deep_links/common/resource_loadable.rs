use super::{encode_query_params, encode_uri_component, MetaPreviewWithDeepLinks};
use serde::Serialize;
use stremio_core::state_types::models::common::ResourceContent;
use stremio_core::types::addons::ResourceRequest;

#[derive(Serialize)]
#[serde(untagged)]
pub enum DeepLinks {
    MetaCatalog { discover: String },
}

#[derive(Serialize)]
pub struct ResourceLoadableWithDeepLinks<'a, T> {
    pub request: &'a ResourceRequest,
    pub content: ResourceContent<T>,
    pub deep_links: DeepLinks,
}

impl<'a> ResourceLoadableWithDeepLinks<'a, Vec<MetaPreviewWithDeepLinks<'a>>> {
    pub fn new(
        request: &'a ResourceRequest,
        content: ResourceContent<Vec<MetaPreviewWithDeepLinks<'a>>>,
    ) -> Self {
        ResourceLoadableWithDeepLinks {
            request,
            content,
            deep_links: DeepLinks::MetaCatalog {
                discover: format!(
                    "#/discover/{}/{}/{}?{}",
                    encode_uri_component(&request.base),
                    encode_uri_component(&request.path.type_name),
                    encode_uri_component(&request.path.id),
                    encode_query_params(&request.path.extra)
                ),
            },
        }
    }
}
