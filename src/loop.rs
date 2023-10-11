fn main() {
  let mut age_int: u8;
  loop{
    println!("Please enter your age: ");
    let mut age: String = String::new();
    std::io::stdin().read_line(&mut age).unwrap();
  
    age_int = age.trim().parse().unwrap();
  
    if age_int >= 18 {
      print!("You may come in");
      break;
    } else {
      print!("You may not come in");
    }
  }
  println!("You are {} years old", age_int);
}