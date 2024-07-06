#[cfg(test)]
mod adt {

    use crate::{Generator, Role};

    use crate::test::util::{from_open_api_test_fn, gen_test, to_open_api_test_fn};

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
        gen_test(
            Generator::Kotlin,
            Role::Server,
            "adt-from-open-api-trust.yml".to_string(),
        );
        gen_test(
            Generator::Kotlin,
            Role::Server,
            "adt-to-open-api-trust.yml".to_string(),
        );
    }

    #[test]
    fn gen_python_server_test() {
        gen_test(
            Generator::Python,
            Role::Server,
            "adt-from-open-api-trust.yml".to_string(),
        );
        gen_test(
            Generator::Python,
            Role::Server,
            "adt-to-open-api-trust.yml".to_string(),
        );
    }

    #[test]
    fn gen_scala_test() {
        gen_test(
            Generator::Scala,
            Role::Server,
            "adt-from-open-api-trust.yml".to_string(),
        );
        gen_test(
            Generator::Scala,
            Role::Server,
            "adt-to-open-api-trust.yml".to_string(),
        );
    }

    #[test]
    fn gen_typescript_test() {
        gen_test(
            Generator::TypeScript,
            Role::Server,
            "adt-from-open-api-trust.yml".to_string(),
        );
        gen_test(
            Generator::TypeScript,
            Role::Server,
            "adt-to-open-api-trust.yml".to_string(),
        );
    }
}
