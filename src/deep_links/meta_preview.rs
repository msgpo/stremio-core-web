use serde::Serialize;
use stremio_core::types::MetaPreview;

#[derive(Serialize)]
pub struct DeepLinks {
    pub meta_details_videos: String,
    pub meta_details_streams: Option<String>,
}

#[derive(Serialize)]
pub struct MetaPreviewWithDeepLinks<'a> {
    #[serde(flatten)]
    pub meta_preview: &'a MetaPreview,
    pub deep_links: DeepLinks,
}
