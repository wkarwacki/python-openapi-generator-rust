#[cfg(test)]
mod mix {
    
    use std::io::{Write};

    use crate::{from_open_api, Generator, Role};
    use crate::open_api::open_api::OpenApi;
    use crate::pkg::Pkg;
    use crate::test::util::{from_open_api_test_fn, gen_test, to_open_api_test_fn};
    use crate::to_open_api;
    use crate::util::read_t;

    #[test]
    fn to_open_api_test() {
        to_open_api_test_fn("mix");
    }

    #[test]
    fn from_open_api_test() {
        from_open_api_test_fn("mix");
    }

    #[test]
    fn gen_kotlin_test() {
        gen_test(Generator::Kotlin, Role::Server, "mix-trust.yml".to_string());
    }

    #[test]
    fn gen_python_server_test() {
        gen_test(Generator::Python, Role::Server, "mix-trust.yml".to_string());
    }

    #[test]
    fn gen_scala_test() {
        gen_test(Generator::Scala, Role::Server, "mix-trust.yml".to_string());
    }

    #[test]
    fn gen_typescript_test() {
        gen_test(Generator::TypeScript, Role::Server, "mix-trust.yml".to_string());
    }
}
