use super::common::{MetaPreviewWithDeepLinks, ResourceLoadableWithDeepLinks};
use serde::Serialize;
use stremio_core::state_types::models::catalogs_with_extra::{CatalogsWithExtra, Selected};

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
                .map(ResourceLoadableWithDeepLinks::new)
                .collect(),
        }
    }
}
