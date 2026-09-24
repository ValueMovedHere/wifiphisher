// 从 airodump-ng 输出的 CSV 中解析与目标相关的信息

use std::io::{BufReader, Cursor, Read};
use std::path::Path;
use std::{fs::File, io::Seek};

use csv;

use crate::records::{ApRecord, StationRecord};

pub fn records(path: &Path) -> (Vec<ApRecord>, Vec<StationRecord>) {
    let mut file = File::open(path).unwrap();
    file.seek(std::io::SeekFrom::Start(2u64)).unwrap();
    let mut reader = BufReader::new(file);
    let mut file_string = String::new();
    reader.read_to_string(&mut file_string).unwrap();
    let parts: Vec<&str> = file_string.split("\r\n\r\n").collect();
    let mut ap_records_reader = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .from_reader(Cursor::new(parts[0].as_bytes()));
    println!("{:#?}", parts[0]);
    let mut station_records_reader = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .from_reader(Cursor::new(parts[1].as_bytes()));
    let mut ap_record_vec = Vec::with_capacity(5usize);
    for result in ap_records_reader.deserialize() {
        let ap_record = result.unwrap();
        ap_record_vec.push(ap_record);
    }
    println!("=============Parsing station record data==============");
    let mut station_records_vec = Vec::with_capacity(5usize);
    for result in station_records_reader.deserialize() {
        let station_record = result.unwrap();
        station_records_vec.push(station_record);
    }
    (ap_record_vec, station_records_vec)
}
