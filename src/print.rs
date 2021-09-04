pub fn run() {
  println!("Hello from print.rs file");

  // Basic formatting
  println!("Number: {} {}", 1, 3);

  // Positional arguments
  println!("{0} is from {1} and {0} likes to {2}", "Brad", "Mass", "code");

  //Named arguments
  println!("{name} likes to play {activity}", name = "John", activity = "Basecball");

  // Placeholder traits
  println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

  // Palceholder for debug trait
  println!("{:?}", (12, true, "hello"));

  // Basic math
  println!("10 + 2 = {result}", result = 10 + 2);
}