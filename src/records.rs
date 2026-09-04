use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ApRecord {}

#[derive(Debug, Deserialize)]
pub struct StationRecord {}

#[derive(Debug, Deserialize)]
pub enum Security {
    Open,
    WEP,
    WPA,
    WPA2,
    WPA3,
}
