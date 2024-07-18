use http::Method;
use serde::{Deserialize, Deserializer, Serialize};
use serde_yaml::Value;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) struct StatusCode {
    pub val: http::StatusCode,
}

impl<'de> Deserialize<'de> for StatusCode {
    fn deserialize<D>(deserializer: D) -> Result<StatusCode, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: Value = Deserialize::deserialize(deserializer).unwrap();
        match value {
            Value::Number(number) => Ok(StatusCode {
                val: http::StatusCode::from_str(number.to_string().as_str()).unwrap(),
            }),
            Value::String(string) => Ok(StatusCode {
                val: http::StatusCode::from_str(string.as_str()).unwrap(),
            }),
            _ => panic!("Illegal status_code type."),
        }
    }
}

impl Serialize for StatusCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.val.as_str().serialize(serializer)
    }
}

impl StatusCode {
    pub(crate) fn of(method: Method) -> StatusCode {
        match method {
            Method::GET => StatusCode {
                val: http::StatusCode::OK,
            },
            Method::POST => StatusCode {
                val: http::StatusCode::CREATED,
            },
            Method::PUT => StatusCode {
                val: http::StatusCode::NO_CONTENT,
            },
            Method::PATCH => StatusCode {
                val: http::StatusCode::OK,
            },
            Method::DELETE => StatusCode {
                val: http::StatusCode::NO_CONTENT,
            },
            _ => StatusCode {
                val: http::StatusCode::METHOD_NOT_ALLOWED,
            },
        }
    }
}
