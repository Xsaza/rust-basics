pub fn run() {
  greeting("Hello", "Oleg");
  println!("add fn: {}", add(1, 2));

  // Closure
  let n3: i32 = 10;
  // let c_add = |n1: i32, n2: i32| {
  //   return n1 + n2 + n3;
  // }; // eq
  // let c_add = |n1: i32, n2: i32| { n1 + n2 + n3 }; // eq
  let c_add = |n1: i32, n2: i32| n1 + n2 + n3;
  println!("c add fn: {}", c_add(2, 3));
}

fn greeting(greet: &str, name: &str) {
  println!("{} {}, nice to meet you", greet, name);
}

// fn add(n1: i32, n2: i32) -> i32 {
//   return n1 + n2;
// } //eq
fn add(n1: i32, n2: i32) -> i32 { n1 + n2 }