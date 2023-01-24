
use std::fmt;

use serde_derive::Deserialize;
use 
serde_json::Value;

use chrono::prelude::*;
use chrono::{DateTime, NaiveDateTime};
use chrono_tz::US::Central;
use chrono_tz::Tz;

// // #[derive(Debug)]
// pub enum ResponsesError {
//     BadResponse,
//     NoTrain
// }

pub struct Arrival {
    stop_destination: String,
    station_name: String,
    destination_name: String,
    arrival_time: DateTime<Tz>,
    current_time: DateTime<Tz>,
    is_delayed: bool,
    is_scheduled: bool,
    is_due: bool
}

impl fmt::Display for Arrival {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let arrives_min = self.arrival_time
            .signed_duration_since(self.current_time)
            .num_minutes();

        write!(f, "{} arrives in {} {}", 
            self.stop_destination, 
            arrives_min, 
            if arrives_min == 1 {"minute"} else {"minutes"}
        )
    }
}

impl Arrival {
    pub fn new(eta: Eta) -> Self {

        let arrival_time = NaiveDateTime::parse_from_str(&eta.arr_t, "%Y-%m-%dT%H:%M:%S")
            .unwrap()
            .and_local_timezone(Central)
            .unwrap();

        let current_time = Utc::now().with_timezone(&Central);

        Arrival{
            stop_destination: String::from(&eta.stp_de),
            station_name: String::from(&eta.sta_nm),
            destination_name: String::from(&eta.dest_nm),
            arrival_time,
            current_time,
            is_delayed: eta.is_dly.eq("1"),
            is_scheduled: eta.is_sch.eq("1"),
            is_due: eta.is_app.eq("1")
        }
    }
}

pub struct Arrivals {
    arrivals: Vec<Arrival>
}

impl fmt::Display for Arrivals {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.arrivals
                            .iter()
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
                        .map(Arrival::new)
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

        ETAResponse { arrivals }
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


