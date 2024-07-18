use crate::lib::{open_api::open_api::OpenApi, util::read_t};
use serde::de;
use serde_yaml::Value;
use std::{collections::HashMap, fs, path::PathBuf};

#[derive(Debug)]
pub(crate) struct Context {
    pub _base: PathBuf,
    pub val: HashMap<Option<String>, Value>,
}

impl Context {
    // FIXME_LATER: clean up algorithm + it should only take nodes that are referenced, not whole files
    pub(crate) fn of(path: &PathBuf) -> Self {
        let mut map = HashMap::new();
        let value: Value = read_t(path);
        map.insert(None, value.clone());
        map.insert(
            path.file_name().unwrap().to_str().map(|s| s.to_string()),
            value.clone(),
        );
        let base = path.parent().unwrap().to_path_buf();
        Self::get_of(value, base.clone(), &mut map);
        Self {
            _base: fs::canonicalize(base.clone()).unwrap(),
            val: map,
        }
    }

    fn get_of(value: Value, base: PathBuf, map: &mut HashMap<Option<String>, Value>) {
        match value {
            Value::Mapping(ref mapping) => {
                let r#ref = mapping.get("$ref");
                r#ref
                    .and_then(|r| {
                        let (src, _) = Self::src_and_path(r.as_str().unwrap().to_string());
                        src
                    })
                    .iter()
                    .for_each(|src| {
                        let base_src = base.to_string_lossy().to_string() + "/" + src.as_str();
                        if !map.contains_key(&Some(src.clone())) {
                            let value: Value = read_t(&base_src.into());
                            map.insert(Some(src.clone()), value.clone());
                            Self::get_of(value, base.clone(), map);
                        }
                    });
                mapping
                    .iter()
                    .for_each(|(_, v)| Self::get_of(v.clone(), base.clone(), map));
            }
            Value::Sequence(sequence) => sequence
                .iter()
                .for_each(|value| Self::get_of(value.clone(), base.clone(), map)),
            _ => (),
        }
    }

    // TODO: REFACTOR
    pub(crate) fn resolve<T: de::DeserializeOwned>(&self, r#ref: String) -> T {
        let (src, path) = Self::src_and_path(r#ref.clone());
        let value = path
            .split('/')
            .filter(|str| !str.is_empty())
            .map(|str| str.replace("~1", "/"))
            .fold(self.val.get(&src).clone().unwrap(), |acc, val| &acc[val]);
        // FIXME_LATER: src should be resolved properly (by not guessing), it requires passing original context src to this fn
        let result = match value {
            Value::Null => {
                let guess = self
                    .val
                    .iter()
                    .find(|(_, &ref val)| {
                        let open_api: OpenApi = serde_yaml::from_value(val.clone()).unwrap();
                        open_api
                            .components
                            .schemas
                            .contains_key(path.split("/").last().unwrap())
                    })
                    .unwrap()
                    .1;
                path.split('/')
                    .filter(|str| !str.is_empty())
                    .fold(guess, |acc, val| &acc[val])
            }
            _ => value,
        }
        .clone();
        serde_yaml::from_value(result).unwrap()
    }

    pub(crate) fn src_and_path(r#ref: String) -> (Option<String>, String) {
        let split: Vec<_> = r#ref.split('#').collect();
        let src = if split[0].is_empty() {
            None
        } else {
            Some(split[0].to_string())
        };
        let path = split[1];
        (src, path.to_string())
    }
}
