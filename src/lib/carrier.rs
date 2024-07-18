use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum Carrier {
    Batch,
    Stream,
}

impl Default for Carrier {
    fn default() -> Self {
        Carrier::Batch
    }
}

impl Carrier {
    pub(crate) fn is_default(&self) -> bool {
        self == &Carrier::default()
    }
}
