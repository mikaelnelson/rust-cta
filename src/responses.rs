
use std::fmt;

use serde_derive::Deserialize;
use 
serde_json::{Value};

// // #[derive(Debug)]
// pub enum ResponsesError {
//     BadResponse,
//     NoTrain
// }

pub struct Arrival {
    arrival_time: String
}

impl fmt::Display for Arrival {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.arrival_time)
    }
}

impl Arrival {
    pub fn new(eta: Eta) -> Self {
        Arrival{arrival_time: String::from(eta.arr_t)}
    }
}

pub struct Arrivals {
    arrivals: Vec<Arrival>
}

impl fmt::Display for Arrivals {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", (&self.arrivals)
                            .into_iter()
                            .map(|arrival| format!("{}", arrival))
                            .collect::<Vec<String>>()
                            .join("\n"))
    }
}

impl Arrivals {
    pub fn new(etas: Vec<Eta>) -> Self {
        Arrivals{
            arrivals: etas
                        .into_iter()
                        .map(|eta| Arrival::new(eta))
                        .collect()
        }
    }
}

pub struct ETAResponse {
    arrivals: Arrivals
}

impl fmt::Display for ETAResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.arrivals)
    }
}

impl ETAResponse {
    pub fn new(data: String) -> Self {

        // let root: Result<Root, ResponsesError> = match serde_json::from_str(&data) {
        //     Ok(root) => root,
        //     Err(_e) => Err(ResponsesError::BadResponse)
        // };

        // Ok(ETAResponse { arrivals: root })

        let root: Root = serde_json::from_str(&data).unwrap();
        let etas = root.ctatt.eta;
        let arrivals = Arrivals::new(etas);

        ETAResponse { arrivals: arrivals }

    }
}


#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub ctatt: Ctatt,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ctatt {
    pub tmst: String,
    pub err_cd: String,
    pub err_nm: Value,
    pub eta: Vec<Eta>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Eta {
    pub sta_id: String,
    pub stp_id: String,
    pub sta_nm: String,
    pub stp_de: String,
    pub rn: String,
    pub rt: String,
    pub dest_st: String,
    pub dest_nm: String,
    pub tr_dr: String,
    pub prdt: String,
    pub arr_t: String,
    pub is_app: String,
    pub is_sch: String,
    pub is_dly: String,
    pub is_flt: String,
    pub flags: Value,
    pub lat: String,
    pub lon: String,
    pub heading: String,
}


