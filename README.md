An AmbientWeather.net REST API client written in Rust. In a very early, hacky state. You will need to edit main.rs to add your api key and application key.

# TODOs:
 - Implement a config file for api/app key values
 - Implement querying the API for a specific device (currently the API only allows queries by MAC address)
 - Attempt to implement dynamic struct creation instead of explicitly declaring >100 JSON fields
