use super::encoder::{encode_query_params, encode_uri_component};
use super::meta_preview::MetaPreviewWithDeepLinks;
use serde::Serialize;
use stremio_core::state_types::models::catalogs_with_extra::{CatalogsWithExtra, Selected};
use stremio_core::state_types::models::common::{ResourceContent, ResourceLoadable};

#[derive(Serialize)]
pub struct DeepLinks {
    pub discover: String,
}

#[derive(Serialize)]
pub struct MetaCatalogWithDeepLinks<'a> {
    #[serde(flatten)]
    pub catalog_resource: ResourceLoadable<Vec<MetaPreviewWithDeepLinks<'a>>>,
    pub deep_links: DeepLinks,
}

#[derive(Serialize)]
pub struct CatalogsWithExtraAndDeepLinks<'a> {
    pub selected: Option<Selected>,
    pub catalog_resources: Vec<MetaCatalogWithDeepLinks<'a>>,
}

impl<'a> CatalogsWithExtraAndDeepLinks<'a> {
    pub fn new(catalogs_with_extra: &'a CatalogsWithExtra) -> Self {
        CatalogsWithExtraAndDeepLinks {
            selected: catalogs_with_extra.selected.to_owned(),
            catalog_resources: catalogs_with_extra
                .catalog_resources
                .iter()
                .map(|catalog_resource| MetaCatalogWithDeepLinks {
                    catalog_resource: ResourceLoadable {
                        request: catalog_resource.request.to_owned(),
                        content: match &catalog_resource.content {
                            ResourceContent::Ready(meta_previews) => ResourceContent::Ready(
                                meta_previews
                                    .iter()
                                    .map(MetaPreviewWithDeepLinks::new)
                                    .collect(),
                            ),
                            ResourceContent::Loading => ResourceContent::Loading,
                            ResourceContent::Err(error) => ResourceContent::Err(error.to_owned()),
                        },
                    },
                    deep_links: DeepLinks {
                        discover: format!(
                            "#/discover/{}/{}/{}?{}",
                            encode_uri_component(&catalog_resource.request.base),
                            encode_uri_component(&catalog_resource.request.path.type_name),
                            encode_uri_component(&catalog_resource.request.path.id),
                            encode_query_params(&catalog_resource.request.path.extra)
                        ),
                    },
                })
                .collect(),
        }
    }
}
