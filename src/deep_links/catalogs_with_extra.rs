use super::common::{MetaPreviewWithDeepLinks, ResourceLoadableWithDeepLinks};
use serde::Serialize;
use stremio_core::state_types::models::catalogs_with_extra::{CatalogsWithExtra, Selected};
use stremio_core::state_types::models::common::ResourceContent;

#[derive(Serialize)]
pub struct CatalogsWithExtraAndDeepLinks<'a> {
    pub selected: &'a Option<Selected>,
    pub catalog_resources:
        Vec<ResourceLoadableWithDeepLinks<'a, Vec<MetaPreviewWithDeepLinks<'a>>>>,
}

impl<'a> CatalogsWithExtraAndDeepLinks<'a> {
    pub fn new(catalogs_with_extra: &'a CatalogsWithExtra) -> Self {
        CatalogsWithExtraAndDeepLinks {
            selected: &catalogs_with_extra.selected,
            catalog_resources: catalogs_with_extra
                .catalog_resources
                .iter()
                .map(|resource_loadable| {
                    ResourceLoadableWithDeepLinks::new(
                        &resource_loadable.request,
                        match &resource_loadable.content {
                            ResourceContent::Ready(meta_previews) => ResourceContent::Ready(
                                meta_previews
                                    .iter()
                                    .map(MetaPreviewWithDeepLinks::new)
                                    .collect(),
                            ),
                            ResourceContent::Loading => ResourceContent::Loading,
                            ResourceContent::Err(error) => ResourceContent::Err(error.to_owned()),
                        },
                    )
                })
                .collect(),
        }
    }
}
