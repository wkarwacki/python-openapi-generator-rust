use crate::lib::{def::Def, desc::Desc, r#ref::Ref};
use handlebars::{Handlebars, JsonValue};
use std::path::PathBuf;

pub trait Lang {
    fn handlebars(&self) -> Handlebars;
    fn out_dir(&self) -> PathBuf;
    fn module(&self) -> String;
    fn pkg_name(&self) -> String;
    fn fmt_class(&self, class: String, origin: Option<String>) -> String;
    fn fmt_enum(&self, val: String) -> String;
    fn fmt_name(&self, name: String) -> String;
    fn fmt_opt(&self, string: String) -> String;
    fn fmt_ref(&self, r#ref: Ref) -> String;
    fn fmt_src(&self, src: String) -> String;
    fn fmt_type(&self, def: Def, name: Option<String>) -> String;
    fn fmt_value(&self, json_value: JsonValue) -> String;
}

pub static DTO_NAME_TEMPLATE_NAME: &str = "dtoName";

pub fn inner(desc: Desc, suffix: &str, name: Option<String>, lang: Box<dyn Lang>) -> String {
    match desc {
        Desc::Def(def) => lang.fmt_opt(
            lang.fmt_type(
                def.clone(),
                name.clone()
                    .map(|n| n + suffix)
                    .or(Some(suffix.to_string())),
            )
            .replace((suffix.to_string() + suffix).as_str(), suffix),
        ),
        Desc::Ref(r#ref) => lang.fmt_ref(r#ref),
        Desc::TypeParam { .. } => unimplemented!("Type parameter not supported yet."),
    }
}
