use std::io;

pub fn run() {
  println!("Generate fibonacci numbers!");
  println!("Enter number of terms - ");

  let mut upto = String::new();

  io::stdin()
    .read_line(&mut upto)
    .expect("Not a valid string");

  let upto: u32 = upto.trim().parse().expect("Can't parse the string!");

  let mut v1: u32 = 0;
  let mut v2: u32 = 1;
  let mut next_term: u32;

  for _ in 0..upto {
    print!("{} ", v1);
    next_term = v1 + v2;
    v1 = v2;
    v2 = next_term;
  }
}
