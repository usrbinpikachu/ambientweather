extern crate serde_derive

use serde::Deserialize;
use reqwest::Error;

pub type Devices = Vec<Device>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Device {
    #[serde(rename = "macAddress")]
    mac_address: String,
    #[serde(rename = "lastData")]
    last_data: LastData,
    info: Info,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Info {
    name: String,
    coords: InfoCoords,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InfoCoords {
    coords: CoordsCoords,
    address: String,
    location: String,
    elevation: f64,
    geo: Geo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoordsCoords {
    lon: f64,
    lat: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Geo {
    #[serde(rename = "type")]
    geo_type: String,
    coordinates: Vec<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LastData {
    dateutc: Option<i64>,
    tempinf: Option<f64>,
    humidityin: Option<i64>,
    baromrelin: Option<f64>,
    baromabsin: Option<f64>,
    tempf: Option<f64>,
    battout: Option<i64>,
    humidity: Option<i64>,
    winddir: Option<i64>,
    windspeedmph: Option<f64>,
    windgustmph: Option<f64>,
    maxdailygust: Option<f64>,
    hourlyrainin: Option<i64>,
    eventrainin: Option<i64>,
    dailyrainin: Option<i64>,
    weeklyrainin: Option<f64>,
    monthlyrainin: Option<f64>,
    totalrainin: Option<f64>,
    solarradiation: Option<f64>,
    uv: Option<i64>,
    #[serde(rename = "temp1f")]
    temp1_f: Option<f64>,
    humidity1: Option<i64>,
    #[serde(rename = "temp2f")]
    temp2_f: Option<f64>,
    humidity2: Option<i64>,
    #[serde(rename = "temp3f")]
    temp3_f: Option<f64>,
    humidity3: Option<i64>,
    #[serde(rename = "temp4f")]
    temp4_f: Option<f64>,
    humidity4: Option<i64>,
    #[serde(rename = "temp5f")]
    temp5_f: Option<f64>,
    humidity5: Option<i64>,
    #[serde(rename = "temp6f")]
    temp6_f: Option<f64>,
    humidity6: Option<i64>,
    #[serde(rename = "temp7f")]
    temp7_f: Option<f64>,
    humidity7: Option<i64>,
    #[serde(rename = "temp8f")]
    temp8_f: Option<f64>,
    humidity8: Option<i64>,
    #[serde(rename = "temp9f")]
    temp9_f: Option<f64>,
    humidity9: Option<i64>,
    #[serde(rename = "temp10f")]
    temp10_f: Option<f64>,
    humidity10: Option<i64>,
    batt1: Option<i64>,
    batt2: Option<i64>,
    batt3: Option<i64>,
    batt4: Option<i64>,
    batt5: Option<i64>,
    batt6: Option<i64>,
    batt7: Option<i64>,
    batt8: Option<i64>,
    batt9: Option<i64>,
    batt10: Option<i64>,
    #[serde(rename = "feelsLike")]
    feels_like: Option<f64>,
    #[serde(rename = "dewPoint")]
    dew_point: Option<f64>,
    #[serde(rename = "feelsLike1")]
    feels_like1: Option<f64>,
    #[serde(rename = "dewPoint1")]
    dew_point1: Option<f64>,
    #[serde(rename = "feelsLike2")]
    feels_like2: Option<f64>,
    #[serde(rename = "dewPoint2")]
    dew_point2: Option<f64>,
    #[serde(rename = "feelsLike3")]
    feels_like3: Option<f64>,
    #[serde(rename = "dewPoint3")]
    dew_point3: Option<f64>,
    #[serde(rename = "feelsLike4")]
    feels_like4: Option<f64>,
    #[serde(rename = "dewPoint4")]
    dew_point4: Option<f64>,
    #[serde(rename = "feelsLike5")]
    feels_like5: Option<f64>,
    #[serde(rename = "dewPoint5")]
    dew_point5: Option<f64>,
    #[serde(rename = "feelsLike6")]
    feels_like6: Option<f64>,
    #[serde(rename = "dewPoint6")]
    dew_point6: Option<f64>,
    #[serde(rename = "feelsLike7")]
    feels_like7: Option<f64>,
    #[serde(rename = "dewPoint7")]
    dew_point7: Option<f64>,
    #[serde(rename = "feelsLike8")]
    feels_like8: Option<f64>,
    #[serde(rename = "dewPoint8")]
    dew_point8: Option<f64>,
    #[serde(rename = "feelsLike9")]
    feels_like9: Option<f64>,
    #[serde(rename = "dewPoint9")]
    dew_point9: Option<f64>,
    #[serde(rename = "feelsLike10")]
    feels_like10: Option<f64>,
    #[serde(rename = "dewPoint10")]
    dew_point10: Option<f64>,
    #[serde(rename = "feelsLikein")]
    feels_likein: Option<f64>,
    #[serde(rename = "dewPointin")]
    dew_pointin: Option<f64>,
    #[serde(rename = "soiltemp1f")]
    soil_temp1_f: Option<f64>,
    #[serde(rename = "soilhum1")]
    soil_hum1: Option<f64>,
    #[serde(rename = "soiltemp2f")]
    soil_temp2_f: Option<f64>,
    #[serde(rename = "soilhum2")]
    soil_hum2: Option<f64>,
    #[serde(rename = "soiltemp3f")]
    soil_temp3_f: Option<f64>,
    #[serde(rename = "soilhum3")]
    soil_hum3: Option<f64>,
    #[serde(rename = "soiltemp4f")]
    soil_temp4_f: Option<f64>,
    #[serde(rename = "soilhum4")]
    soil_hum4: Option<f64>,
    #[serde(rename = "soiltemp5f")]
    soil_temp5_f: Option<f64>,
    #[serde(rename = "soilhum5")]
    soil_hum5: Option<f64>,
    #[serde(rename = "soiltemp6f")]
    soil_temp6_f: Option<f64>,
    #[serde(rename = "soilhum6")]
    soil_hum6: Option<f64>,
    #[serde(rename = "soiltemp7f")]
    soil_temp7_f: Option<f64>,
    #[serde(rename = "soilhum7")]
    soil_hum7: Option<f64>,
    #[serde(rename = "soiltemp1f")]
    soil_temp8_f: Option<f64>,
    #[serde(rename = "soilhum8")]
    soil_hum8: Option<f64>,
    #[serde(rename = "soiltemp9f")]
    soil_temp9_f: Option<f64>,
    #[serde(rename = "soilhum1")]
    soil_hum9: Option<f64>,
    #[serde(rename = "soiltemp10f")]
    soil_temp10_f: Option<f64>,
    #[serde(rename = "soilhum10")]
    soil_hum10: Option<f64>,
    #[serde(rename = "lastRain")]
    last_rain: Option<String>,
    tz: Option<String>,
    date: Option<String>,
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
