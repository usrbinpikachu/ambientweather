extern crate restson;
extern crate chrono;

#[macro_use]
extern crate serde_derive;

use restson::{Error, RestClient, RestPath};
use std::time::Duration;

#[derive(Deserialize)]
struct Devices {
    macAddress: String,
    info: Info,
    lastData: LastData
};

struct Info {
    name: String,
    location: String
};

struct LastData {
    dateutc: DateTime,
    date: String,
    winddir: u8,
    windspeedmph: f64,
    windgustmph: f64,
    maxdailygust: f64,
    windgustdir: i16,
    winddir_avg2m: i16,
    windspeedmph_avg2m: f64,
    winddir_avg10m: i16,
    windspeed_avg10m: f64,
    tempf: f64,
    humidity: f64,
    baromrelin: f64,
    baromabsin: f64,
    tempinf: f64,
    humidityin: f64,
    hourlyrainin: f64,
    dailyrainin: f64,
    monthlyrainin: f64,
    yearlyrainin: f64,
    feelsLike: f64,
    dewPoint: f64
};