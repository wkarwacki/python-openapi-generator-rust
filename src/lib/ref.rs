use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Ref {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src: Option<String>,
    pub path: String,
}

impl Ref {
    pub fn class_name(&self) -> String {
        self.path.split('.').last().unwrap().to_string()
    }

    pub fn src_and_path(&self) -> (Option<String>, Vec<&str>) {
        (self.src.clone(), self.path.split('.').collect())
    }

    pub fn to_string(self) -> String {
        self.src
            .map(|src| src + "#" + self.path.as_str())
            .unwrap_or(self.path)
    }
}
