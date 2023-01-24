
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

    #[rstest]
    fn cta_client_build_url(cta_client: CTAClient) {
        let url = cta_client.build_url(String::from("www.example.com/test.aspx"));
        
        assert_eq!(String::from("www.example.com/test.aspx?key=TESTKEY&outputType=JSON"), url);
    }

    #[rstest]
    fn cta_client_mapid_build_url(cta_client: CTAClient) {
        let url = cta_client
                            .mapid(String::from("12345"))
                            .build_url(String::from("www.example.com/test.aspx"));

        assert_eq!(String::from("www.example.com/test.aspx?key=TESTKEY&mapid=12345&outputType=JSON"), url);
    }

    #[rstest]
    fn cta_client_stpid_build_url(cta_client: CTAClient) {
        let url = cta_client
                            .stpid(String::from("12345"))
                            .build_url(String::from("www.example.com/test.aspx"));

        assert_eq!(String::from("www.example.com/test.aspx?key=TESTKEY&outputType=JSON&stpid=12345"), url);
    }

    #[rstest]
    fn cta_client_both_mapid_build_url(cta_client: CTAClient) {
        let url = cta_client
                            .stpid(String::from("12345"))
                            .mapid(String::from("54321"))
                            .build_url(String::from("www.example.com/test.aspx"));

        assert_eq!(String::from("www.example.com/test.aspx?key=TESTKEY&mapid=54321&outputType=JSON"), url);
    }

    #[rstest]
    fn cta_client_both_stpid_build_url(cta_client: CTAClient) {
        let url = cta_client
                            .mapid(String::from("54321"))
                            .stpid(String::from("12345"))
                            .build_url(String::from("www.example.com/test.aspx"));

        assert_eq!(String::from("www.example.com/test.aspx?key=TESTKEY&outputType=JSON&stpid=12345"), url);
    }
}