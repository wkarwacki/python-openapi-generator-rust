use serde::{Deserialize, Serialize};

use crate::lib::def::Def;
use crate::lib::r#ref::Ref;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Desc {
    Def(Def),
    Ref(Ref),
    Param {
        param: String
    },
}

impl Desc {
    pub fn def(&self) -> Option<&Def> {
        match self {
            Desc::Def(def) => Some(def),
            _ => None
        }
    }

    pub fn r#ref(&self) -> Option<&Ref> {
        match self {
            Desc::Ref(r#ref) => Some(r#ref),
            _ => None
        }
    }

    pub fn param(&self) -> Option<&str> {
        match self {
            Desc::Param{param} => Some(param),
            _ => None
        }
    }
}
