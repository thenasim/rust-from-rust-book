pub fn run() {
  // ! If/Els
  let age = 32;

  if age < 18 {
    println!("You are a kid");
  } else {
    println!("Adult");
  }

  let number = 3;

  if number != 0 {
    println!("Number is not 0");
  }

  if number % 4 == 0 {
    println!("number is divisible by 4");
  } else if number % 3 == 0 {
    println!("number is divisible by 3");
  } else if number % 2 == 0 {
    println!("number is divisible by 2");
  } else {
    println!("number is not divisible by 4, 3, or 2");
  }

  // ? Using if in a let Statement
  let condition = true;
  let number = if condition { 5 } else { 50 };
  // let number = if condition { 5 } else { "six" }; // ! Error - if and else must return same type
  println!("The value of number is: {}", number);

  // ! Loop
  // ? 3 types of loop - while, for, loop

  // ? infinite loop
  loop {
    println!("again!");
    break;
  }

  // ? Returning value from loop
  let mut counter = 0;

  let result = loop {
    counter += 1;
    if counter == 5 {
      break counter * 2; // ? sets the value to result
    }
  };

  println!("The result is {}", result);

  // ? While loop
  let mut number = 3;

  while number != 0 {
    println!("{}!", number);

    number -= 1;
  }

  let a = [10, 20, 30, 40, 50];
  let mut index = 0;

  // ? This code adds runtime
  while index < 5 {
    println!("the value is: {}", a[index]);

    index += 1;
  }

  // ? For loop
  let a = [10, 20, 30, 40, 50];

  for element in a.iter() {
    println!("the value is: {}", element);
  }

  // * we could also use 1..4
  for number in (1..4).rev() {
    println!("{}!", number);
  }
  println!("LIFTOFF!!!");
}
