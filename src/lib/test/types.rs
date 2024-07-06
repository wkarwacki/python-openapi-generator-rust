#[cfg(test)]
mod types {
    use crate::{
        test::util::{from_open_api_test_fn, gen_test, to_open_api_test_fn, trust_only_test_fn},
        Generator, Role,
    };

    #[test]
    fn to_open_api_test() {
        to_open_api_test_fn("types");
    }

    #[test]
    fn from_open_api_test() {
        from_open_api_test_fn("types");
    }

    #[test]
    fn trust_only_test() {
        trust_only_test_fn("types");
    }

    #[test]
    fn gen_kotlin_test() {
        gen_test(
            Generator::Kotlin,
            Role::Server,
            "types-trust.yml".to_string(),
        );
    }

    #[test]
    fn gen_python_server_test() {
        gen_test(
            Generator::Python,
            Role::Server,
            "types-trust.yml".to_string(),
        );
    }

    #[test]
    fn gen_scala_test() {
        gen_test(
            Generator::Scala,
            Role::Server,
            "types-trust.yml".to_string(),
        );
    }

    #[test]
    fn gen_typescript_test() {
        gen_test(
            Generator::TypeScript,
            Role::Server,
            "types-trust.yml".to_string(),
        );
    }
}
