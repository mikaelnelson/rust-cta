
mod param_builder_tests {
    use std::collections::{HashMap};
    use crate::client::{ParamBuilder};
    use rstest::*;

    #[fixture]
    fn param_builder() -> ParamBuilder {
        return ParamBuilder::new(String::from("TESTKEY"));
    }

    #[rstest]
    fn param_builder_build_no_params(param_builder: ParamBuilder) {
        let input_params: HashMap<String, String> = HashMap::new();

        let expected_params = HashMap::from([
            (String::from("key"), String::from("TESTKEY")),
            (String::from("outputType"), String::from("JSON"))
        ]);
        
        assert_eq!( expected_params, param_builder.build(input_params));
    }

    #[rstest]
    fn param_builder_build_params(param_builder: ParamBuilder) {
        let input_params = HashMap::from([
            (String::from("mapid"), String::from("12345"))
        ]);

        let expected_params = HashMap::from([
            (String::from("key"), String::from("TESTKEY")),
            (String::from("outputType"), String::from("JSON")),
            (String::from("mapid"), String::from("12345"))
        ]);

        assert_eq!( expected_params, param_builder.build(input_params));
    }
}

mod cta_client_tests {
    use crate::client::{CTAClient};
    use rstest::*;

    #[fixture]
    fn cta_client() -> CTAClient {
        return CTAClient::new(Some(String::from("TESTKEY"))).unwrap();

    }

    #[rstest]
    fn cta_client_base_url(cta_client: CTAClient) {
        let url = cta_client.base_url();

        assert_eq!(String::from("http://lapi.transitchicago.com/api/1.0"), url);
    }
}