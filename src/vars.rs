pub fn run() {
  let name = "Oleg";
  let mut age = 24;

  println!("My name is {} and I'm {} y.o.", name, age);
  age = age + 1;
  println!("My name is {} and I'm {} y.o.", name, age);

  // Define constant
  const ID: i32 = 1;
  println!("ID: {}", ID);

  // Assign multiple vars
  let ( my_name, my_age) = ("Oleg", 24);
  println!("{} is {} y.o.", my_name, my_age);
}