use serde::Serialize;
use stremio_core::state_types::models::common::ResourceLoadable;

#[derive(Serialize)]
pub struct DeepLinks {
    pub discover: String,
}

#[derive(Serialize)]
pub struct ResourceLoadableWithDeepLinks<T> {
    #[serde(flatten)]
    pub resource_loadable: ResourceLoadable<T>,
    pub deep_links: DeepLinks,
}
