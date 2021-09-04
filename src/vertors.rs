use std::mem;

pub fn run() {
  let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

  // Reasign value
  numbers[1] = 10;

  // Add on to vector
  numbers.push(6);
  numbers.push(7);

  // Pop off last value
  numbers.pop();

  println!("{:?}", numbers);

  // Get value
  println!("Single value: {}", numbers[0]);

  // Get length
  println!("Length: {}", numbers.len());

  // Arrays are stack allocated
  println!("Array occupies {} bytes", mem::size_of_val(&numbers));

  // Get slice
  let slice: &[i32] = &numbers[0..2];
  println!("Slice {:?}", slice);

  for x in numbers.iter() {
    println!("Number: {}", x);
  }

  // Loop and mutate values
  for x in numbers.iter_mut() {
    *x *= 2;
  }

  println!("{:?}", numbers);
}