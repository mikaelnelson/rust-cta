mod client;
use client::CTAClient;

use dotenv::dotenv;

fn main() {
    dotenv().ok();   
    let cta_client = CTAClient::new(None).unwrap();

    // let params: HashMap<String, String> = HashMap::from([
    //     (String::from("mapid"), String::from("40590"))
    // ]);
    // let arrivals = cta_client.arrivals(params).unwrap();


    match cta_client.mapid(String::from("40590")).arrivals() {
        Ok(arrivals) => println!("{arrivals}"),
        Err(e) => println!("{:?}", e)
    };
}
