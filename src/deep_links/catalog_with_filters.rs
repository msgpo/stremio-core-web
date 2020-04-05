use super::common::MetaPreviewWithDeepLinks;
use serde::Serialize;
use stremio_core::state_types::models::catalog_with_filters::{
    CatalogWithFilters, Selectable, Selected,
};
use stremio_core::state_types::models::common::{ResourceContent, ResourceLoadable};
use stremio_core::types::MetaPreview;

#[derive(Serialize)]
pub struct CatalogWithFiltersAndDeepLinks<'a, T> {
    pub selected: &'a Option<Selected>,
    pub selectable: &'a Selectable,
    pub catalog_resource: Option<ResourceLoadable<Vec<T>>>,
}

impl<'a> CatalogWithFiltersAndDeepLinks<'a, MetaPreviewWithDeepLinks<'a>> {
    pub fn new(catalog_with_filters: &'a CatalogWithFilters<MetaPreview>) -> Self {
        CatalogWithFiltersAndDeepLinks {
            selected: &catalog_with_filters.selected,
            selectable: &catalog_with_filters.selectable,
            catalog_resource: catalog_with_filters.catalog_resource.as_ref().map(
                |catalog_resource| ResourceLoadable {
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
            ),
        }
    }
}
