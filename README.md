# Ambient Weather API

`ambient_weather_api` is a collection of functions for downloading current and historical data from the Ambient Weather API. It features built in support for choosing which device you want to pull data from, and has (some) safety measures built in to avoid hitting Ambient Weather's rate limits. 

To learn more about how the Ambient Weather API works, and to obtain the required API and Application keys to use this creat, check out the [Ambient Weather API Documentation](https://ambientweather.docs.apiary.io).

In order to use this API, you will need to look over the [list of device parameters](https://github.com/ambient-weather/api-docs/wiki/Device-Data-Specs) that Ambient Weather offers. Not all device parameters may be used, so make sure you are calling one that is associated with your device.

Currently, this Rust crate is only capable of utilizing the Ambient Weather REST API. Support for their Realtime Socket.IO API will come at a later date.

# Getting Started

To get started with pulling in the latest weather data from your Ambient Weather device, simply follow the example below:

```Rust
use ambient_weather_api::*;

fn main() {

    let api_credentials = AmbientWeatherAPICredentials {
        api_key: String::from("Your API Key"),
        app_key: String::from("Your Application Key"),
        device_id: 0,
        use_new_api_endpoint: false,
    };
    
    // Get the current temperature
    let latest_data = get_latest_aw_device_data(&api_credentials);
    println!("The current temp is: {}F", latest_data.tempf.unwrap());

    // Get the historic temperatures and loop through them going back in time
    let historic_data = get_historic_aw_device_data(&api_credentials);
    for i in 0..historic_data.len() {
        println!("The historic temp was: {}F", historic_data[i].tempf.unwrap());
    }
}
```