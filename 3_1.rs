#![allow(dead_code)]
use std::fmt;

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

struct Unit;

struct Pair(i32, f32);

struct Point {
    x: f32,
    y: f32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rect: Rectangle) -> f32 {
    let Rectangle { top_left: Point { x: x1, y: y1 }, bottom_right: Point { x: x2, y: y2 } } = rect;
    ((y2 - y1) * (x2 - x1)).abs()

}

fn square(top_left: Point, edge: f32) -> Rectangle {
    let bottom_right = Point { x: top_left.x + edge, y: top_left.y - edge };
    Rectangle { top_left: top_left, bottom_right: bottom_right }
}

fn main() {
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    println!("{:?}", peter);

    let point: Point = Point { x: 5.2, y: 0.4 };
    let another_point = Point { x: 10.3, y: 0.2 };
    println!("Point cooridnates: ({}, {})", point.x, point.y);

    let bottom_right = Point { x: 10.3, ..another_point };
    println!("Second point: ({}, {})", bottom_right.x, bottom_right.y);

    let copy_all = Point { ..point };
    println!("Copied point: {}", copy_all);

    let copy_last = Point { y: 3.1, ..point };
    println!("Point with copied x: {}", copy_last);

    let Point { x: left_edge, y: top_edge } = point;

    let _rectangle = Rectangle {
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    let _unit = Unit;

    let pair = Pair(1, 0.1);
    println!("Pair containts {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;
    println!("Pair contains {:?} and {:?}", integer, decimal);

    let tl = Point { x: 0.0, y: 5.0 };
    let br = Point { x: 4.0, y: 0.0 };
    let rect = Rectangle { top_left: tl, bottom_right: br };
    println!("Area of the rectangle: {}", rect_area(rect));

    let big_square = square(Point { x: 5.0, y: 6.0 }, 5.0);
    println!("Area of the square: {}", rect_area(big_square));
}
