use std::fmt;
use std::collections::BTreeMap;

#[cfg(test)]
#[path="./client_test.rs"]
mod client_test;

#[path="./responses.rs"]
mod responses;
use responses::{ETAResponse, ResponseError};


pub trait CTAClientRequest {
    fn get(&self, url: String) -> Result<String, CTAClientError>;
}

#[derive(Debug, PartialEq, Eq)]
pub enum CTAClientError {
    RequiredArgMissing,
    RequestFailed
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct CTAClient {
    url: String,
    version: f32,
    max_number_params: u32,
    params: BTreeMap<String, String>
}

impl fmt::Display for CTAClient {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CTAClient")
    }
}

#[allow(dead_code)]
impl CTAClient {
    pub fn new(cta_key: String) -> Self {
        CTAClient {
            url: String::from("http://lapi.transitchicago.com/api"),
            version: 1.0,
            max_number_params: 4,
            params:BTreeMap::from([
                            (String::from("key"), cta_key),
                            (String::from("outputType"), String::from("JSON"))
                            ]),
        }
    }

    fn base_url(&self) -> String {
        format!("{}/{:.1}", self.url, self.version)
    }

    fn build_url(&self, url: String) -> Result<String, CTAClientError> {
        if !self.params.contains_key("mapid") && !self.params.contains_key("stpid") {
            return Err(CTAClientError::RequiredArgMissing);
        }

        Ok(format!(
            "{}?{}", 
            url, 
            self.params
                .iter()
                .map(|(k, v)| format!("{k}={v}"))
                .collect::<Vec<String>>()
                .join("&")
            ))
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

    pub fn arrivals(&self/*, request: &impl CTAClientRequest*/) -> Result<ETAResponse, CTAClientError> {
        let resp = self.get(
            self.build_url(
                format!("{}/ttarrivals.aspx", self.base_url()))?)?;

        ETAResponse::new(
            resp)
                .map_err(|_err: ResponseError| {
                    CTAClientError::RequestFailed
                })
    }

}
