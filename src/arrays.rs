use std::mem;

pub fn run() {
  const N: usize = 5;

  let mut numbers: [i32; N] = [1, 2, 3, 4, 5];

  // Reasign value
  numbers[1] = 123;

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
}