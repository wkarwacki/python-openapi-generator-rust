#[cfg(test)]
mod req_res {

    use crate::test::util::{from_open_api_test_fn, to_open_api_test_fn};

    #[test]
    fn to_open_api_test() {
        to_open_api_test_fn("req-res");
    }

    #[test]
    fn from_open_api_test() {
        from_open_api_test_fn("req-res");
    }
}
