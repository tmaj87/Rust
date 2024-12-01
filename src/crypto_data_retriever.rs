use std::time::{SystemTime, UNIX_EPOCH};
use reqwest::blocking::get;

fn get_data() -> () {
    let x_days = 24 * 60 * 60;
    let symbol_binance = "TIAUSDT";
    let interval = "1d";
    let end_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let start_time = end_time - (30 * x_days);
    let url = format!("https://api.binance.com/api/v3/klines?symbol={symbol_binance}&interval={interval}&startTime={start_time}&endTime={end_time}");
    let response = get(url);
    response.unwrap().text().unwrap();
}

fn profitability_score() {
    // ...
}

pub fn analyse() {
    let result = get_data();
    println!("Result: {:?}", result);
}