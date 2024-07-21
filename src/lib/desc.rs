use crate::lib::{def::Def, r#ref::Ref};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub(crate) enum Desc {
    Def(Def),
    Ref(Ref),
    TypeParam { param: String },
}

impl Desc {
    pub(crate) fn refs(&self) -> Vec<Ref> {
        match self {
            Desc::Def(def) => def.refs(),
            Desc::Ref(r#ref) => vec![r#ref.clone()],
            _ => Vec::new(),
        }
    }
    pub(crate) fn def(&self) -> Option<&Def> {
        match self {
            Desc::Def(def) => Some(def),
            _ => None,
        }
    }

    pub(crate) fn r#ref(&self) -> Option<&Ref> {
        match self {
            Desc::Ref(r#ref) => Some(r#ref),
            _ => None,
        }
    }

    pub(crate) fn param(&self) -> Option<&str> {
        match self {
            Desc::TypeParam { param } => Some(param),
            _ => None,
        }
    }
}
