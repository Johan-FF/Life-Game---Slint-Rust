fn main() {
  let signed_int: i8 = -42;
  println!("signed_int: {signed_int}");

  let unsigned_int: u8 = 42;
  println!("unsigned_int: {unsigned_int}");

  let simple_float: f32 = 3.14;
  println!("simple_float: {simple_float}");

  let double_float: f64 = 3.14;
  println!("double_float: {double_float}");

  let char: char = '6';
  println!("char: {char}");

  let boolean: bool = true;
  println!("boolean: {boolean}");

  let immutable_string: &str = "Hellow!";
  println!("immutable_string: {immutable_string}");

  let mut changeable_string: String = String::from("Rust");
  changeable_string = changeable_string;
  println!("changeable_string: {changeable_string}");

  let _tuple: (i32, f64, &str) = (42, 3.14, "Rust");
  println!("_tuple: {:?}", _tuple);

  let array: [i32; 5] = [1, 2, 3, 4, 5];
  println!("array: {:?}", array);

  let slice: &[i32] = &array[1..4]; // Contain [2, 3, 4]
  println!("slice: {:?}", slice);

  let optional_value: Option<i32> = Some(42);
  let null_value: Option<i32> = None;
  println!("optional_value: {:?}", optional_value);
  println!("null_value: {:?}", null_value);

  fn split(a: f64, b: f64) -> Result<f64, &'static str> {
    if b != 0.0 {
        Ok(a / b)
    } else {
        Err("División por cero")
    }
  }
  println!("split_function: {:?}", split(12.0,3.0));
}