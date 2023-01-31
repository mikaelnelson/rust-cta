mod cta_client_tests {
    use crate::client::{CTAClient, CTAClientError, CTAClientRequest};
    use rstest::*;

    #[fixture]
    fn cta_client() -> CTAClient {
        CTAClient::new(String::from("TESTKEY"))
    }

    #[rstest]
    fn cta_client_base_url(cta_client: CTAClient) {
        let url = cta_client.base_url();

        assert_eq!(String::from("http://lapi.transitchicago.com/api/1.0"), url);
    }

    #[rstest]
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

        assert!(url.is_ok());
        assert_eq!(String::from(expected), url.unwrap());
    }

    #[rstest]
    fn cta_client_build_url_fail(cta_client: CTAClient) {
        let url = cta_client.build_url(String::from("www.example.com/test.aspx"));

        assert!(url.is_err());
        assert_eq!(CTAClientError::RequiredArgMissing, url.unwrap_err());
    }

    #[rstest]
    fn cta_client_arrivals_missing_args(cta_client: CTAClient) {
        struct MockRequest();
        impl CTAClientRequest for MockRequest {
            fn get(&self, _url: String) -> Result<String, CTAClientError> {
                Ok(String::from("test"))
            }
        }

        let resp = cta_client.arrivals(&MockRequest());

        assert!(resp.is_err());
        assert_eq!(CTAClientError::RequiredArgMissing, resp.unwrap_err());
    }

    #[rstest]
    fn cta_client_arrivals(cta_client: CTAClient) {

        struct MockRequest();

        impl CTAClientRequest for MockRequest {
            fn get(&self, _url: String) -> Result<String, CTAClientError> {

                let json_resp = String::from("{ 
                    \"ctatt\":{ 
                        \"tmst\":\"2015-04-30T20:23:53\",
                        \"errCd\":\"0\",
                        \"errNm\":null,
                        \"eta\":[ 
                           { 
                               \"staId\":\"40960\",
                               \"stpId\":\"30185\",
                               \"staNm\":\"Pulaski\",
                               \"stpDe\":\"Service toward Loop\",
                               \"rn\":\"726\",
                               \"rt\":\"Org\",
                               \"destSt\":\"30182\",
                               \"destNm\":\"Loop\",
                               \"trDr\":\"1\",
                               \"prdt\":\"2015-04-30T20:23:32\",
                               \"arrT\":\"2015-04-30T20:25:32\",
                               \"isApp\":\"0\",
                               \"isSch\":\"0\",
                               \"isDly\":\"0\",
                               \"isFlt\":\"0\",
                               \"flags\":null,
                               \"lat\":\"41.78661\",
                               \"lon\":\"-87.73796\",
                               \"heading\":\"357\"
                           }
                        ]
                    }
                }");
                
                Ok(json_resp)
            }
        }

        let arrivals = cta_client.mapid(String::from("40590")).mapid(String::from("40590")).arrivals(&MockRequest());
        assert!(arrivals.is_ok());

        let arrivals = arrivals.unwrap().arrivals;

        assert_eq!(arrivals.by_delayed().len(), 0);
        assert_eq!(arrivals.by_due().len(), 0);

        todo!("Update mapid to reflect response & add missing tests");

    }
}