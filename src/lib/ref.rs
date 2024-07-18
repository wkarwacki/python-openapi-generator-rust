use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(crate) struct Ref {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src: Option<String>,
    pub path: String,
}

impl Ref {
    pub(crate) fn class_name(&self) -> String {
        self.path.split('.').last().unwrap().to_string()
    }

    pub(crate) fn src_and_path(&self) -> (Option<String>, Vec<&str>) {
        (self.src.clone(), self.path.split('.').collect())
    }
}
