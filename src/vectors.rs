
fn main() {
  let mut names: Vec<String> = Vec::new();

  for _ in 0..3 {
    println!("Please enter your name: ");
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).unwrap();
    names.push(name);
  }

  println!("{:?}", names);
  println!("{}", names[0]);

  for name in names {
    println!("Name: {}", name);
  }

  let hellow: [&str; 6] = ["H", "E", "L", "L", "O", "W"];
  println!("{}",hellow[0]);
  println!("{}",hellow[1]);
  println!("{}",hellow[2]);
  println!("{}",hellow[3]);
  println!("{}",hellow[4]);
  println!("{}",hellow[5]);
}