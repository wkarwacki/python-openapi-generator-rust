use core::fmt::Debug;
use std::collections::HashMap;
use std::ops::Not;

use serde::{Deserialize, Serialize};
use serde_yaml::Value;

use crate::lib::adt::Adt;
use crate::lib::desc::Desc;
use crate::lib::ext::Ext;
use crate::lib::r#ref::Ref;
use crate::lib::var::Var;

pub static DEFS: &str = "defs";

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum Def {
    Alias(Alias),
    Bool(Bool),
    Const(Const),
    Dec(Dec),
    Enum(Enum),
    Int(Int),
    Map(Box<Map>),
    Obj(Obj),
    Seq(Box<Seq>),
    Str(Str),
    Struct(Struct),
}

impl Def {
    pub fn obj(&self) -> Option<&Obj> {
        match self {
            Def::Obj(obj) => Some(obj),
            _ => None
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Alias {
    #[serde(flatten)]
    pub r#ref: Ref
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Bool {
    #[serde(default)]
    #[serde(skip_serializing_if = "<&bool>::not")]
    pub null: bool
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Const {
    pub val: Value,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub desc: Option<Box<Desc>>
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Dec {
    #[serde(default)]
    #[serde(skip_serializing_if = "<&bool>::not")]
    pub null: bool
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Enum {
    pub vals: EnumVals,
    #[serde(default, skip_serializing_if = "<&bool>::not")]
    pub null: bool
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum EnumVals {
    Int(Vec<i64>),
    Str(Vec<String>),
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Int {
    #[serde(default)]
    #[serde(skip_serializing_if = "<&bool>::not")]
    pub null: bool
}

// TODO_LATER: for http gen, validation, whether a key is string-like
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Map {
    pub key: Desc,
    pub val: Desc,
    #[serde(default)]
    #[serde(skip_serializing_if = "<&bool>::not")]
    pub null: bool
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Obj {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Ext>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub mix: Vec<Ref>,
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub vars: HashMap<String, Box<Var>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adt: Option<Adt>,
    #[serde(default)]
    #[serde(skip_serializing_if = "<&bool>::not")]
    pub null: bool

}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Seq {
    pub item: Desc,
    #[serde(default)]
    #[serde(skip_serializing_if = "<&bool>::not")]
    pub null: bool

}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Str {
    #[serde(default)]
    #[serde(skip_serializing_if = "<&bool>::not")]
    pub null: bool
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Struct {}