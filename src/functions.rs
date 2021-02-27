pub fn run() {
  // ! Statement vs Expressions
  // ? Statement is some action but don't return value
  // ? Expression is same as statement but returns value
  // ! Function defination are also statements

  // ? Statement example
  // let y = 34;
  // ? Expression example
  // 5 + 5

  another_function(6);
  println!("Hundred func {}", hundred());
}

// here u32 is return type
fn hundred() -> u32 {
  100
}

fn another_function(x: i32) {
  println!("Value of x is {}", x);
}
