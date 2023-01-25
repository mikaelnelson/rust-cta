mod client;
use client::CTAClient;

use dotenv::dotenv;

fn main() {
    dotenv().ok();   
    let cta_client = CTAClient::new(None).unwrap();

    let resp = match cta_client.mapid(String::from("40590")).arrivals() {
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
