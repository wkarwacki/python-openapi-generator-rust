use crate::lib::{
    context::Context,
    gen::python::{
        client::gen_python_http_client::GenPythonHttpClient, lang_python::LangPython,
        server::gen_python_http_server::GenPythonHttpServer,
    },
    open_api::{
        open_api::OpenApi,
        processing::{refs, refs_rec},
    },
    pkg::Pkg,
    util::{read_t, write},
};
use clap::{Parser, Subcommand, ValueEnum};
use gen::{gen::Gen, lang::DTO_NAME_TEMPLATE_NAME};
use handlebars::Handlebars;
use itertools::Itertools;
use serde::Deserialize;
use std::{
    collections::HashMap,
    fs,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
    str::FromStr,
};
use strum_macros::IntoStaticStr;
use typetag::serde;

pub mod adt {
    pub use crate::lib::adt::*;
}

pub mod carrier {
    pub use crate::lib::carrier::*;
}

pub mod context {
    pub use crate::lib::context::*;
}

pub mod def {
    pub use crate::lib::def::*;
}

pub mod desc {
    pub use crate::lib::desc::*;
}

pub mod ext {
    pub use crate::lib::ext::*;
}

pub mod open_api {
    pub use crate::lib::open_api::*;
}

pub mod pkg {
    pub use crate::lib::pkg::*;
}

pub mod op {
    pub use crate::lib::op::*;
}

pub mod r#ref {
    pub use crate::lib::r#ref::*;
}

pub mod test {
    pub use crate::lib::test::*;
}

pub mod util {
    pub use crate::lib::util::*;
}

pub mod var {
    pub use crate::lib::var::*;
}

pub mod op_param {
    pub use crate::lib::op_param::*;
}

pub mod res {
    pub use crate::lib::res::*;
}

pub mod req {
    pub use crate::lib::req::*;
}

pub mod gen {
    pub use crate::lib::gen::*;
}

pub mod meta {
    pub use crate::lib::meta::*;
}

pub mod validation {
    pub use crate::lib::validation::*;
}

pub mod lib {
    pub mod adt;
    pub mod carrier;
    pub mod context;
    pub mod def;
    pub mod desc;
    pub mod ext;
    pub mod open_api;
    pub mod pkg;

    pub mod op;
    pub mod r#ref;

    pub mod gen;
    pub mod meta;
    pub mod op_param;
    pub mod req;
    pub mod res;
    pub mod test;
    pub mod util;
    pub mod validation;
    pub mod var;
}

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Cmd,
}

#[derive(Clone, Subcommand)]
pub enum Cmd {
    FromOpenApi {
        input: PathBuf,
        output: PathBuf,
        #[clap(short, value_enum, default_value_t = Layout::Default)]
        layout: Layout,
    },
    ToOpenApi {
        input: PathBuf,
    },
    Generate {
        #[clap(value_enum)]
        lang: Lang,
        #[clap(value_enum)]
        role: Role,
        input: PathBuf,
        output: PathBuf,
        #[clap(short)]
        config: Option<PathBuf>,
        #[clap(short)]
        templates_path: Option<PathBuf>,
    },
}

#[derive(Clone, Debug, IntoStaticStr, ValueEnum)]
pub enum Role {
    Client,
    Server,
}

