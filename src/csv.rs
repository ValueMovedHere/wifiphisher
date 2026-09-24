// 从 airodump-ng 输出的 CSV 中解析与目标相关的信息

use std::{fs::File, io::Seek}
use std::io::{BufReader, Cursor, Read};
use std::path::Path;

use csv;
use serde::Deserialize;

use crate::records::{ApRecord, StationRecord};

fn records(path: &Path) -> (Vec<ApRecord>, Vec<StationRecord>) {
    let file = File::open(path)
        .unwrap();
    file
        .seek(std::io::SeekFrom::Start(1u64)).unwrap();
    let reader = BufReader::new(file);
    let mut file_string = String::new();
    reader.read_to_string(&mut file_string).unwrap();
    let parts: Vec<&str> = file_string.split("\n\n").collect();
    let ap_records_reader = csv::Reader::from_reader(Cursor::new(parts[0].as_bytes()));
    let station_records_reader = csv::Reader::from_reader(Cursor::new(parts[1].as_bytes()));
    (Vec<ApRecord>, Vec<StationRecord>)
}
