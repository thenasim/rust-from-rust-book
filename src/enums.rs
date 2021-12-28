// enum IpAddrKind {
//   V4,
//   V6,
// }

// enum IpAddrKind {
//   V4(String),
//   V6(String),
// }

#[derive(Debug)]
enum IpAddrKind {
  V4(u8, u8, u8, u8),
  V6(String),
}

enum Message {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(i32, i32, i32),
}

impl Message {
  fn call(&self) {
    println!("call() in Message enum");
  }
}

pub fn run() {
  // let home = IpAddrKind::V4(String::from("127.0.0.1"));
  // let loopback = IpAddrKind::V6(String::from("::1"));

  let home = IpAddrKind::V4(127, 0, 0, 1);
  let loopback = IpAddrKind::V6(String::from("::1"));

  route(loopback);
  println!("V4 Ip is: {:?}", home);

  // ? Message enums
  let msg = Message::Write(String::from("Hello msg"));
  msg.call();

  // ? Options enum
  let some_number = Some(5);
  let absent_number: Option<i32> = None;
}

fn route(_ip: IpAddrKind) {}
