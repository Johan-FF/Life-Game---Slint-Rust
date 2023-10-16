use csv::ReaderBuilder;
use std::fs;

const FILE_NAME: &str = "types.csv";

fn factorial(number: i32) -> i32 {
  if number==0 || number==1 {
    return 1;
  }
  return number * factorial(number-1); 
}

fn main() {
  let file_content: String = fs::read_to_string(FILE_NAME).unwrap();
  print!("Factorial 12: {}", factorial(4));

  let mut rdr = ReaderBuilder::new().delimiter(b';').from_reader(file_content.as_bytes());

  for result in rdr.records() {
    println!("{:?}", result);
    println!("{:?}", result.unwrap().get(1).unwrap().trim());
  }
}