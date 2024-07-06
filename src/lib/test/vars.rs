#[cfg(test)]
mod vars {
    use crate::test::util::{from_open_api_test_fn, gen_test, to_open_api_test_fn};
    use crate::{Generator, Role};

    #[test]
    fn to_open_api_test() {
        to_open_api_test_fn("vars");
    }

    #[test]
    fn from_open_api_test() {
        from_open_api_test_fn("vars");
    }

    #[test]
    fn gen_kotlin_test() {
        gen_test(
            Generator::Kotlin,
            Role::Server,
            "vars-trust.yml".to_string(),
        );
    }

    #[test]
    fn gen_python_server_test() {
        gen_test(
            Generator::Python,
            Role::Server,
            "vars-trust.yml".to_string(),
        );
    }

    #[test]
    fn gen_scala_test() {
        gen_test(Generator::Scala, Role::Server, "vars-trust.yml".to_string());
    }

    #[test]
    fn gen_typescript_test() {
        gen_test(
            Generator::TypeScript,
            Role::Server,
            "vars-trust.yml".to_string(),
        );
    }
}
