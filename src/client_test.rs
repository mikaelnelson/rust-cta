mod cta_client_tests {
    use crate::client::{CTAClient, CTAClientError};
    use rstest::*;

    #[fixture]
    fn cta_client() -> CTAClient {
        CTAClient::new(Some(String::from("TESTKEY"))).unwrap()
    }

    #[rstest]
    fn cta_client_new_fail() {
        let cta_client = CTAClient::new(None);

        assert!(cta_client.is_err());
        assert_eq!(cta_client.unwrap_err(), CTAClientError::MissingCTAKey);
    }

    #[rstest]
    fn cta_client_base_url(cta_client: CTAClient) {
        let url = cta_client.base_url();

        assert_eq!(String::from("http://lapi.transitchicago.com/api/1.0"), url);
    }

    #[rstest]
    #[case(Vec::from([]), "www.example.com/test.aspx?key=TESTKEY&outputType=JSON")]
    #[case(Vec::from([("mapid", "12345")]), "www.example.com/test.aspx?key=TESTKEY&mapid=12345&outputType=JSON")]
    #[case(Vec::from([("stpid", "12345")]), "www.example.com/test.aspx?key=TESTKEY&outputType=JSON&stpid=12345")]
    #[case(Vec::from([("stpid", "12345"), 
                      ("mapid", "54321")]), "www.example.com/test.aspx?key=TESTKEY&mapid=54321&outputType=JSON")]
    #[case(Vec::from([("mapid", "54321"), 
                      ("stpid", "12345")]), "www.example.com/test.aspx?key=TESTKEY&outputType=JSON&stpid=12345")]
    #[case(Vec::from([("mapid", "54321"), 
                      ("stpid", "12345"),
                      ("mapid", "12345")]), "www.example.com/test.aspx?key=TESTKEY&mapid=12345&outputType=JSON")]                      
    fn cta_client_build_url(mut cta_client: CTAClient, #[case] params: Vec<(&str, &str)> , #[case] expected: &str) {

        for (k, v) in params {
            if k == "stpid" {
                cta_client = cta_client.stpid(String::from(v));
            }
            else if k == "mapid" {
                cta_client = cta_client.mapid(String::from(v));
            }
        }

        let url = cta_client.build_url(String::from("www.example.com/test.aspx"));

        assert_eq!(String::from(expected), url);
    }
}