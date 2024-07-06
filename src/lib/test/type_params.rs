#[cfg(test)]
mod type_params {
    use crate::test::util::{gen_test, to_open_api_test_fn};
    use crate::{Generator, Role};

    #[test]
    fn to_open_api_test() {
        to_open_api_test_fn("type-params");
    }

    #[test]
    fn gen_kotlin_test() {
        gen_test(
            Generator::Kotlin,
            Role::Server,
            "type-params-trust.yml".to_string(),
        );
    }

    #[test]
    fn gen_python_server_test() {
        gen_test(
            Generator::Python,
            Role::Server,
            "type-params-trust.yml".to_string(),
        );
    }

    #[test]
    fn gen_scala_test() {
        gen_test(
            Generator::Scala,
            Role::Server,
            "type-params-trust.yml".to_string(),
        );
    }

    #[test]
    fn gen_typescript_test() {
        gen_test(
            Generator::TypeScript,
            Role::Server,
            "type-params-trust.yml".to_string(),
        );
    }
}
