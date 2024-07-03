use crate::{Cli, Cmd, do_main, from_open_api, Generator, Layout, Role, to_open_api};
use crate::open_api::open_api::OpenApi;
use crate::pkg::Pkg;
use crate::util::{read_t, write};

pub fn from_open_api_test_fn(name: &str) {
    let pkgs = from_open_api(("src/test/".to_string() + name + "-open-api.yml").into(), Layout::Default);
    let pkg = pkgs.get(&None).unwrap();

    write(pkg, ("test_debug/".to_string() + name + "-from-open-api.yml").into());

    let expected: Pkg = read_t(("src/test/".to_string() + name + "-trust.yml").into());

    assert_eq!(pkg, &expected);
}

pub fn to_open_api_test_fn(name: &str) {
    let open_api = to_open_api(("src/test/".to_string() + name + "-trust.yml").into());

    write(open_api.clone(), ("test_debug/".to_string() + name + "-to-open-api.yml").into());

    let expected: OpenApi = read_t(("src/test/".to_string() + name + "-open-api.yml").into());

    assert_eq!(open_api, expected);
}

pub fn trust_only_test_fn(name: &str) {
    let pkg: Pkg = read_t(("src/test/".to_string() + name + "-trust-only.yml").into());

    write(&pkg, ("test_debug/".to_string() + name + "-trust-only.yml").into());

    let expected: Pkg = read_t(("src/test/".to_string() + name + "-trust-only.yml").into());

    assert_eq!(pkg, expected);
}


pub fn gen_test(generator: Generator, role: Role, input: String) {
    match generator {
        Generator::Kotlin => { },
        Generator::Python => do_gen_test(generator, role, input),
        Generator::Scala => { },
        Generator::TypeScript => { },
    };
}

fn do_gen_test(generator: Generator, role: Role, input: String) {
    let output = "test_debug/gen/".to_string() + match generator {
        Generator::Kotlin => "kotlin/src/main/kotlin",
        Generator::Python => "python/src/trust",
        Generator::Scala => "scala/src/main/scala",
        Generator::TypeScript => "typescript/src/trust",
    };
    do_main(Cli{
        cmd: Cmd::Generate {
            generator,
            role,
            input: format!("src/test/{input}").into(),
            output: output.into(),
            config: None,
            templates_path: None
        }
    });
}
