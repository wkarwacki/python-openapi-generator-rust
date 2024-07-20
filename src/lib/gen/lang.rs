use crate::lib::{def::Def, desc::Desc, r#ref::Ref};
use handlebars::{Handlebars, JsonValue};
use std::path::PathBuf;

pub(crate) trait Lang {
    fn handlebars(&self) -> Handlebars;
    fn out_dir(&self) -> PathBuf;
    fn module(&self) -> String;
    fn pkg_name(&self) -> String;
    fn fmt_class(&self, class: &str, origin: &Option<String>) -> String;
    fn fmt_enum(&self, val: &str) -> String;
    fn fmt_name(&self, name: &str) -> String;
    fn fmt_opt(&self, string: &str) -> String;
    fn fmt_ref(&self, r#ref: &Ref) -> String;
    fn fmt_src(&self, src: &str) -> String;
    fn fmt_type(&self, def: &Def, name: &Option<&str>) -> String;
    fn fmt_value(&self, json_value: &JsonValue) -> String;
}

pub static DTO_NAME_TEMPLATE_NAME: &str = "dtoName";

pub(crate) fn inner(
    desc: &Desc,
    suffix: &str,
    name: &Option<&str>,
    lang: &Box<dyn Lang>,
) -> String {
    match desc {
        Desc::Def(def) => {
            let name_with_suffix = name
                .map(|n| n.to_string() + suffix)
                .unwrap_or_else(|| suffix.to_string());
            let formatted_type = lang.fmt_type(def, &Some(name_with_suffix.as_str()));
            let formatted_type_replaced =
                formatted_type.replace(&(suffix.to_string() + suffix), suffix);
            lang.fmt_opt(&formatted_type_replaced)
        }
        Desc::Ref(r#ref) => lang.fmt_ref(r#ref),
        Desc::TypeParam { .. } => unimplemented!("Type parameter not supported yet."),
    }
}
