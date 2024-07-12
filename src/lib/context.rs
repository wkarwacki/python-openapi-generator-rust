use crate::lib::{def::Def, pkg::Pkg, r#ref::Ref, util::read_t};
use serde_yaml::Value;
use std::{collections::HashMap, fs, path::PathBuf};

#[derive(Clone, Debug)]
pub struct Context {
    pub base: PathBuf,
    pub val: HashMap<Option<String>, Value>,
}

impl Context {
    pub fn of(path: PathBuf) -> Context {
        let mut map = HashMap::new();
        let value: Value = read_t(path.clone());
        map.insert(None, value.clone());
        let base = path.parent().unwrap().to_path_buf();
        Self {
            base: fs::canonicalize(base.clone()).unwrap(),
            val: Self::get_of(value, base, map),
        }
    }

    fn get_of(
        value: Value,
        base: PathBuf,
        map: HashMap<Option<String>, Value>,
    ) -> HashMap<Option<String>, Value> {
        match value {
            Value::Mapping(mapping) => {
                let path = mapping.get("path");
                let src_opt = mapping.get("src").and_then(|src| src.as_str());
                if path.is_some() && src_opt.is_some() {
                    let src = src_opt.unwrap();
                    let mut new_map = map.clone();
                    new_map.insert(Some(src.to_string()), {
                        let value: Value =
                            read_t((base.to_string_lossy().to_string() + "/" + src).into());
                        value
                    });
                    new_map
                } else {
                    mapping
                        .iter()
                        .flat_map(|(_key, value)| {
                            Self::get_of(value.clone(), base.clone(), map.clone())
                        })
                        .collect()
                }
            }
            Value::Sequence(sequence) => sequence
                .iter()
                .flat_map(|value| Self::get_of(value.clone(), base.clone(), map.clone()))
                .collect::<HashMap<_, _>>(),
            _ => map,
        }
    }
    pub fn resolve(&self, r#ref: Ref) -> Def {
        let (src, path) = r#ref.src_and_path();
        let value = path
            .iter()
            .fold(self.val.get(&src).clone().unwrap(), |acc, val| &acc[val]);
        serde_yaml::from_value(value.clone()).unwrap()
    }

    pub fn defs(&self) -> Vec<(Option<String>, Vec<String>)> {
        self.val
            .iter()
            .map(|(src, value)| {
                let pkg: Pkg = serde_yaml::from_value(value.clone()).unwrap();
                (src.clone(), pkg.defs.keys().cloned().collect::<Vec<_>>())
            })
            .collect()
    }
}
