use std::fmt;

use serde_derive::Deserialize;
use serde_json::Value;

use chrono::prelude::*;
use chrono::{DateTime, NaiveDateTime};
use chrono_tz::US::Central;
use chrono_tz::Tz;

#[allow(dead_code)]
#[derive(Debug)]
pub enum ResponseError {
    ParsingFailed,
    NoTrain
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Arrival {
    stop_destination: String,
    station_name: String,
    destination_name: String,
    destination_number: String,
    route_number: String,
    arrival_time: DateTime<Tz>,
    current_time: DateTime<Tz>,
    is_delayed: bool,
    is_scheduled: bool,
    is_due: bool
}

impl fmt::Display for Arrival {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let arrives_min = (self.arrival_time
            .signed_duration_since(self.current_time)
            .num_seconds() + (60/2)) / 60;

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
            destination_number: String::from(&eta.dest_st),
            route_number: String::from(&eta.rn),
            arrival_time,
            current_time,
            is_delayed: eta.is_dly.eq("1"),
            is_scheduled: eta.is_sch.eq("1"),
            is_due: eta.is_app.eq("1")
        }
    }
}

#[derive(Debug)]
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

#[allow(dead_code)]
impl Arrivals {
    pub fn new(etas: Vec<Eta>) -> Self {   
        Arrivals{
            arrivals: etas
                        .into_iter()
                        .map(Arrival::new)
                        .collect()
        }
    }

    pub fn by_destination(&self, dest: String) -> Vec<&Arrival> {
        self.arrivals.iter()
            .filter(|a| {
                a.destination_number.eq(&dest) || 
                a.destination_name.eq(&dest)
            }).collect()
    }

    pub fn by_route_number(&self, route_number: String) -> Vec<&Arrival> {
        self.arrivals.iter()
            .filter(|a| {
                a.route_number.eq(&route_number)
            }).collect()
    }

    pub fn by_due(&self) -> Vec<&Arrival> {
        self.arrivals.iter()
            .filter(|a| {
                a.is_due
            }).collect()
    }

    pub fn by_delayed(&self) -> Vec<&Arrival> {
        self.arrivals.iter()
            .filter(|a| {
                a.is_delayed
            }).collect()
    }

    pub fn by_scheduled(&self) -> Vec<&Arrival> {
        self.arrivals.iter()
            .filter(|a| {
                a.is_scheduled
            }).collect()
    }

    pub fn by_live(&self) -> Vec<&Arrival> {
        self.arrivals.iter()
            .filter(|a| {
                !a.is_scheduled
            }).collect()
    }

}

#[derive(Debug)]
pub struct ETAResponse {
    pub arrivals: Arrivals
}

impl fmt::Display for ETAResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.arrivals)
    }
}

impl ETAResponse {
    pub fn new(data: String) -> Result<Self, ResponseError> {
        let root = serde_json::from_str::<Root>(&data)
            .map_err(|_err: serde_json::Error| {
                ResponseError::ParsingFailed
            })?;

        Ok(ETAResponse {
            arrivals: Arrivals::new(root.ctatt.eta)
        })
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


