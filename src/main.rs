mod client;
use client::CTAClient;

use dotenv::dotenv;

fn main() {
    dotenv().ok();   
    let cta_client = CTAClient::new(None).unwrap();

    match cta_client.mapid(String::from("40590")).arrivals() {
        Ok(arrivals) => println!("{arrivals}"),
        Err(e) => println!("{:?}", e)
    };
}
