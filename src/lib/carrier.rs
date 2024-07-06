use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Carrier {
    Batch,
    Stream
}

impl Default for Carrier {
    fn default() -> Self {
        Carrier::Batch
    }
}

impl Carrier {
    pub fn is_default(&self) -> bool {
        self == &Carrier::default()
    }
}
