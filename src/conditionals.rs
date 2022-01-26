pub fn run() {
  // == != >= <= > < && ||

  let age = 17;

  if age != 18 {
    println!("hello");
  };

  // Ternary
  let is_of_age = if age >= 21 { true } else { false };
  println!("{}", is_of_age);
}
