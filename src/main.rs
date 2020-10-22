#![allow(non_snake_case)]
use serde::Deserialize;
use reqwest::Error;

#[derive(Deserialize, Debug)]
struct Devices(Vec<Device>);

#[derive(Deserialize, Debug)]
struct Device {
    macAddress: String,
    lastData: LastData,
    info: Info
}

#[derive(Deserialize, Debug)]
struct Info {
    name: String,
    coords: Coords
}

#[derive(Deserialize, Debug)]
struct Coords {
    coords: LonLat,
    address: String,
    location: String,
    elevation: f64,
    geo: Geo
}

#[derive(Deserialize, Debug)]
struct Geo {
    r#type: String,
    coordinates: (f64, f64)
}

#[derive(Deserialize, Debug)]
struct LonLat {
    lon: f64,
    lat: f64
}

#[derive(Deserialize, Debug)]
struct LastData {
    dateutc: i64,
    tempinf: f64,
    humidityin: f64,
    baromrelin: f64,
    baromabsin: f64,
    tempf: f64,
    battout: i8,
    humidity: f64,
    winddir: i16,
    windspeedmph: f64,
    windgustmph: f64,
    maxdailygust: f64,
    hourlyrainin: f64,
    eventrainin: f64,
    dailyrainin: f64,
    weeklyrainin: f64,
    monthlyrainin: f64,
    totalrainin: f64,
    solarradiation: f64,
    uv: i8,
    temp1f: f64,
    humidity1: f64,
    temp2f: f64,
    humidity2: f64,
    batt1: f32,
    batt2: f32,
    feelsLike: f64,
    dewPoint: f64,
    feelsLike1: f64,
    dewPoint1: f64,
    feelsLike2: f64,
    dewPoint2: f64,
    feelsLikein: f64,
    dewPointin: f64,
    lastRain: String,
    tz: String,
    date: String
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let application_key = "";
    let api_key = "";
    let request_url = format!("https://api.ambientweather.net/v1/devices?applicationKey={application_key}&apiKey={api_key}",
                              application_key = application_key,
                              api_key = api_key);
    let response = reqwest::get(&request_url).await?.json::<Devices>().await?;
    println!("{:#?}", response);
    Ok(())
}
