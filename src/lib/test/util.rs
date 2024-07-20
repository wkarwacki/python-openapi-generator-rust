use crate::{
    do_main, from_open_api,
    lib::util::{read_t, write},
    open_api::open_api::OpenApi,
    pkg::Pkg,
    to_open_api, Cli, Cmd, Lang, Layout, Role,
};
use convert_case::{Case, Casing};

#[allow(dead_code)]
pub(crate) fn from_open_api_test_fn(name: &str) {
    let pkgs = from_open_api(
        &("src/lib/test/".to_string() + name + "-open-api.yml").into(),
        &Layout::Default,
    );
    let pkg = pkgs.get(&None).unwrap();

    write(
        pkg,
        &("test/default/spec/".to_string() + name + "-from-open-api.yml").into(),
    );

    let expected: Pkg = read_t(&("src/lib/test/".to_string() + name + "-trust.yml").into());

    assert_eq!(pkg, &expected);
}

#[allow(dead_code)]
pub(crate) fn to_open_api_test_fn(name: &str) {
    let open_api = to_open_api(&("src/lib/test/".to_string() + name + "-trust.yml").into());

    write(
        open_api.clone(),
        &("test/default/spec/".to_string() + name + "-to-open-api.yml").into(),
    );

    let expected: OpenApi = read_t(&("src/lib/test/".to_string() + name + "-open-api.yml").into());

    assert_eq!(open_api, expected);
}

#[allow(dead_code)]
pub(crate) fn trust_only_test_fn(name: &str) {
    let pkg: Pkg = read_t(&("src/lib/test/".to_string() + name + "-trust-only.yml").into());

    write(
        &pkg,
        &("test/default/spec/".to_string() + name + "-trust-only.yml").into(),
    );

    let expected: Pkg = read_t(&("src/lib/test/".to_string() + name + "-trust-only.yml").into());

    assert_eq!(pkg, expected);
}

#[allow(dead_code)]
pub(crate) fn gen_test(generator: Lang, role: Role, input: &str) {
    match generator {
        Lang::Kotlin => {}
        Lang::Python => do_gen_test(generator, role, input),
        Lang::Scala => {}
        Lang::TypeScript => {}
    };
}

#[allow(dead_code)]
pub(crate) fn do_gen_test(lang: Lang, role: Role, input: &str) {
    let role_str: &str = role.clone().into();
    let output = "test/default/gens/".to_string()
        + match lang {
            Lang::Kotlin => "kotlin/src/main/kotlin".to_string(),
            Lang::Python => "python/".to_string() + role_str.to_case(Case::Lower).as_str() + "/src",
            Lang::Scala => "scala/src/main/scala".to_string(),
            Lang::TypeScript => "typescript/src/trust".to_string(),
        }
        .as_str();
    do_main(Cli {
        cmd: Cmd::Generate {
            lang,
            role,
            input: format!("src/lib/test/{input}").into(),
            output: output.into(),
            config: None,
            templates_path: None,
        },
    });
}
