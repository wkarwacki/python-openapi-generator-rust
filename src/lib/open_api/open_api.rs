use crate::lib::{
    context::Context,
    def,
    open_api::{
        components::Components, context::Context as OpenApiContext, path::Path, ref_or::RefOr,
        schema::schemas_path,
    },
    pkg::Pkg,
    r#ref::Ref,
};
use def::DEFS;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use typetag::serde;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(crate) struct OpenApi {
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub paths: HashMap<String, RefOr<Path>>,
    #[serde(skip_serializing_if = "Components::is_empty")]
    #[serde(default)]
    pub components: Components,
}

impl OpenApi {
    pub(crate) fn of(pkg: Pkg, context: &Context) -> OpenApi {
        OpenApi {
            paths: pkg
                .ops
                .iter()
                .map(|(id, ops)| {
                    let path_prefix = if pkg.use_namespace {
                        Some(context._base.file_stem().unwrap().to_str().unwrap())
                    } else {
                        None
                    };
                    (
                        Self::path_id(id, path_prefix),
                        RefOr::Item(Path::of(ops, context)),
                    )
                })
                .collect(),
            components: Components::of(pkg.defs, context),
        }
    }

    fn path_id(id: &String, path_prefix: Option<&str>) -> String {
        match path_prefix {
            Some(prefix) => format!("/{}{}", prefix, id),
            None => id.clone(),
        }
    }

    pub(crate) fn pkg(&self, context: &OpenApiContext) -> Pkg {
        let mut with_mapped_all_of: Vec<_> = self
            .components
            .schemas
            .iter()
            .filter(|(_, schema)| !schema.all_of.is_empty())
            .map(|(name, schema)| {
                let name_clone = name.clone();
                (name_clone, schema.with_mapped_all_of())
            })
            .collect();

        let mut other: Vec<_> = self
            .components
            .schemas
            .iter()
            .filter(|(_, schema)| schema.all_of.is_empty())
            .map(|(name, schema)| (name.clone(), schema.clone()))
            .collect();
        other.append(&mut with_mapped_all_of);

        Pkg {
            ops: self
                .paths
                .iter()
                .flat_map(|(id, path)| path.clone().item().map(|p| (id, p)))
                .map(|(id, path)| (id.clone(), path.ops(context)))
                .filter(|(_, ops)| !ops.is_empty())
                .collect(),
            defs: other
                .iter()
                .map(|(name, schema)| (name.clone().clone(), schema.def(name.clone(), context)))
                .collect(),
            use_namespace: Default::default(),
        }
    }

    pub(crate) fn trust_ref(r#ref: String) -> Ref {
        let (src, path) = OpenApiContext::src_and_path(r#ref);
        let defs = DEFS;
        Ref {
            src,
            path: path.replace((schemas_path() + "/").as_str(), format!("{defs}.").as_str()),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Discriminator {
    pub(crate) property_name: String,
    pub(crate) mapping: HashMap<String, String>,
}
