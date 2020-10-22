use serde::Deserialize;
use reqwest::Error;

#[derive(Deserialize, Debug)]
struct Devices {
    macAddress: String,
    info: Info,
    lastData: LastData
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
enum Info {
    String
}

#[derive(Deserialize, Debug)]
struct LastData {
    dateutc: i32,
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
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
  let application_key = "";
  let api_key = "";
  let request_url = format!("https://api.ambientweather.net/v1/applicationKey={application_key}&apiKey={api_key}",
                            application_key = application_key,
                            api_key = api_key);
  let response = reqwest::get(&request_url).await?;
  let devices: Vec<Devices> = response.json().await?;
  println!("{:?}", devices);
  Ok(())
}
