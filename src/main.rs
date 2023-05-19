// write a program to calculate the area of rectangle using struct

#[derive(Debug)]
struct Rectangle {
    length: i32,
    breadth: i32,
}

fn main() {
    let rect1 = Rectangle {
        length: 22,
        breadth: 11,
    };
    let area = calculate_area(rect1);
    println!("the area of rect1 is {:?}", area);
    // println!("Hello, World!");
}

fn calculate_area(rectangle: Rectangle) -> i32 {
    rectangle.breadth * rectangle.length
}
