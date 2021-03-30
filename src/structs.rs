struct User {
  username: String,
  email: String,
  sign_in_count: u32,
  active: bool,
}

pub fn run() {
  let mut user1 = User {
    username: String::from("thenasim"),
    email: String::from("thenasim@gmail.com"),
    sign_in_count: 4,
    active: true,
  };

  user1.email = String::from("nasim@gmail.com");
  println!(
    "Username: {}, SignCount: {}, Active: {}",
    user1.username, user1.sign_in_count, user1.active
  );

  // let user2 = build_user(
  //   String::from("mehedi@gmail.com"),
  //   String::from("mehedihasan"),
  // );

  let user3 = User {
    email: String::from("kola@gmail.com"),
    username: String::from("kola"),
    ..user1
  };

  println!(
    "Username: {}, SignCount: {}, Active: {}",
    user3.username, user3.sign_in_count, user3.active
  );

  tuple_structs();
}

// fn build_user(email: String, username: String) -> User {
//   User {
//     email,    // short hand
//     username, // short hand
//     active: true,
//     sign_in_count: 1,
//   }
// }

fn tuple_structs() {
  struct Color(i32, i32, i32);
  struct Point(i32, i32, i32);

  // ? even though Color and Point has same i32 fields
  // ? they are not the same type

  let white = Color(255, 255, 255);
  let origin = Point(0, 0, 0);

  println!("Origin: {} {} {}", origin.0, origin.1, origin.2);
  println!("White color: {} {} {}", white.0, white.1, white.2);
}
