//! # Ambient Weather API
//!
//! `ambient_weather_api` is a collection of functions for downloading current and historical data from the Ambient Weather API. It features built in support for choosing which device you want to pull data from, and has (some) safety measures built in to avoid hitting Ambient Weather's rate limits.
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
//!     // Get the current temperature
//!     let latest_data = get_latest_device_data(&api_credentials);
//!     println!("The current temp is: {}F", latest_data.tempf.unwrap());
//!
//!     // Get the historic temperatures and loop through them going back in time
//!     let historic_data = get_historic_device_data(&api_credentials);
//!     for i in 0..historic_data.len() {
//!         println!("The historic temp was: {}F", historic_data[i].tempf.unwrap());
//!     }
//! }
//! ```

use serde_json::{self, json, Value};
use std::{thread, time::Duration};

mod weather_data_struct;

#[derive(Clone)]

/// The struct for holding the API and App keys, the device idea, and whether or not to use the new API endpoint or not.
pub struct AmbientWeatherAPICredentials {
    /// The API key received from Ambient Weather.
    pub api_key: String,
    /// The Application key received from Ambient Weather.
    pub app_key: String,
    /// The device id, which for a user with a single station will be 0.
    pub device_id: usize,
    /// A bool to determine if the new API endpoint should be used. Due to problematic behavior, I recommend leaving this set to false.
    pub use_new_api_endpoint: bool,
}

/// A private function for crafting the appropriate Ambient Weather API URL.
fn get_aw_api_url(
    api_credentials: &AmbientWeatherAPICredentials,
    device_mac_address: &str,
) -> String {
    let url_endpoint = if api_credentials.use_new_api_endpoint {
        "rt"
    } else {
        "api"
    };

    format!("https://{url_endpoint}.ambientweather.net/v1/devices/{device_mac_address}?applicationKey={}&apiKey={}", api_credentials.app_key, api_credentials.api_key)
}

/// A private function that gets the raw device data from the Ambient Weather REST API, and then returns either the latest or the historical data for a device
#[tokio::main]
async fn get_raw_device_data(
    api_credentials: &AmbientWeatherAPICredentials,
    device_mac_address: String,
    retrieve_history: bool,
) -> Result<Value, reqwest::Error> {
    let device_id = api_credentials.device_id;

    let response: Value = reqwest::get(get_aw_api_url(api_credentials, &device_mac_address))
        .await?
        .json()
        .await?;

    thread::sleep(Duration::from_millis(1000));

    // If True, this will get and return the historic data for a given device
    if retrieve_history {
        let mut device_mac_address =
            response[device_id].as_object().unwrap()["macAddress"].to_string();

        device_mac_address.pop();
        device_mac_address.remove(0);

        let historical_response: Value =
            reqwest::get(get_aw_api_url(api_credentials, &device_mac_address))
                .await?
                .json()
                .await?;

        thread::sleep(Duration::from_millis(1000));

        return Ok(json!(historical_response));
    }

    Ok(json!(response[device_id]))
}

/// Gets the latest device data from the Ambient Weather API.
///
/// In order to use this API, you will need to look over the [list of device parameters](https://github.com/ambient-weather/api-docs/wiki/Device-Data-Specs) that Ambient Weather offers. Not all device parameters may be used, so make sure you are calling one that is associated with your device.
///
/// When calling the `get_latest_device_data` function, you must pass the api_credentials as a reference (`&api_credentials`), as this allows for it to be called multiple times elsewhere in a program if necessary.
///
/// # Examples
///
/// ```
/// use ambient_weather_api::*;
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
///     // Get the current temperature
///     let latest_data = get_latest_device_data(&api_credentials);
///     println!("The current temp is: {}F", latest_data.tempf.unwrap());
///
/// }
/// ```
pub fn get_latest_device_data(
    api_credentials: &AmbientWeatherAPICredentials,
) -> weather_data_struct::WeatherData {
    let raw_device_data =
        get_raw_device_data(api_credentials, "".to_string(), false).unwrap();

    serde_json::from_value(json!(raw_device_data["lastData"])).unwrap_or(weather_data_struct::WeatherData::default())
}

/// Gets the historic device data from the Ambient Weather API.
///
/// As of version 0.3.0 this now functions in an asynchronous manner.
///
/// In order to use this API, you will need to look over the [list of device parameters](https://github.com/ambient-weather/api-docs/wiki/Device-Data-Specs) that Ambient Weather offers. Not all device parameters may be used, so make sure you are calling one that is associated with your device.
///
/// # Examples
///
/// ```
/// use ambient_weather_api::*;
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
///     // Get the historic temperatures and loop through them going back in time
///     let historic_data = get_historic_device_data(&api_credentials);
///        for i in 0..historic_data.len() {
///            println!("The historic temp was: {}F", historic_data[i].tempf.unwrap());
///        }
///     
/// }
/// ```
pub fn get_historic_device_data(
    api_credentials: &AmbientWeatherAPICredentials,
) -> Vec<weather_data_struct::WeatherData> {
    let raw_device_data =
        get_raw_device_data(api_credentials, "".to_string(), true).unwrap();

    let weather_data_array: Vec<Value> = raw_device_data
        .as_array()
        .unwrap()
        .to_vec();

    weather_data_array
        .into_iter()
        .map(|data| serde_json::from_value(data)
        .unwrap())
        .collect()
}
