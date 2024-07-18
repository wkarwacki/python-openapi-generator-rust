use crate::lib::open_api::context::Context;
use serde::{de, Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub(crate) enum RefOr<T> {
    Ref {
        #[serde(rename = "$ref")]
        r#ref: String,
    },
    Item(T),
}

impl<T: de::DeserializeOwned> RefOr<T> {
    pub(crate) fn item(self) -> Option<T> {
        match self {
            RefOr::Item(item) => Some(item),
            _ => None,
        }
    }
    pub(crate) fn map_item<R>(&self, map: impl FnOnce(&T) -> R) -> RefOr<R> {
        match self {
            RefOr::Ref { r#ref } => RefOr::Ref {
                r#ref: r#ref.to_string(),
            },
            RefOr::Item(item) => RefOr::Item(map(item)),
        }
    }

    pub(crate) fn unwrap_or(self, default: T) -> T {
        match self {
            RefOr::Item(item) => item,
            RefOr::Ref { r#ref: _ } => default,
        }
    }

    pub(crate) fn unwrap(self, context: &Context) -> T {
        match self {
            RefOr::Item(item) => item,
            RefOr::Ref { r#ref } => context.resolve(r#ref),
        }
    }
}
