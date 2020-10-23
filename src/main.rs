use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_json::Value;
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
  #[serde(flatten)]
  #[serde(rename = "lastData")]
  last_data: HashMap<String, Value>,
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
