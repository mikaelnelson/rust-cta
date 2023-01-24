use std::env;
use std::collections::BTreeMap;

#[cfg(test)]
#[path="./client_test.rs"]
mod client_test;

#[path="./responses.rs"]
mod responses;
use responses::*;

#[derive(Debug)]
pub enum CTAClientError {
    MissingCTAKey,
    RequiredArgMissing,
    RequestFailed
}

#[allow(dead_code)]
pub struct CTAClient {

    url: String,
    version: f32,
    max_number_params: u32,
    params: BTreeMap<String, String>
}

impl CTAClient {
    pub fn new(key: Option<String>) -> Result<Self, CTAClientError> {
        let cta_key = key
            .unwrap_or_else(|| env::var("CTA_KEY")
                .unwrap_or_default()
            );

        if cta_key.is_empty() {
            return Err(CTAClientError::MissingCTAKey);
        }

        Ok(CTAClient {
            url: String::from("http://lapi.transitchicago.com/api"),
            version: 1.0,
            max_number_params: 4,
            params:BTreeMap::from([
                            (String::from("key"), cta_key),
                            (String::from("outputType"), String::from("JSON"))
                            ]),
        })
    }

    fn base_url(&self) -> String {
        format!("{}/{:.1}", self.url, self.version)
    }

    fn build_url(&self, url: String) -> String {
        format!(
            "{}?{}", 
            url, 
            self.params
                .iter()
                .map(|(k, v)| format!("{k}={v}"))
                .collect::<Vec<String>>()
                .join("&")
            )
    }

    fn send_request(&self, url: String) -> Result<String, CTAClientError> {

        let url = self.build_url(url);

        let resp = match ureq::get(&url).call() {
            Ok(resp) => resp,
            Err(_e) => return Err(CTAClientError::RequestFailed)
        };

        let resp_json = match resp.into_string() {
            Ok(resp_json) => resp_json,
            Err(_e) => return Err(CTAClientError::RequestFailed)
        };

        Ok(resp_json)
    }

    pub fn mapid(mut self, mapid: String) -> Self {
        self.params.remove(&String::from("stpid"));
        self.params.insert(String::from("mapid"), mapid);
        self
    }

    pub fn stpid(mut self, stpid: String) -> Self {
        self.params.remove(&String::from("mapid"));
        self.params.insert(String::from("stpid"), stpid);
        self
    }

    pub fn arrivals(&self) -> Result<ETAResponse, CTAClientError> {

        if !self.params.contains_key("mapid") && !self.params.contains_key("stpid") {
            return Err(CTAClientError::RequiredArgMissing);
        }

        Ok(ETAResponse::new(self.send_request(format!("{}/ttarrivals.aspx", self.base_url()))?))
    }

}
