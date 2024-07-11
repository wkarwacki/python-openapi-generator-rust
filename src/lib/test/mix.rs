#[cfg(test)]
mod mix {

    use crate::{
        test::util::{from_open_api_test_fn, gen_test, to_open_api_test_fn},
        Generator, Role,
    };

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
    fn gen_python_client_test() {
        gen_test(Generator::Python, Role::Client, "mix-trust.yml".to_string());
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
        gen_test(
            Generator::TypeScript,
            Role::Server,
            "mix-trust.yml".to_string(),
        );
    }
}
