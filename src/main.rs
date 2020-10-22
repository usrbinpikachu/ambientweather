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
    dateutc: Option<i64>,
    tempinf: Option<f64>,
    humidityin: Option<f64>,
    baromrelin: Option<f64>,
    baromabsin: Option<f64>,
    tempf: Option<f64>,
    battout: Option<i8>,
    humidity: Option<f64>,
    winddir: Option<i16>,
    winddir_avg2m: Option<i16>,
    winddir_avg10m: Option<i16>,
    windspeedmph: Option<f64>,
    windspdmph_avg2m: Option<f64>,
    windspdmph_avg10m: Option<f64>,
    windgustmph: Option<f64>,
    maxdailygust: Option<f64>,
    hourlyrainin: Option<f64>,
    eventrainin: Option<f64>,
    dailyrainin: Option<f64>,
    weeklyrainin: Option<f64>,
    monthlyrainin: Option<f64>,
    totalrainin: Option<f64>,
    solarradiation: Option<f64>,
    uv: Option<i8>,
    temp1f: Option<f64>,
    humidity1: Option<f64>,
    temp2f: Option<f64>,
    humidity2: Option<f64>,
    temp3f: Option<f64>,
    humidity3: Option<f64>,
    temp4f: Option<f64>,
    humidity4: Option<f64>,
    temp5f: Option<f64>,
    humidity5: Option<f64>,
    temp6f: Option<f64>,
    humidity6: Option<f64>,
    temp7f: Option<f64>,
    humidity7: Option<f64>,
    temp8f: Option<f64>,
    humidity8: Option<f64>,
    temp9f: Option<f64>,
    humidity9: Option<f64>,
    temp10f: Option<f64>,
    humidity10: Option<f64>,
    batt1: Option<i8>,
    batt2: Option<i8>,
    batt3: Option<i8>,
    batt4: Option<i8>,
    batt5: Option<i8>,
    batt6: Option<i8>,
    batt7: Option<i8>,
    batt8: Option<i8>,
    batt9: Option<i8>,
    batt10: Option<i8>,
    batt_25: Option<i8>,
    co2: Option<f32>,
    pm25: Option<f32>,
    pm25_24h: Option<f32>,
    pm25_in: Option<f32>,
    pm25_in_24h: Option<f32>,
    feelsLike: Option<f64>,
    dewPoint: Option<f64>,
    feelsLike1: Option<f64>,
    dewPoint1: Option<f64>,
    feelsLike2: Option<f64>,
    dewPoint2: Option<f64>,
    feelsLikein: Option<f64>,
    dewPointin: Option<f64>,
    lastRain: Option<String>,
    tz: Option<String>,
    date: Option<String>
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
