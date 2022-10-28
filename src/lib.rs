//! # Ambient Weather API
//! 
//! `ambient_weather_api` is a collection of functions for downloading current and historical data from the Ambient Weather API. It features built in support for choosing which device you want to pull data from, and has safety measures built in to avoid hitting Ambient Weather's rate limits. 
//! 
//! To learn more about how the Ambient Weather API works, and to obtain the required API and Application keys to use this creat, check out the [Ambient Weather API Documentation](https://ambientweather.docs.apiary.io).
//! 
//! In order to use this API, you will need to look over the [list of device parameters](https://github.com/ambient-weather/api-docs/wiki/Device-Data-Specs) that Ambient Weather offers. Not all device parameters may be used, so make sure you are calling one that is associated with your device.
//! 
//! Currently, this Rust crate is only capable of utilizing the Ambient Weather REST API. Support for their Realtime Socket.IO API will come at a later date.
//! 
//! # Getting Started
//! 
//! To get started with pulling in the latest weather data from your Ambient Weather device, simply follow the example below:
//! 
//! ```
//! use ambient_weather_api::*;
//! 
//! fn main() {
//!
//!     let api_credentials = AmbientWeatherAPICredentials {
//!         api_key: String::from("Your API Key"),
//!         app_key: String::from("Your Application Key"),
//!         device_id: 0,
//!         use_new_api_endpoint: false,
//!     };
//!
//!     let latest_data = get_latest_aw_device_data(api_credentials);
//!
//!     println!("{}", latest_data["tempf"]);
//! }
//! ```

use reqwest;
use serde_json::{self, Value, json};
use std::{thread, time::Duration};

#[derive(Clone)]

/// The struct for holding the API and App keys, the device idea, and whether or not to use the new API endpoint or not.
pub struct AmbientWeatherAPICredentials {
    /// The API key received from Ambient Weather.
    pub api_key: String,
    /// The Application key received from Ambient Weather.
    pub app_key: String,
    /// The device idea, which for a user with a single station will be 0.
    pub device_id: usize,
    /// A bool to determine if the new API endpoint should be used. Due to problematic behavior, I recommend leaving this set to false.
    pub use_new_api_endpoint: bool,
}

/// A private function for crafting the appropriate Ambient Weather API URL.
fn get_aw_api_url(api_credentials: AmbientWeatherAPICredentials, device_mac_address: &str) -> String {

    let url_endpoint;

    if api_credentials.use_new_api_endpoint {
        url_endpoint = "rt";
    } else {
        url_endpoint = "api";
    }

    let ambient_weather_url = format!("https://{}.ambientweather.net/v1/devices/{}?applicationKey={}&apiKey={}", url_endpoint, device_mac_address, api_credentials.app_key, api_credentials.api_key);

    return ambient_weather_url;
}

/// Advanced: Gets the latest raw device data from the Ambient Weather REST API. 
/// 
/// Unless you have a specific need to access the raw device data, I would not recommend using this, due to the messy nature of the output.
pub fn get_raw_latest_aw_device_data(api_credentials: AmbientWeatherAPICredentials, device_mac_address: String) -> Result<Value, reqwest::Error> {

    let device_id = api_credentials.device_id;

    let response: Value = reqwest::blocking::get(get_aw_api_url(api_credentials, &device_mac_address))?
        .json()?;

    thread::sleep(Duration::from_millis(1000));
 
    return Ok(json!(response[device_id]))
}

/// Advanced: Gets the historical raw data from the Ambient Weather REST API.
/// 
/// Unless you have a specific need to access the raw device data, I would not recommend using this, due to the messy nature of the output.
pub fn get_raw_historic_aw_device_data(api_credentials: AmbientWeatherAPICredentials) -> Result<Value, reqwest::Error> {

    let mut device_mac_address = json!(get_raw_latest_aw_device_data(Clone::clone(&api_credentials), "".to_string()).unwrap()["macAddress"]).to_string();

    device_mac_address.pop();
    device_mac_address.remove(0);


    thread::sleep(Duration::from_millis(1000));

    let response: Value = reqwest::blocking::get(get_aw_api_url(api_credentials, &device_mac_address.to_string()))?
        .json()?;

    return Ok(json!(response))
}

/// Gets the latest device data from the Ambient Weather API.
/// 
/// Currently does so in a blocking manner. Asyncronus support will be added eventually.
/// 
/// In order to use this API, you will need to look over the [list of device parameters](https://github.com/ambient-weather/api-docs/wiki/Device-Data-Specs) that Ambient Weather offers. Not all device parameters may be used, so make sure you are calling one that is associated with your device.
/// 
/// # Examples
/// 
/// ```
/// use ambient_weather_api::get_latest_aw_device_data;
/// 
/// fn main() {
///
///     let api_credentials = AmbientWeatherAPICredentials {
///         api_key: String::from("Your API Key"),
///         app_key: String::from("Your Application Key"),
///         device_id: 0,
///         use_new_api_endpoint: false,
///     };
///
///     let latest_data = get_latest_aw_device_data(api_credentials);
///
///     println!("{}", latest_data["tempf"]);
/// }
/// ```
pub fn get_latest_aw_device_data(api_credentials: AmbientWeatherAPICredentials) -> Value {
    let latest_raw_device_data = get_raw_latest_aw_device_data(api_credentials, "".to_string()).unwrap();

    return json!(latest_raw_device_data["lastData"])
}

/// Gets the historic device data from the Ambient Weather API.
/// 
/// Currently does so in a blocking manner. Asyncronus support will be added eventually.
/// 
/// In order to use this API, you will need to look over the [list of device parameters](https://github.com/ambient-weather/api-docs/wiki/Device-Data-Specs) that Ambient Weather offers. Not all device parameters may be used, so make sure you are calling one that is associated with your device.
/// 
/// # Examples
/// 
/// ```
/// use ambient_weather_api::get_historic_aw_device_data;
/// 
/// fn main() {
///
///     let api_credentials = AmbientWeatherAPICredentials {
///         api_key: String::from("Your API Key"),
///         app_key: String::from("Your Application Key"),
///         device_id: 0,
///         use_new_api_endpoint: false,
///     };
///
///     let historic_data = get_historic_aw_device_data(api_credentials);
///
///     println!("{}", historic_data[0]["tempf"]);
/// }
/// ```
pub fn get_historic_aw_device_data(api_credentials: AmbientWeatherAPICredentials) -> Value {
    let historic_raw_device_data = get_raw_historic_aw_device_data(api_credentials).unwrap();

    return json!(historic_raw_device_data)
}