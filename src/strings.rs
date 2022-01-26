pub fn run() {
  // let hello = "Hello";
  let mut hello = String::from("Hello ");

  // Get length
  println!("Length: {}", hello.len());

  // Push char
  hello.push('W');

  // Push string
  hello.push_str("orld!");

  // Capacity in bytes
  println!("Capacity: {}", hello.capacity());

  println!("Is empty: {}", hello.is_empty());

  println!("Contains 'World': {}", hello.contains("World"));

  println!("Replace: {}", hello.replace("World", "There"));

  // Loop through string by whitespace
  for word in hello.split_whitespace() {
    println!("{}", word);
  }

  if 2 == 3 {
    println!("2 is 2");
  } else if 1 == 1 {
    println!("another");
  }

  println!("{}", hello);
}
