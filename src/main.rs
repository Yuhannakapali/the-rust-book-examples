// write a program to calculate the area of rectangle using struct

#[derive(Debug)]
struct Rectangle {
    height: i32,
    width: i32,
}

impl Rectangle {
    fn area(&self) -> i32 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.area() >= other.area()
    }

    fn square(size: i32) -> Self {
        Self {
            height: size,
            width: size,
        }
    }
}

// fn calculate_area(rectangle: Rectangle) -> i32 {
//     rectangle.breadth * rectangle.length
// }

fn main() {
    // let rect1 = Rectangle {
    //     length: 22,
    //     breadth: 11,
    // };
    // // let area = calculate_area(rect1);
    // // println!("the area of rect1 is {:?}", area);
    // let area = rect1.area();
    // println!("the area of rect1 is {:?}", area);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    let square = Rectangle::square(22);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Can rect1 hold square? {}", rect1.can_hold(&square));

    // println!("Hello, World!");
}
