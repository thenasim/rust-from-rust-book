pub fn run() {
  let mut s1 = String::from("hello");
  let len = calculate_length(&s1);

  change(&mut s1);

  // let _r1 = &mut s1;
  // let _r2 = &mut s1; // ! Error: only one mutable reference to a particular piece of data in a particular scope
  // println!("{}, {}", _r1, _r2); // ! this sloves data races and many problems

  println!("Str: {}, length: {}", s1, len);
  another_example();
}

fn calculate_length(s: &String) -> usize {
  s.len()
}

fn change(s: &mut String) {
  s.push_str(", world");
}

fn another_example() {
  let mut s = String::from("hello");

  let r1 = &s; // no problem
  let r2 = &s; // no problem

  // let r3 = &mut s; // ! Error
  println!("{} and {}", r1, r2);
  // r1 and r2 are no longer used after this point
  // Users of an immutable reference don’t expect the values to suddenly change out from under them!

  let r3 = &mut s; // no problem
  println!("{}", r3);
  // println!("{}", r1); // ! r1 can't be used later
}

// fn dangle() -> &String {
//   let s = String::from("Hello");
// ! reference would be pointing to an invalid String
//   &s
// }