#[derive(Clone, PartialEq, ValueEnum)]
pub enum Layout {
    Default,
    Tag,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenCfg {
    #[serde(default)]
    type_mapping: HashMap<String, String>,
    module: Option<PathBuf>,
    dto_name: Option<String>,
}

#[derive(Clone, ValueEnum)]
pub enum Lang {
    Kotlin,
    Python,
    Scala,
    TypeScript,
}

pub fn do_main(cli: Cli) {
    match cli.cmd {
        Cmd::FromOpenApi {
            input,
            output,
            layout,
        } => {
            let pkgs = from_open_api(input.clone(), layout);

            pkgs.iter().for_each(|(src, pkg)| {
                let p = output.to_string_lossy().to_string()
                    + "/"
                    + src
                        .clone()
                        .unwrap_or(input.file_name().unwrap().to_str().unwrap().to_string())
                        .as_str();
                let path = Path::new(p.as_str());
                fs::create_dir_all(path.parent().unwrap()).unwrap();
                write(pkg, path.into());
            });
        }
        Cmd::ToOpenApi { input } => {
            let open_api = to_open_api(input);

            write(open_api, "out.yml".into())
        }
        Cmd::Generate {
            lang,
            role,
            input,
            output,
            config,
            templates_path,
        } => {
            let generator_config = config
                .map(|c| serde_yaml::from_reader::<File, GenCfg>(File::open(c).unwrap()).unwrap())
                .unwrap_or(GenCfg {
                    type_mapping: HashMap::new(),
                    module: None,
                    dto_name: None,
                });
            gen(
                input.clone(),
                lang,
                role,
                generator_config,
                templates_path,
            )
            .iter()
            .for_each(|(path, content)| {
                let full_path = output.to_string_lossy().to_string()
                    + "/"
                    + path.to_string_lossy().to_string().as_str();
                fs::create_dir_all(
                    PathBuf::from_str(full_path.as_str())
                        .unwrap()
                        .parent()
                        .unwrap(),
                )
                .unwrap();
                let mut out = std::fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(full_path)
                    .unwrap();
                out.write_all(content.as_bytes()).unwrap()
            })
        }
    }
}

fn from_open_api(input: PathBuf, layout: Layout) -> HashMap<Option<String>, Pkg> {
    let context = open_api::context::Context::of(input.clone());
    context
        .val
        .iter()
        .flat_map(|(src, value)| {
            // TIDY: hide processing behind trait
            let open_api: OpenApi = serde_yaml::from_value(value.clone()).unwrap();
            if layout == Layout::Tag {
                let tag_and_path_with_its_name_and_used_refs_within: Vec<_> = open_api
                    .paths
                    .iter()
                    .flat_map(|(name, ref_or_path)| {
                        let context = &context;
                        let path = ref_or_path.clone().unwrap(context);
                        path.operations()
                            .iter()
                            .map(move |operation| {
                                let tag = operation
                                    .tags
                                    .first()
                                    .cloned()
                                    .map(|tag| tag.to_string() + ".yml")
                                    .or_else(|| src.clone())
                                    .unwrap_or_else(|| "default.yml".to_string());

                                let operation_value =
                                    serde_yaml::to_value(operation.clone()).unwrap();

                                let refs = refs(&operation_value);
                                (tag, (name, ref_or_path, refs))
                            })
                            .collect::<Vec<_>>()
                    })
                    .collect();

                let open_api_value = serde_yaml::to_value(open_api.clone()).unwrap();
                let tag_and_pkg: Vec<_> = tag_and_path_with_its_name_and_used_refs_within
                    .iter()
                    .into_group_map_by(|(src, _)| src)
                    .iter()
                    .map(|(&src, vec)| {
                        (
                            (
                                src.clone(),
                                vec.iter()
                                    .flat_map(|(_, (_, _, refs))| refs)
                                    .collect::<Vec<_>>(),
                            ),
                            vec.iter()
                                .map(|(_, (&ref name, &ref path, _refs))| {
                                    (name.clone(), path.clone())
                                })
                                .collect::<HashMap<_, _>>(),
                        )
                    })
                    .map(|((src, refs), paths)| {
                        let open_api_with_all_refs_rec_value = refs_rec(
                            &open_api_value,
                            refs.iter()
                                .map(|r#ref| r#ref.to_string())
                                .collect::<Vec<_>>(),
                        );
                        let open_api_with_all_refs_rec: OpenApi =
                            serde_yaml::from_value(open_api_with_all_refs_rec_value.clone())
                                .unwrap();
                        let open_api_for_tag = OpenApi {
                            paths: paths,
                            components: open_api_with_all_refs_rec.components,
                        };
                        let pkg = open_api_for_tag.pkg(&context);
                        (Some(src.clone().clone()), pkg)
                    })
                    .collect();
                tag_and_pkg
            } else {
                let pkg = open_api.pkg(&context);
                vec![(src.clone(), pkg)]
            }
        })
        .collect()
}

// TODO_LATER: make it work for multiple files
fn to_open_api(input: PathBuf) -> OpenApi {
    let pkg: Pkg = read_t(input.clone());
    OpenApi::of(pkg, &Context::of(input))
}

fn gen(
    input: PathBuf,
    lang: Lang,
    role: Role,
    gen_cfg: GenCfg,
    templates_path: Option<PathBuf>,
) -> HashMap<PathBuf, String> {
    let pkg: Pkg = read_t(input.clone());
    let context = Context::of(input.clone());

    let gen = get_gen(lang, gen_cfg.clone(), input.clone(), role.clone());

    gen::gen::go(
        &pkg,
        gen,
        templates_path,
        context,
    )
}

fn get_gen(lang: Lang, gen_cfg: GenCfg, input: PathBuf, role: Role) -> Box<dyn Gen> {
    match lang {
        Lang::Kotlin => unimplemented!("not supported yet")/*Box::new(Kotlin {
            gen_cfg,
            feature: input.file_stem().unwrap().to_str().unwrap().to_string()
        })*/,
        Lang::Python => {
            let mut handlebars = Handlebars::new();
            handlebars.register_template_string(DTO_NAME_TEMPLATE_NAME, gen_cfg.clone().dto_name.unwrap_or(LangPython::dto_name_template())).unwrap();

            let lang = LangPython {
                gen_cfg,
                feature: input.file_stem().unwrap().to_str().unwrap().to_string(),
                handlebars: handlebars,
            };
            match role {
                Role::Client => Box::new(GenPythonHttpClient {
                    lang: lang
                }),
                Role::Server => Box::new(GenPythonHttpServer {
                    lang: lang
                })
            }
        }
        Lang::Scala => unimplemented!("not supported yet")/*Box::new(Scala {
            gen_cfg,
            feature: input.file_stem().unwrap().to_str().unwrap().to_string()
        })*/,
        Lang::TypeScript => unimplemented!("not supported yet")/*Box::new(TypeScript {
            gen_cfg,
            feature: input.file_stem().unwrap().to_str().unwrap().to_string()
        })*/
    }
}