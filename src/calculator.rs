use regex::Regex;

fn main() {
  let re_add: Regex = Regex::new(r"(-?\d+)\s?\+\s?(-?\d+)").unwrap();
  let re_rest: Regex = Regex::new(r"(-?\d+)\s?\-\s?(-?\d+)").unwrap();
  let re_mult: Regex = Regex::new(r"(-?\d+)\s?\*\s?(-?\d+)").unwrap();
  let re_div: Regex = Regex::new(r"(-?\d+)\s?\/\s?(-?\d+)").unwrap();
  let re_rais: Regex = Regex::new(r"(-?\d+)\s?\^\s?(-?\d+)").unwrap();

  println!("Please enter your expression: ");
  let mut expression: String = String::new();
  std::io::stdin().read_line(&mut expression).unwrap();

  let mut caps;
  let mut left_value: f64;
  let mut right_value: f64;
  let mut result: f64; 
  let mut cap_expression: &str;
  let mut operation: &str = "rais";
  loop{
    if operation=="rais"{
      caps = re_rais.captures(expression.as_str());
    } else if operation=="mult"{
      caps = re_mult.captures(expression.as_str());
    } else if operation=="div" {
      caps = re_div.captures(expression.as_str());
    } else if operation=="add" {
      caps = re_add.captures(expression.as_str());
    } else if operation=="rest" {
      caps = re_rest.captures(expression.as_str());
    } else {
      break;
    }

    if caps.is_none() {
      if operation=="rais"{
        operation = "mult";
      } else if operation=="mult"{
        operation = "div";
      } else if operation=="div" {
        operation = "add";
      } else if operation=="add" {
        operation = "rest";
      } else if operation=="rest" {
        operation = "end";
      }
      continue;
    }
    let caps = caps.unwrap();

    cap_expression = caps.get(0).unwrap().as_str();
    left_value = caps.get(1).unwrap().as_str().parse().unwrap();
    right_value = caps.get(2).unwrap().as_str().parse().unwrap();

    if operation=="rais"{
      if right_value<0.0 {
        result = left_value.powf(1.0/(right_value*-1.0));
      } else {
        result = left_value.powf(right_value);
      }
    } else if operation=="mult"{
      result = left_value * right_value;
    } else if operation=="div" {
      result = left_value / right_value;
    } else if operation=="add" {
      result = left_value + right_value;
    } else {
      result = left_value - right_value;
    }
    expression = expression.replace(cap_expression, &result.to_string());
  }

  print!("Result: {}", expression);
}