#[cfg(test)]
mod op_params {

    use crate::test::util::{from_open_api_test_fn, to_open_api_test_fn};

    #[test]
    fn to_open_api_test() {
        to_open_api_test_fn("op-params");
    }

    #[test]
    fn from_open_api_test() {
        from_open_api_test_fn("op-params");
    }
}
