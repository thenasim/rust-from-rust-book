pub fn run() {
  let mut mystr = String::from("hello world");
  let word = first_word(&mystr);

  let _hello = &mystr[0..5];
  let _world = &mystr[6..11];
  println!("Result: {}", word);

  mystr.clear(); // ? string chages but len is still 5
                 // ! this would cause problem if we try to access mystr with invalid length

  // println!("String {}", word); // ! now it will show error
}

// ! bad example
// fn first_word(s: &String) -> usize {
//   let bytes = s.trim().as_bytes();

//   for (i, &item) in bytes.iter().enumerate() {
//     if item == b' ' {
//       return i;
//     }
//   }

//   s.len()
// }

fn first_word(s: &str) -> &str {
  let bytes = s.trim().as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[0..i];
    }
  }

  &s[..]
}
