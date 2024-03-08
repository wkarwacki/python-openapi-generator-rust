#[cfg(test)]
mod mix {
    use crate::{Generator, Role};
    use crate::test::util::{from_open_api_test_fn, gen_test};

    #[test]
    fn from_open_api_test() {
        from_open_api_test_fn("mix-of-mix");
    }

    #[test]
    fn gen_kotlin_test() {
        gen_test(Generator::Kotlin, Role::Server, "mix-of-mix-trust.yml".to_string());
    }

    #[test]
    fn gen_python_server_test() {
        gen_test(Generator::Python, Role::Server, "mix-of-mix-trust.yml".to_string());
    }

    #[test]
    fn gen_scala_test() {
        gen_test(Generator::Scala, Role::Server, "mix-of-mix-trust.yml".to_string());
    }

    #[test]
    fn gen_typescript_test() {
        gen_test(Generator::TypeScript, Role::Server, "mix-of-mix-trust.yml".to_string());
    }
}
