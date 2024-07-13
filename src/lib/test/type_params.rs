#[cfg(test)]
mod type_params {
    use crate::{
        test::util::{gen_test, to_open_api_test_fn},
        Lang, Role,
    };

    #[test]
    fn to_open_api_test() {
        to_open_api_test_fn("type-params");
    }

    #[test]
    fn gen_kotlin_test() {
        gen_test(
            Lang::Kotlin,
            Role::Server,
            "type-params-trust.yml".to_string(),
        );
    }

    #[test]
    fn gen_python_client_test() {
        gen_test(
            Lang::Python,
            Role::Client,
            "type-params-trust.yml".to_string(),
        );
    }

    #[test]
    fn gen_python_server_test() {
        gen_test(
            Lang::Python,
            Role::Server,
            "type-params-trust.yml".to_string(),
        );
    }

    #[test]
    fn gen_scala_test() {
        gen_test(
            Lang::Scala,
            Role::Server,
            "type-params-trust.yml".to_string(),
        );
    }

    #[test]
    fn gen_typescript_test() {
        gen_test(
            Lang::TypeScript,
            Role::Server,
            "type-params-trust.yml".to_string(),
        );
    }
}
