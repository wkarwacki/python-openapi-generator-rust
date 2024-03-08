#[cfg(test)]
mod adt {

    
    use std::io::{Write};

    use crate::{from_open_api, Generator, Role};
    use crate::open_api::open_api::OpenApi;
    use crate::pkg::Pkg;
    use crate::test::util::{from_open_api_test_fn, gen_test, to_open_api_test_fn};
    use crate::to_open_api;
    use crate::util::read_t;

    #[test]
    fn to_open_api_test() {
        to_open_api_test_fn("adt-to-open-api");
    }

    #[test]
    fn from_open_api_test() {
        from_open_api_test_fn("adt-from-open-api");
    }

    #[test]
    fn gen_kotlin_test() {
        gen_test(Generator::Kotlin, Role::Server, "adt-from-open-api-trust.yml".to_string());
        gen_test(Generator::Kotlin, Role::Server, "adt-to-open-api-trust.yml".to_string());
    }

    #[test]
    fn gen_python_server_test() {
        gen_test(Generator::Python, Role::Server, "adt-from-open-api-trust.yml".to_string());
        gen_test(Generator::Python, Role::Server, "adt-to-open-api-trust.yml".to_string());
    }

    #[test]
    fn gen_scala_test() {
        gen_test(Generator::Scala, Role::Server, "adt-from-open-api-trust.yml".to_string());
        gen_test(Generator::Scala, Role::Server, "adt-to-open-api-trust.yml".to_string());
    }

    #[test]
    fn gen_typescript_test() {
        gen_test(Generator::TypeScript, Role::Server, "adt-from-open-api-trust.yml".to_string());
        gen_test(Generator::TypeScript, Role::Server, "adt-to-open-api-trust.yml".to_string());
    }
}
