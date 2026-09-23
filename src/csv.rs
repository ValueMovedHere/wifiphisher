// 从 airodump-ng 输出的 CSV 中解析与目标相关的信息

use std::{fs::File, io::Seek}
use std::io::BufReader;
use std::path::Path;

use crate::records::{ApRecord, StationRecord};

fn records(path: &Path) -> (Vec<ApRecord>, Vec<StationRecord>) {
    let file = File::open(path)
        .unwrap();
        file
        .seek(std::io::SeekFrom::Start(1u64)).unwrap();
    let reader = BufReader::new(file);
    (Vec<ApRecord>, Vec<StationRecord>)
}
