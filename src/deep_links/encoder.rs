use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use stremio_core::types::addons::ExtraProp;

pub fn encode_uri_component(component: &str) -> String {
    utf8_percent_encode(component, NON_ALPHANUMERIC).to_string()
}

pub fn encode_query_params(query_params: &[ExtraProp]) -> String {
    query_params
        .iter()
        .map(|(name, value)| {
            format!(
                "{}={}",
                encode_uri_component(&name),
                encode_uri_component(&value)
            )
        })
        .collect::<Vec<_>>()
        .join("&")
}
