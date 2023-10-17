use csv::{Reader,ReaderBuilder};
use std::fs;

const FILE_NAME: &str = "types.csv";

fn print_file(mut rdr: Reader<&[u8]>) {
  for result in rdr.records() {
    println!("{:?}", result);
    println!("{:?}", result.unwrap().get(1).unwrap().trim());
  }
}

fn main() {
  let file_content: String = fs::read_to_string(FILE_NAME).unwrap();

  let rdr: Reader<&[u8]> = ReaderBuilder::new().delimiter(b';').from_reader(file_content.as_bytes());

  print_file(rdr);
}