/**
 * Scalar Types
 * - integer, floating point, booleans, characters
*/

pub fn run() {
  // ? Integer type
  let x = 3_444; // default is u32
  let y: u8 = 34; // explicit data type
  let z = -21i32; // explicit type at the end
  println!("{} {} {}", x, y, z);

  // ? Boolean type
  let isnoob: bool = false;
  let isnoob: bool = !isnoob;
  println!("Are you a noob coder? {}", isnoob);

  // ? Character type
  // Same as other language but it also can store UTF-8 char

  // ! Compound type - tuple, array
  // ? Tuple (fixed size) (support mutiple types)
  let tup1 = ('x', "hello", 1.9); // implicit
  let tup2: (i16, char, f32) = (23, 'a', 3.3); // explicit
  println!("{:?} {:?}", tup1, tup2);

  // ? Tuple destructure
  let (a, b, c) = tup1;
  let mychar = tup2.1; // access with array like index with dots
  println!("{} {} {} {}", a, b, c, mychar);

  // ? Array (fixed size) (similar type)
  let months = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
  ];
  println!("Months {:?}", months);

  // ?  [type; length] - fixed size
  let e: [i32; 3] = [1, 2, 4];
  println!("{:?}", e);

  // ? Array repeat same value
  let m = [3; 2]; // [value; length] = [3, 3]
  println!("{:?}", m);

  // ? Accessing array elements
  println!("Values {} {} {}", e[0], e[1], e[2]);
}
