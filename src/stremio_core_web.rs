use crate::app_model::{AppModel, ModelFieldName};
use crate::deep_links::catalog_with_filters::CatalogWithFiltersAndDeepLinks;
use crate::deep_links::catalogs_with_extra::CatalogsWithExtraAndDeepLinks;
use env_web::Env;
use futures::future;
use futures::stream::Stream;
use std::panic;
use stremio_core::state_types::msg::{Action, Msg};
use stremio_core::state_types::{Environment, Runtime, Update, UpdateWithCtx};
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

#[wasm_bindgen]
pub struct StremioCoreWeb {
    runtime: Runtime<Env, AppModel>,
}

#[wasm_bindgen]
impl StremioCoreWeb {
    #[wasm_bindgen(constructor)]
    pub fn new(emit: js_sys::Function) -> StremioCoreWeb {
        panic::set_hook(Box::new(console_error_panic_hook::hook));
        let app = AppModel::default();
        let (runtime, rx) = Runtime::<Env, AppModel>::new(app, 1000);
        Env::exec(Box::new(rx.for_each(move |msg| {
            let _ = emit.call1(&JsValue::NULL, &JsValue::from_serde(&msg).unwrap());
            future::ok(())
        })));
        StremioCoreWeb { runtime }
    }
    pub fn dispatch(&self, action: &JsValue, model_field: &JsValue) -> JsValue {
        if let Ok(action) = action.into_serde::<Action>() {
            let message = Msg::Action(action);
            let effects = if let Ok(model_field) = model_field.into_serde::<ModelFieldName>() {
                self.runtime.dispatch_with(|model| match model_field {
                    ModelFieldName::Ctx => model.ctx.update(&message),
                    ModelFieldName::LibraryItems => {
                        model.library_items.update(&model.ctx, &message)
                    }
                    ModelFieldName::ContinueWatchingPreview => {
                        model.continue_watching_preview.update(&model.ctx, &message)
                    }
                    ModelFieldName::Board => model.board.update(&model.ctx, &message),
                    ModelFieldName::Discover => model.discover.update(&model.ctx, &message),
                    ModelFieldName::Library => model.library.update(&model.ctx, &message),
                    ModelFieldName::ContinueWatching => {
                        model.continue_watching.update(&model.ctx, &message)
                    }
                    ModelFieldName::Search => model.search.update(&model.ctx, &message),
                    ModelFieldName::MetaDetails => model.meta_details.update(&model.ctx, &message),
                    ModelFieldName::Addons => model.addons.update(&model.ctx, &message),
                    ModelFieldName::AddonDetails => {
                        model.addon_details.update(&model.ctx, &message)
                    }
                    ModelFieldName::StreamingServer => {
                        model.streaming_server.update(&model.ctx, &message)
                    }
                    ModelFieldName::Player => model.player.update(&model.ctx, &message),
                })
            } else {
                self.runtime.dispatch(&message)
            };
            Env::exec(effects);
            JsValue::TRUE
        } else {
            JsValue::FALSE
        }
    }
    pub fn get_state(&self, model_field: &JsValue) -> JsValue {
        let model = &*self.runtime.app.read().unwrap();
        match model_field.into_serde::<ModelFieldName>() {
            Ok(ModelFieldName::Ctx) => JsValue::from_serde(&model.ctx).unwrap(),
            Ok(ModelFieldName::LibraryItems) => JsValue::from_serde(&model.library_items).unwrap(),
            Ok(ModelFieldName::ContinueWatchingPreview) => {
                JsValue::from_serde(&model.continue_watching_preview).unwrap()
            }
            Ok(ModelFieldName::Board) => {
                JsValue::from_serde(&CatalogsWithExtraAndDeepLinks::new(&model.board)).unwrap()
            }
            Ok(ModelFieldName::Discover) => {
                JsValue::from_serde(&CatalogWithFiltersAndDeepLinks::new(&model.discover)).unwrap()
            }
            Ok(ModelFieldName::Library) => JsValue::from_serde(&model.library).unwrap(),
            Ok(ModelFieldName::ContinueWatching) => {
                JsValue::from_serde(&model.continue_watching).unwrap()
            }
            Ok(ModelFieldName::Search) => {
                JsValue::from_serde(&CatalogsWithExtraAndDeepLinks::new(&model.search)).unwrap()
            }
            Ok(ModelFieldName::MetaDetails) => JsValue::from_serde(&model.meta_details).unwrap(),
            Ok(ModelFieldName::Addons) => JsValue::from_serde(&model.addons).unwrap(),
            Ok(ModelFieldName::AddonDetails) => JsValue::from_serde(&model.addon_details).unwrap(),
            Ok(ModelFieldName::StreamingServer) => {
                JsValue::from_serde(&model.streaming_server).unwrap()
            }
            Ok(ModelFieldName::Player) => JsValue::from_serde(&model.player).unwrap(),
            _ => JsValue::NULL,
        }
    }
}
