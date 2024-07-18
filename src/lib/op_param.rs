use crate::lib::desc::Desc;
use serde::{Deserialize, Deserializer, Serialize};
use serde_yaml::Value;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(crate) struct OpParam {
    pub loc: Option<String>,
    pub name: String,
    #[serde(flatten)]
    pub desc: Desc,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "deserialize_default"
    )]
    pub default: Option<Value>,
}

fn deserialize_default<'de, D>(deserializer: D) -> Result<Option<Value>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum DefaultValue {
        Some(Value),
        Null,
    }

    match DefaultValue::deserialize(deserializer)? {
        DefaultValue::Some(value) => Ok(Some(value)),
        DefaultValue::Null => Ok(Some(Value::Null)),
    }
}
