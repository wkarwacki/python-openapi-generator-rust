#[cfg(test)]
mod op_params {
    use crate::{
        test::util::{from_open_api_test_fn, gen_test, to_open_api_test_fn},
        Generator, Role,
    };

    #[test]
    fn to_open_api_test() {
        to_open_api_test_fn("op-params");
    }

    #[test]
    fn from_open_api_test() {
        from_open_api_test_fn("op-params");
    }

    #[test]
    fn gen_python_client_test() {
        gen_test(
            Generator::Python,
            Role::Client,
            "op-params-trust.yml".to_string(),
        );
    }

    #[test]
    fn gen_python_server_test() {
        gen_test(
            Generator::Python,
            Role::Server,
            "op-params-trust.yml".to_string(),
        );
    }
}