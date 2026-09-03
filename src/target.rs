use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Ap {
    ssid: String,
    bssid: String,
    channel: u8,
    security: Security,
}

#[derive(Debug, Deserialize)]
pub enum Security {
    Open,
    WEP,
    WPA,
    WPA2,
    WPA3,
}
