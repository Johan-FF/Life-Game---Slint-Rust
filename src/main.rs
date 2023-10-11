use regex::Regex;

fn main() {
  let re_add: Regex = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();
  let re_mult: Regex = Regex::new(r"(\d+)\s?\*\s?(\d+)").unwrap();

  println!("Please enter your expression: ");
  let mut expression: String = String::new();
  std::io::stdin().read_line(&mut expression).unwrap();

  let mut caps;
  let mut left_value: i32;
  let mut right_value: i32;
  let mut result: i32; 
  let mut cap_expression: &str;
  loop{
    caps = re_mult.captures(expression.as_str());
    if caps.is_none() {
      break;
    }
    let caps = caps.unwrap();

    cap_expression = caps.get(0).unwrap().as_str();
    left_value = caps.get(1).unwrap().as_str().parse().unwrap();
    right_value = caps.get(2).unwrap().as_str().parse().unwrap();
    result = left_value * right_value;
    expression = expression.replace(cap_expression, &result.to_string());
  }
  loop{
    caps = re_add.captures(expression.as_str());
    if caps.is_none() {
      break;
    }
    let caps = caps.unwrap();

    cap_expression = caps.get(0).unwrap().as_str();
    left_value = caps.get(1).unwrap().as_str().parse().unwrap();
    right_value = caps.get(2).unwrap().as_str().parse().unwrap();
    result = left_value + right_value;
    expression = expression.replace(cap_expression, &result.to_string());
  }

  print!("Result: {}", expression);
}