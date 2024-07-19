#[cfg(test)]
mod adt {
    use crate::{
        lib::test::util::{from_open_api_test_fn, gen_test, to_open_api_test_fn},
        Lang, Role,
    };

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
        gen_test(Lang::Kotlin, Role::Server, "adt-from-open-api-trust.yml");
        gen_test(Lang::Kotlin, Role::Server, "adt-to-open-api-trust.yml");
    }

    #[test]
    fn gen_python_client_test() {
        gen_test(Lang::Python, Role::Client, "adt-from-open-api-trust.yml");
        gen_test(Lang::Python, Role::Client, "adt-to-open-api-trust.yml");
    }

    #[test]
    fn gen_python_server_test() {
        gen_test(Lang::Python, Role::Server, "adt-from-open-api-trust.yml");
        gen_test(Lang::Python, Role::Server, "adt-to-open-api-trust.yml");
    }

    #[test]
    fn gen_scala_test() {
        gen_test(Lang::Scala, Role::Server, "adt-from-open-api-trust.yml");
        gen_test(Lang::Scala, Role::Server, "adt-to-open-api-trust.yml");
    }

    #[test]
    fn gen_typescript_test() {
        gen_test(
            Lang::TypeScript,
            Role::Server,
            "adt-from-open-api-trust.yml",
        );
        gen_test(Lang::TypeScript, Role::Server, "adt-to-open-api-trust.yml");
    }
}
