pub fn run() {
  // ? Variables are immutable by default
  let mut pi = 3.1416;
  println!("Pi is {}", pi);
  pi = 3.14159;
  println!("Pi is {}", pi);

  // ? Constants
  const GRAVITATIONAL_CONST: f64 = 6.67408;
  println!("Gravitational constand is {}", GRAVITATIONAL_CONST);

  // ? Shadowing
  let spaces = "    ";
  let spaces = spaces.len();
  println!("Total spaces {}", spaces);
}
