pub fn run() {
  let person: (&str, &str, i8) = ("Oleg", "Yar", 24);

  println!("{name} is from {city} and is {age}", name = person.0, age = person.2, city = person.1);
}