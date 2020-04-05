use super::{encode_query_params, encode_uri_component, MetaPreviewWithDeepLinks};
use serde::Serialize;
use stremio_core::state_types::models::common::{ResourceContent, ResourceLoadable};
use stremio_core::types::addons::ResourceRequest;
use stremio_core::types::MetaPreview;

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
    pub fn new(resource_loadable: &'a ResourceLoadable<Vec<MetaPreview>>) -> Self {
        ResourceLoadableWithDeepLinks {
            request: &resource_loadable.request,
            content: match &resource_loadable.content {
                ResourceContent::Ready(meta_previews) => ResourceContent::Ready(
                    meta_previews
                        .iter()
                        .map(MetaPreviewWithDeepLinks::new)
                        .collect(),
                ),
                ResourceContent::Loading => ResourceContent::Loading,
                ResourceContent::Err(error) => ResourceContent::Err(error.to_owned()),
            },
            deep_links: DeepLinks::MetaCatalog {
                discover: format!(
                    "#/discover/{}/{}/{}?{}",
                    encode_uri_component(&resource_loadable.request.base),
                    encode_uri_component(&resource_loadable.request.path.type_name),
                    encode_uri_component(&resource_loadable.request.path.id),
                    encode_query_params(&resource_loadable.request.path.extra)
                ),
            },
        }
    }
}
