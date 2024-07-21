use crate::lib::{def::Def, r#ref::Ref, util::read_t};
use itertools::Itertools;
use serde_yaml::Value;
use std::{collections::HashMap, fs, path::PathBuf};

#[derive(Clone, Debug)]
pub(crate) struct Context {
    pub _base: PathBuf,
    pub val: HashMap<Option<String>, Value>,
}

impl Context {
    pub(crate) fn of(path: &PathBuf) -> Context {
        let mut map = HashMap::new();
        let value: Value = read_t(path);
        map.insert(None, value.clone());
        let base = path.parent().unwrap().to_path_buf();
        Self {
            _base: fs::canonicalize(base.clone()).unwrap(),
            val: Self::get_of(&value, &base, &map),
        }
    }

    fn get_of(
        value: &Value,
        base: &PathBuf,
        map: &HashMap<Option<String>, Value>,
    ) -> HashMap<Option<String>, Value> {
        match value {
            Value::Mapping(mapping) => {
                let path = mapping.get("path");
                let src_opt = mapping.get("src").and_then(Value::as_str);
                if path.is_some() && src_opt.is_some() {
                    let src = src_opt.unwrap();
                    let mut new_map = map.clone();
                    new_map.insert(Some(src.to_string()), {
                        let value: Value =
                            read_t(&(base.to_string_lossy().to_string() + "/" + src).into());
                        value
                    });
                    new_map
                } else {
                    mapping
                        .iter()
                        .flat_map(|(_key, value)| Self::get_of(value, base, map))
                        .collect()
                }
            }
            Value::Sequence(sequence) => sequence
                .iter()
                .flat_map(|value| Self::get_of(value, base, map))
                .collect::<HashMap<_, _>>(),
            _ => map.clone(),
        }
    }
    pub(crate) fn resolve(&self, r#ref: &Ref) -> Def {
        let (src, path) = r#ref.src_and_path();
        let value = path
            .iter()
            .fold(self.val.get(&src).clone().unwrap(), |acc, val| &acc[val]);
        serde_yaml::from_value(value.clone()).unwrap()
    }

    pub(crate) fn refs(&self, def: &Def) -> Vec<(Option<String>, Vec<String>)> {
        let refs = def.refs();
        let grouped = refs.iter().into_group_map_by(|r#ref| r#ref.src.clone());
        grouped
            .iter()
            .map(|(src, refs)| {
                (
                    src.clone(),
                    refs.iter()
                        .map(|r#ref| r#ref.class_name())
                        .collect::<Vec<_>>(),
                )
            })
            .collect()
    }
}
