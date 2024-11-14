use reqwest;

pub fn fetch_satellite_data(coordinates: &str) -> Result<String, reqwest::Error> {
    let api_url = format!("https://api.sentinel-hub.com/ogc/wms/{}", coordinates);
    let response = reqwest::blocking::get(&api_url)?;
    Ok(response.text()?)
}
