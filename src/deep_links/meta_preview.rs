use super::encoder::encode_uri_component;
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

impl<'a> MetaPreviewWithDeepLinks<'a> {
    pub fn new(meta_preview: &'a MetaPreview) -> Self {
        MetaPreviewWithDeepLinks {
            meta_preview,
            deep_links: DeepLinks {
                meta_details_videos: format!(
                    "#/metadetails/{}/{}",
                    encode_uri_component(&meta_preview.type_name),
                    encode_uri_component(&meta_preview.id)
                ),
                meta_details_streams: meta_preview.behavior_hints.default_video_id.as_ref().map(
                    |default_video_id| {
                        format!(
                            "#/metadetails/{}/{}/{}",
                            encode_uri_component(&meta_preview.type_name),
                            encode_uri_component(&meta_preview.id),
                            encode_uri_component(&default_video_id)
                        )
                    },
                ),
            },
        }
    }
}
