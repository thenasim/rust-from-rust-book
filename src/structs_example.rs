#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  // ? Associated Functions (does not have a &self param) call with ::
  fn square(size: u32) -> Rectangle {
    Rectangle {
      width: size,
      height: size,
    }
  }
  fn area(&self) -> u32 {
    self.width * self.height
  }
  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }
}

pub fn run() {
  let rect1 = Rectangle {
    width: 30,
    height: 50,
  };

  let rect2 = Rectangle {
    width: 29,
    height: 21,
  };

  let rect3 = Rectangle {
    width: 62,
    height: 77,
  };

  println!("The rect is {:?}", rect1);
  println!("The rect is {:#?}", rect1);
  println!(
    "The area of the rectangle is {} square pixels.",
    area(&rect1)
  );
  println!("The area is {}", rect1.area());
  println!("Can rect1 hold rect2 {}", rect1.can_hold(&rect2));
  println!("Can rect1 hold rect2 {}", rect1.can_hold(&rect3));

  let square1 = Rectangle::square(20);

  println!("The square is {:?}", square1);
}

fn area(rectangle: &Rectangle) -> u32 {
  rectangle.width * rectangle.height
}
