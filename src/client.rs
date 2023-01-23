use std::env;
use std::collections::{HashMap};

use ureq;

#[cfg(test)]
#[path = "./client_test.rs"]
mod client_test;


#[allow(dead_code)]
pub struct ParamBuilder {
    key: String
}

impl ParamBuilder {
    pub fn new(key: String) -> Self {
        return ParamBuilder{key};
    }

    pub fn build_url(self, url: String, params: HashMap<String, String>) -> String {
        let mut params_vec: Vec<String>  = Vec::new();
        for (key, value) in self.build(params) {
            params_vec.push(format!("{}={}", key, value));
        };

        let url = format!("{}?{}", url, params_vec.join("&"));

        return url;
    }

    pub fn build(&self, params: HashMap<String, String>) -> HashMap<String, String> {
        let mut ret_params: HashMap<String, String> = HashMap::from([
            (String::from("key"), String::from(&self.key)),
            (String::from("outputType"), String::from("JSON"))
        ]);

        for (key, value) in params {
            ret_params.insert(key, value);
        }

        return ret_params;
    }
}

#[derive(Debug)]

pub enum CTAClientError {
    MissingCTAKey,
    RequiredArgMissing,
    RequestFailed
}

#[allow(dead_code)]
pub struct CTAClient {

    url: String,
    key: String,
    version: f32,
    max_number_params: u32,
    builder: ParamBuilder
}

impl CTAClient {
    pub fn new(key: Option<String>) -> Result<Self, CTAClientError> {
        let cta_key = key
            .unwrap_or(env::var("CTA_KEY")
                .unwrap_or(String::new())
            );


            if cta_key.is_empty() {
                return Err(CTAClientError::MissingCTAKey);
            }

        Ok(CTAClient {
            url: String::from("http://lapi.transitchicago.com/api"),
            key: cta_key.to_owned(),
            version: 1.0,
            max_number_params: 4,
            builder: ParamBuilder::new(cta_key.to_owned())
        })
    }

    fn base_url(&self) -> String {
        return format!("{}/{:.1}", self.url, self.version);
    }

    fn send_request(self, url: String, params: HashMap<String, String>) -> Result<String, CTAClientError> {
        let url = self.builder.build_url(url, params);

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

    pub fn arrivals(self, params: HashMap<String, String>) -> Result<String, CTAClientError> {

        if !params.contains_key("mapid") && !params.contains_key("stpid") {
            return Err(CTAClientError::RequiredArgMissing);
        }
        else if params.contains_key("mapid") && params.contains_key("stpid")  {
            todo!("Warn user about using mapid and stpid");
        }

        let url = format!("{}/ttarrivals.aspx", self.base_url());

        let params = self.builder.build(params);

        let data = self.send_request(url, params)?;

        Ok(data)
    }

}
