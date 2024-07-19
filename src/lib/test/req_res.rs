#[cfg(test)]
mod req_res {
    use crate::{
        lib::test::util::{from_open_api_test_fn, gen_test, to_open_api_test_fn},
        Lang, Role,
    };

    #[test]
    fn to_open_api_test() {
        to_open_api_test_fn("req-res");
    }

    #[test]
    fn from_open_api_test() {
        from_open_api_test_fn("req-res");
    }

    #[test]
    fn gen_python_client_test() {
        gen_test(Lang::Python, Role::Client, "req-res-trust.yml");
    }

    #[test]
    fn gen_python_server_test() {
        gen_test(Lang::Python, Role::Server, "req-res-trust.yml");
    }
}
