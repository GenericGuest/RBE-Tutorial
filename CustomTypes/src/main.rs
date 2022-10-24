// Three types of structures: tuples, c structs, unit structs

// Attribute - hides unused code warnings
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// Unit struct
struct Unit;

// Tuple Struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Can use previous structs as fields
struct Rectangle {
    top_left: Point,
    bottom_right: Point
}

impl Rectangle {
    fn square(topleft: Point, size: f32) -> Rectangle {
        Rectangle {
            top_left: Point {..topleft },
            bottom_right: Point{ x: topleft.x + size, y: topleft.y + size }
        }
    }
}

fn rect_area(rect: Rectangle) -> f32 {
    let Rectangle { top_left: tl, bottom_right: br} = rect;
    (br.x- tl.x)*(tl.y - br.y)
}

fn main() {
    let name = String::from("Peter");
    let age = 27;
    let peter = Person {name, age};
    println!("{:?}", peter);

    let point: Point = Point {x: 10.3, y: 0.5 };
    println!("Coords: ({}, {})", point.x, point.y);
    

    let bottom_right = Point {x: 5.2, ..point};

    // Destructuring
    let Point {x: left_edge, y: top_edge } = point;

    let _rectangle = Rectangle {
        top_left: Point {x: left_edge, y: top_edge},
        bottom_right: bottom_right,
    };

    let _unit = Unit; // ?????

    let pair = Pair(1, 0.1);

    println!("Pair: {:?} and {:?}", pair.0, pair.1);
    

    // destructure tuple
    let Pair(integer, decimal) = pair;

    println!("Pair contains {:?} and {:?}", integer,decimal);

    println!("Area: {:?}", rect_area(_rectangle));

}
