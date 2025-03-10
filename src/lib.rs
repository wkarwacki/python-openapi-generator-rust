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
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs,
    fs::{create_dir_all, metadata, read_dir, File},
    io::Write,
    path::{Path, PathBuf},
    str::FromStr,
};
use strum_macros::IntoStaticStr;
use typetag::serde;

pub mod adt {}

pub mod carrier {}

pub mod context {}

pub mod def {}

pub mod desc {}

pub mod ext {}

pub mod open_api {
    pub(crate) use crate::lib::open_api::*;
}

pub mod pkg {
    pub(crate) use crate::lib::pkg::*;
}

pub mod op {}

pub mod r#ref {}

pub mod test {}

pub mod var {}

pub mod op_param {}

pub mod res {}

pub mod req {}

pub mod gen {
    pub(crate) use crate::lib::gen::*;
}

pub mod meta {}

pub mod validation {}

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

#[derive(Parser)]
#[clap(version)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Cmd,
}

#[derive(Clone, Subcommand)]
pub enum Cmd {
    /// Convert an OpenAPI specification to a Trust specification. Integrate this into your build process to utilize Trust's code generators.
    FromOpenApi {
        /// Path to the OpenAPI specification file.
        input: PathBuf,
        /// Directory where the output Trust specification will be saved.
        output: PathBuf,
        /// Specify the structure of the converted Trust specification.
        #[clap(short, value_enum, default_value_t = Layout::Default)]
        layout: Layout,
    },
    /// Convert a Trust specification back to an OpenAPI specification, useful when a Trust code generator is not available for your target language.
    ToOpenApi {
        /// Path to the Trust specification file.
        input: PathBuf,

        /// Directory where the output OpenApi specification will be saved.
        output: PathBuf,
    },
    /// Generate code based on a Trust specification.
    Generate {
        /// Select the target programming language for the generated code.
        #[clap(value_enum)]
        lang: Lang,
        /// Specify whether to generate server or client code.
        #[clap(value_enum)]
        role: Role,
        /// Path to the Trust specification file.
        input: PathBuf,
        /// Directory where the generated code will be saved.
        output: PathBuf,
        #[clap(short)]
        /// Optional path to a generator configuration file. Refer to the Trust documentation for details.
        config: Option<PathBuf>,
        /// Optional path to a custom templates directory. For instance, you can override any template found at https://github.com/wkarwacki/python-openapi-generator-rust/tree/master/src/lib/gen/python/server/templates, however this can be configured for all languages and roles.
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
    /// Each file in the Trust specification maps directly to a single file in the output.
    Default,
    /// Organize output based on OpenAPI tags, where each tag generates a separate file with all related references included.
    Tag,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GenCfg {
    #[serde(default)]
    type_mapping: HashMap<String, String>,
    module: Option<PathBuf>,
    dto_name: Option<String>,
    #[serde(default)]
    auto_implement: bool,
}

#[derive(Clone, ValueEnum, Debug)]
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
            let pkgs = from_open_api(&input, &layout);

            pkgs.iter().for_each(|(src, pkg)| {
                let p = output.to_string_lossy().to_string()
                    + "/"
                    + src
                        .clone()
                        .unwrap_or(input.file_name().unwrap().to_str().unwrap().to_string())
                        .as_str();
                let path = Path::new(p.as_str());
                fs::create_dir_all(path.parent().unwrap()).unwrap();
                write(pkg, &path.into());
            });
        }
        Cmd::ToOpenApi { input, output } => {
            let md = metadata(&input).map_err(|_| {
                create_dir_all(&input).unwrap();
                metadata(&input)
            });
            let dir = md.unwrap().is_dir();
            if dir {
                let read_dir = read_dir(input).unwrap();
                for entry in read_dir {
                    let entry = entry.unwrap();
                    let path = entry.path();
                    if path.is_file() {
                        to_open_api_write(&path, &output)
                    }
                }
            } else {
                to_open_api_write(&input, &output)
            }
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
                .unwrap_or(Default::default());

            println!("Running generator for lang: {:?}, role: {:?}, input: {:?}, output: {:?}, templates_path: {:?}", lang, role, input, output, templates_path);
            println!(
                "Generator config:\n{}",
                serde_json::to_string(&generator_config).unwrap()
            );

            let md = metadata(&input).map_err(|_| {
                create_dir_all(&input).unwrap();
                metadata(&input)
            });
            let dir = md.unwrap().is_dir();
            if dir {
                let read_dir = read_dir(input).unwrap();
                for entry in read_dir {
                    let entry = entry.unwrap();
                    let path = entry.path();
                    if path.is_file() {
                        gen_write(
                            &path,
                            &lang,
                            &role,
                            &generator_config,
                            &templates_path,
                            &output,
                        )
                    }
                }
            } else {
                gen_write(
                    &input,
                    &lang,
                    &role,
                    &generator_config,
                    &templates_path,
                    &output,
                )
            }
        }
    }
}

