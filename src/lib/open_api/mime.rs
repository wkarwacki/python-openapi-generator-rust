use mime;
use serde::{Deserialize, Deserializer, Serialize};
use serde_yaml::Value;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) struct Mime {
    pub val: mime::Mime,
}

impl<'de> Deserialize<'de> for Mime {
    fn deserialize<D>(deserializer: D) -> Result<Mime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: Value = Deserialize::deserialize(deserializer).unwrap();
        match value {
            Value::String(string) => Ok(Mime {
                val: mime::Mime::from_str(string.as_str()).unwrap(),
            }),
            _ => panic!("Illegal mime type."),
        }
    }
}

impl Serialize for Mime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.val.to_string().serialize(serializer)
    }
}

impl Mime {
    pub(crate) fn of(string: Option<String>) -> Mime {
        Mime {
            val: string
                .map(|s| mime::Mime::from_str(s.as_str()).unwrap())
                .unwrap_or(mime::APPLICATION_JSON),
        }
    }
}
