use serde::Deserialize;

use chrono::NaiveDateTime;

use reqwest;

use crate::errors::*;

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct Flight {
    pub flight_status: String,
    pub flight_remarks_adjusted: String,
    pub flight_scheduled_time: Option<NaiveDateTime>,
    pub flight_estimated_time: Option<NaiveDateTime>,
    pub flight_number: String,
    pub flight_airline_name: String,
    pub flight_aircraft_type: String,
    pub flight_desk_to: String,
    pub flight_desk_from: String,
    pub flight_carousel: String,
    pub flight_range: String,
    pub flight_carrier: String,
    pub flight_city: String,
    pub flight_type: String,
    pub flight_airport_code: String,
    pub flight_gate: String,
    pub flight_remarks: String,
    #[serde(rename = "FlightID")]
    pub flight_id: u64,
    pub flight_quick_connect: String,
}

#[derive(Debug, Deserialize)]
struct ODataFlights {
    value: Vec<Flight>
}

pub fn get_flights() -> Result<Vec<Flight>> {
    let res: ODataFlights = reqwest::get("http://www.yvr.ca/en/_api/Flights?%24filter=((FlightScheduledTime%20gt%20DateTime%272019-07-11T00%3A00%3A00%27%20and%20FlightScheduledTime%20lt%20DateTime%272019-07-12T00%3A00%3A00%27%20and%20FlightType%20eq%20%27A%27)%20or%20(FlightEstimatedTime%20gt%20DateTime%272019-07-11T00%3A00%3A00%27%20and%20FlightEstimatedTime%20lt%20DateTime%272019-07-12T00%3A00%3A00%27%20and%20FlightType%20eq%20%27A%27))&%24orderby=FlightScheduledTime%20asc")?.json()?;

    Ok(res.value)
}