fn from_open_api(input: &PathBuf, layout: &Layout) -> HashMap<Option<String>, Pkg> {
    let context = open_api::context::Context::of(input);
    context
        .val
        .iter()
        .flat_map(|(src, value)| {
            // TIDY: hide processing behind trait
            let open_api: OpenApi = serde_yaml::from_value(value.clone()).unwrap();
            if layout == &Layout::Tag {
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
                                    .map(|tag| tag + ".yml")
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
                            paths,
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

fn to_open_api_write(input: &PathBuf, output: &PathBuf) {
    let open_api = to_open_api(input);
    write(
        open_api,
        &(output.as_os_str().to_str().unwrap().to_string()
            + "/"
            + input.file_name().unwrap().to_str().unwrap())
        .into(),
    )
}

fn to_open_api(input: &PathBuf) -> OpenApi {
    let pkg: Pkg = read_t(input);
    let tag = input.file_stem().unwrap().to_str().unwrap();
    let context = &Context::of(input);

    let open_api = OpenApi::of(pkg, tag, context);

    open_api
}

fn gen_write(
    input: &PathBuf,
    lang: &Lang,
    role: &Role,
    gen_cfg: &GenCfg,
    templates_path: &Option<PathBuf>,
    output: &PathBuf,
) {
    gen(&input, &lang, &role, &gen_cfg, &templates_path)
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
            let mut out = fs::OpenOptions::new()
                .write(true)
                .create(true)
                .open(full_path)
                .unwrap();
            out.write_all(content.as_bytes()).unwrap()
        })
}

fn gen(
    input: &PathBuf,
    lang: &Lang,
    role: &Role,
    gen_cfg: &GenCfg,
    templates_path: &Option<PathBuf>,
) -> HashMap<PathBuf, String> {
    let pkg: Pkg = read_t(input);
    let context = Context::of(input);

    let gen = get_gen(lang, gen_cfg, input, role);

    gen::gen::go(&pkg, &gen, templates_path, &gen_cfg.type_mapping, &context)
}

fn get_gen(lang: &Lang, gen_cfg: &GenCfg, input: &PathBuf, role: &Role) -> Box<dyn Gen> {
    match lang {
        Lang::Kotlin => unimplemented!("not supported yet")/*Box::new(Kotlin {
            gen_cfg,
            feature: input.file_stem().unwrap().to_str().unwrap().to_string()
        })*/,
        Lang::Python => {
            let mut handlebars = Handlebars::new();
            handlebars.register_template_string(DTO_NAME_TEMPLATE_NAME, gen_cfg.clone().dto_name.unwrap_or(LangPython::dto_name_template())).unwrap();

            let lang = LangPython {
                gen_cfg: gen_cfg.clone(),
                feature: input.file_stem().unwrap().to_str().unwrap().to_string(),
                handlebars,
            };
            match role {
                Role::Client => Box::new(GenPythonHttpClient {
                    lang
                }),
                Role::Server => Box::new(GenPythonHttpServer {
                    lang
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
