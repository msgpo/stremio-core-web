use super::encoder::{encode_query_params, encode_uri_component};
use super::meta_preview::MetaPreviewWithDeepLinks;
use super::resource_loadable::{
    DeepLinks as ResourceLoadableDeepLinks, ResourceLoadableWithDeepLinks,
};
use serde::Serialize;
use stremio_core::state_types::models::catalogs_with_extra::{
    CatalogsWithExtra, Selected as CatalogsWithExtraSelected,
};
use stremio_core::state_types::models::common::{ResourceContent, ResourceLoadable};

#[derive(Serialize)]
pub struct CatalogsWithExtraAndDeepLinks<'a> {
    pub selected: &'a Option<CatalogsWithExtraSelected>,
    pub catalog_resources: Vec<ResourceLoadableWithDeepLinks<Vec<MetaPreviewWithDeepLinks<'a>>>>,
}

impl<'a> CatalogsWithExtraAndDeepLinks<'a> {
    pub fn new(catalogs_with_extra: &'a CatalogsWithExtra) -> Self {
        CatalogsWithExtraAndDeepLinks {
            selected: &catalogs_with_extra.selected,
            catalog_resources: catalogs_with_extra
                .catalog_resources
                .iter()
                .map(|resource_loadable| ResourceLoadableWithDeepLinks {
                    resource_loadable: ResourceLoadable {
                        request: resource_loadable.request.to_owned(),
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
                    },
                    deep_links: ResourceLoadableDeepLinks {
                        discover: format!(
                            "#/discover/{}/{}/{}?{}",
                            encode_uri_component(&resource_loadable.request.base),
                            encode_uri_component(&resource_loadable.request.path.type_name),
                            encode_uri_component(&resource_loadable.request.path.id),
                            encode_query_params(&resource_loadable.request.path.extra)
                        ),
                    },
                })
                .collect(),
        }
    }
}
