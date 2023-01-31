use std::env;

mod client;
use client::{CTAClient, CTAClientError, CTAClientRequest};

use dotenv::dotenv;

struct Request();

impl CTAClientRequest for Request {
    fn get(&self, url: String) -> Result<String, CTAClientError> {
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
}

fn main() {
    dotenv().ok();   

    let cta_client = CTAClient::new(env::var("CTA_KEY").unwrap());

    let resp = match cta_client.mapid(String::from("40590")).arrivals(&Request()) {
        Ok(arrivals) => arrivals,
        Err(e) => panic!("{:?}", e)
    };

    let fp_dst = resp.arrivals.by_destination(String::from("Forest Park"));

    for dest in fp_dst {
        println!("{dest}");
    }

    let oh_dst = resp.arrivals.by_destination(String::from("O'Hare"));
    for dest in oh_dst {
        println!("{dest}");
    }

    println!("Trains Due:");
    for dest in resp.arrivals.by_due() {
        println!("{dest}");
    }

    println!("Trains Scheduled:");
    for dest in resp.arrivals.by_scheduled() {
        println!("{dest}");
    }

    println!("Trains Live:");
    for dest in resp.arrivals.by_live() {
        println!("{dest}");
    }
}
