#[cfg(test)]
mod mix {
    use crate::{
        lib::test::util::{from_open_api_test_fn, gen_test},
        Lang, Role,
    };

    #[test]
    fn from_open_api_test() {
        from_open_api_test_fn("mix-of-mix");
    }

    #[test]
    fn gen_kotlin_test() {
        gen_test(Lang::Kotlin, Role::Server, "mix-of-mix-trust.yml");
    }

    #[test]
    fn gen_python_client_test() {
        gen_test(Lang::Python, Role::Client, "mix-of-mix-trust.yml");
    }

    #[test]
    fn gen_python_server_test() {
        gen_test(Lang::Python, Role::Server, "mix-of-mix-trust.yml");
    }

    #[test]
    fn gen_scala_test() {
        gen_test(Lang::Scala, Role::Server, "mix-of-mix-trust.yml");
    }

    #[test]
    fn gen_typescript_test() {
        gen_test(Lang::TypeScript, Role::Server, "mix-of-mix-trust.yml");
    }
}
