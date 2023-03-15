use std::io::Write;
use std::time::Instant;

use flate2::Compression;
use flate2::write::GzEncoder;

fn main() {
    for file in std::fs::read_dir("/home/sct/Downloads/mails/raw").unwrap() {
        let entry = file.unwrap();
        let path = entry.path();
        println!("===========================================================");
        println!("Start: {}", path.file_name().unwrap().to_str().unwrap());
        let start = Instant::now();
        let data: Vec<u8> = std::fs::read(path).unwrap();
        println!("Read: {}", start.elapsed().as_millis());
        println!("Data len: {}", data.len());
        let start = Instant::now();
        let compressed: Vec<u8> = compress(&data);
        println!("Compress: {}", start.elapsed().as_millis());
        let new_path = format!("/home/sct/Downloads/mails/compressed/{}.tar", entry.file_name().to_str().unwrap());
        let start = Instant::now();
        std::fs::write(new_path, compressed).unwrap();
        println!("Write: {}", start.elapsed().as_millis());
    }

    println!("Ended");
}

fn compress(data: &[u8]) -> Vec<u8> {
    let mut encoder: GzEncoder<Vec<u8>> = GzEncoder::new(Vec::new(), Compression::best());
    encoder.write_all(data).unwrap();
    encoder.finish().unwrap()
}