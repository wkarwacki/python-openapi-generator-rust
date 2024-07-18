use serde_yaml::{Mapping, Value};
use std::collections::HashMap;

pub(crate) fn refs(value: &Value) -> Vec<String> {
    match value {
        Value::Mapping(map) => map
            .iter()
            .flat_map(|(key, val)| {
                if key.as_str() == Some("$ref") {
                    if let Value::String(ref_val) = val {
                        vec![ref_val.clone()]
                    } else {
                        Vec::new()
                    }
                } else {
                    refs(val)
                }
            })
            .collect(),
        Value::Sequence(seq) => seq.iter().flat_map(refs).collect(),
        _ => Vec::new(),
    }
}

pub(crate) fn refs_rec(open_api: &Value, refs: Vec<String>) -> Value {
    let mut visited = HashMap::new();
    get_refs_rec(open_api, &refs, &mut visited)
}

fn resolve<'a>(value: &'a Value, r#ref: &str) -> Option<&'a Value> {
    let mut v = value;
    for part in r#ref.replace("#/", "").split('/') {
        v = match v {
            Value::Mapping(map) => map.get(&Value::String(part.to_string())).unwrap(),
            _ => return None,
        };
    }
    Some(v)
}

fn get_refs_rec(
    open_api: &Value,
    refs_param: &Vec<String>,
    visited: &mut HashMap<String, Value>,
) -> Value {
    let schemas = Value::Mapping(Mapping::new());
    let mut components_map = Mapping::new();
    components_map.insert(Value::String("schemas".to_string()), schemas.clone());
    let mut res_map = Mapping::new();
    res_map.insert(
        Value::String("components".to_string()),
        Value::Mapping(components_map.clone()),
    );
    let mut res = Value::Mapping(res_map);

    for r#ref in refs_param {
        if let Some(value) = resolve(open_api, r#ref) {
            let (_, name) = r#ref.rsplit_once('/').unwrap();
            add_schema_to_open_api(value, name, &mut res);

            let refs = refs(value);
            let inner = get_refs_rec(open_api, &refs, visited);
            merge_schemas(&inner, &mut res);
        }
    }

    res
}

fn merge_schemas(from: &Value, to: &mut Value) {
    if let Value::Mapping(from_map) = from {
        if let Some(Value::Mapping(components)) = from_map.get(&Value::String("components".into()))
        {
            if let Some(Value::Mapping(schemas)) = components.get(&Value::String("schemas".into()))
            {
                for (name, from_schema) in schemas {
                    add_schema_to_open_api(from_schema, name.as_str().unwrap(), to);
                }
            }
        }
    }
}

fn add_schema_to_open_api(schema: &Value, name: &str, open_api: &mut Value) {
    if let Value::Mapping(open_api_map) = open_api {
        if let Some(Value::Mapping(components)) =
            open_api_map.get_mut(&Value::String("components".into()))
        {
            if let Some(Value::Mapping(schemas)) =
                components.get_mut(&Value::String("schemas".into()))
            {
                schemas.insert(Value::String(name.into()), schema.clone());
            }
        }
    }
}
