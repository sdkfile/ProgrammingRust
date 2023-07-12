use std::io;

fn main() {
    println!("Hello, world!");
}

struct WeatherReport {
    temp: String,
    wind: String,
    humidity: String,
}

fn get_weather(location: &str) -> Result<WeatherReport, io::Error> {
    let url = format!("https://wttr.in/{}", location);
    let resp = reqwest::blocking::get(&url)?;
    if resp.status().is_success() {
        Ok(WeatherReport::parse(resp.text()?))
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!("failed to get weather from {}", url),
        ))
    }
}

fn display_weather(hometown: &str, weather: WeatherReport) {
    println!("Weather for {}:", hometown);
    println!("  Temperature: {}", weather.temp);
    println!("  Wind: {}", weather.wind);
    println!("  Humidity: {}", weather.humidity);
}
fn schedule_weather_retry() {
    let mut retry_times: i32 = 0;
    loop {
        match get_weather("Seoul") {
            Ok(report) => {
                display_weather("Seoul", report);
                break;
            }
            Err(err) => {
                println!("error querying the weather: {}", err);
                retry_times += 1;
                if retry_times > 3 {
                    println!("Aborting!");
                    break;
                }
            }
        }
    }
}
fn handle_get_weather() {
    let hometown = "Seoul";
    match get_weather(hometown) {
        Ok(report) => display_weather(hometown, report),
        Err(err) => {
            println!("error querying the weather: {}", err);
            schedule_weather_retry()
        }
    }
}

fn handle_get_weather2() {
    const THE_USUAL: WeatherReport = WeatherReport {
        temp: "72".to_string(),
        wind: "5 mph".to_string(),
        humidity: "40%".to_string(),
    };

    let report = get_weather("Seoul").unwrap_or(THE_USUAL);
    display_weather("Seoul", report);
}

use std::fmt;
#[derive(Debug, Clone)]
pub struct JsonError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{} ({}:{})", self.message, self.line, self.column)
    }
}

impl std::error::Error for JsonError {}

fn asdf_json_usage() -> Err(JsonError) {
    return Err(JsonError {
        message: "expected value".to_string(),
        line: 1,
        column: 1,
    });
}

use thiserror::Error;
#[derive(Error, Debug)]
#[error("{message} ({line}:{column})")]
pub struct JsonError2 {
    message: String,
    line: usize,
    column: usize,
}
