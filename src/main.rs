use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{Write};
use std::path::{Path, PathBuf};
use std::str::FromStr;

use clap::{Parser, Subcommand, ValueEnum};
use handlebars::Handlebars;
use serde::Deserialize;
use strum_macros::IntoStaticStr;
use typetag::serde;

use gen::gen::Gen;
use gen::lang::DTO_NAME_TEMPLATE_NAME;

use crate::context::Context;
use crate::gen::python::client::gen_python_http_client::GenPythonHttpClient;
use crate::gen::python::lang_python::LangPython;
use crate::gen::python::server::gen_python_http_server::GenPythonHttpServer;
use crate::open_api::open_api::OpenApi;
use crate::pkg::Pkg;
use crate::util::{read_t, write};

mod adt;
mod carrier;
mod context;
mod def;
mod desc;
mod ext;
mod open_api;
mod pkg;

mod op;
mod r#ref;

mod test;
mod util;
mod var;
mod op_param;
mod res;
mod req;
mod gen;
mod meta;
mod validation;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Clone, Subcommand)]
enum Cmd {
    FromOpenApi {
        input: PathBuf,
        output: PathBuf,
    },
    ToOpenApi {
        input: PathBuf,
    },
    Generate {
        #[clap(value_enum)]
        generator: Generator,
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

#[derive(Clone, ValueEnum)]
enum Generator {
    Kotlin,
    Python,
    Scala,
    TypeScript,
}

#[derive(Clone, Debug, IntoStaticStr, ValueEnum)]
pub enum Role {
    Client,
    Server,
}

impl Generator {
    fn gen(&self, gen_cfg: GenCfg, input: PathBuf, role: Role) -> Box<dyn Gen> {
        match self {
            Generator::Kotlin => unimplemented!("not supported yet")/*Box::new(Kotlin {
                gen_cfg,
                feature: input.file_stem().unwrap().to_str().unwrap().to_string()
            })*/,
            Generator::Python => {
                let mut handlebars = Handlebars::new();
                handlebars.register_template_string(DTO_NAME_TEMPLATE_NAME, gen_cfg.clone().dto_name.unwrap_or(LangPython::dto_name_template())).unwrap();

                let lang = LangPython {
                    gen_cfg,
                    feature: input.file_stem().unwrap().to_str().unwrap().to_string(),
                    handlebars: handlebars
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
            Generator::Scala => unimplemented!("not supported yet")/*Box::new(Scala {
                gen_cfg,
                feature: input.file_stem().unwrap().to_str().unwrap().to_string()
            })*/,
            Generator::TypeScript => unimplemented!("not supported yet")/*Box::new(TypeScript {
                gen_cfg,
                feature: input.file_stem().unwrap().to_str().unwrap().to_string()
            })*/
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GenCfg {
    #[serde(default)]
    type_mapping: HashMap<String, String>,
    subdir: Option<PathBuf>,
    dto_name: Option<String>
}

fn main() {
    let cli = Cli::parse();
    do_main(cli);
}

fn do_main(cli: Cli) {
    match cli.cmd {
        Cmd::FromOpenApi { input, output } => {
            let pkgs = from_open_api(input.clone());

            pkgs.iter().for_each(|(src, pkg)| {
                let p = output.to_string_lossy().to_string() + "/" + src.clone().unwrap_or(input.file_name().unwrap().to_str().unwrap().to_string()).as_str();
                let path = Path::new(p.as_str());
                fs::create_dir_all(path.parent().unwrap());
                write(pkg, path.into());
            });
        }
        Cmd::ToOpenApi { input } => {
            let open_api = to_open_api(input);

            write(open_api, "out.yml".into())
        }
        Cmd::Generate { generator, role, input, output, config, templates_path } => {
            let generator_config = config.map(|c| serde_yaml::from_reader::<File, GenCfg>(File::open(c).unwrap()).unwrap()).unwrap_or(GenCfg { type_mapping: HashMap::new(), subdir: None, dto_name: None });
            gen(input.clone(), generator, role, generator_config, templates_path).iter().for_each(|(path, content)| {
                let full_path = output.to_string_lossy().to_string() + "/" + path.to_string_lossy().to_string().as_str();
                fs::create_dir_all(PathBuf::from_str(full_path.as_str()).unwrap().parent().unwrap()).unwrap();
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

fn from_open_api(input: PathBuf) -> HashMap<Option<String>, Pkg> {
    let context = open_api::context::Context::of(input.clone());
    context.val.iter().map(|(src, value)| {
        let open_api: OpenApi = serde_yaml::from_value(value.clone()).unwrap();
        let pkg = open_api.pkg(&context);

        (src.clone(), pkg)
    }).collect()
}

// TODO_LATER: make it work for multiple files
fn to_open_api(input: PathBuf) -> OpenApi {
    let pkg: Pkg = read_t(input.clone());
    OpenApi::of(pkg, &Context::of(input))
}

fn gen(input: PathBuf, generator: Generator, role: Role, gen_cfg: GenCfg, templates_path: Option<PathBuf>) -> HashMap<PathBuf, String> {
    let pkg: Pkg = read_t(input.clone());
    let context = Context::of(input.clone());

    gen::gen::go(&pkg, generator.gen(gen_cfg, input, role), templates_path, context)
}
