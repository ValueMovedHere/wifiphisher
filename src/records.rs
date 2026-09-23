use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ApRecord {
    #[serde(rename = "BSSID")]
    pub bssid: String,
    #[serde(rename = "First time seen")]
    pub first_time_seen: String,
    #[serde(rename = "Last time seen")]
    pub last_time_seen: String,
    #[serde(rename = "channel")]
    pub channel: Option<i32>,
    #[serde(rename = "Speed")]
    pub speed: Option<i32>,
    #[serde(rename = "Privacy")]
    pub privacy: Option<String>,
    #[serde(rename = "Cipher")]
    pub cipher: Option<String>,
    #[serde(rename = "Authentication")]
    pub authentication: Option<String>,
    #[serde(rename = "Power")]
    pub power: Option<i32>,
    #[serde(rename = "# beacons")]
    pub beacons: Option<i32>,
    #[serde(rename = "# IV")]
    pub iv: Option<i32>,
    #[serde(rename = "LAN IP")]
    pub lan_ip: Option<String>,
    #[serde(rename = "ID-length")]
    pub id_length: Option<i32>,
    #[serde(rename = "ESSID")]
    pub essid: Option<String>,
    #[serde(rename = "Key")]
    pub key: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct StationRecord {
    #[serde(rename = "Station MAC")]
    pub station_mac: String,
    #[serde(rename = "First time seen")]
    pub first_time_seen: String,
    #[serde(rename = "Last time seen")]
    pub last_time_seen: String,
    #[serde(rename = "Power")]
    pub power: Option<i32>,
    #[serde(rename = "# packets")]
    pub packets: Option<i32>,
    #[serde(rename = "BSSID")]
    pub bssid: Option<String>, // 关联的 AP，可能为空
    #[serde(rename = "Probed ESSIDs")]
    pub probed_essids: Option<String>,
}
