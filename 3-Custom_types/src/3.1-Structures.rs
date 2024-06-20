// An attribute to hide warnings for unused code.
#![allow(dead_code)]
#[derive(Copy, Clone)]
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{} is {} years old", peter.name, peter.age);

    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 1.4 };
    let another_point: Point = Point { x: 5.2, y: 0.2 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: another_point.x, y: another_point.y };

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point { x: left_edge, y: top_edge } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 4.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    let _square: Rectangle = square(_rectangle.top_left , 3.0);
    
    rect_area(_rectangle);
    
    println!("The square points are {:?} and {:?}", _square.top_left.x, _square.bottom_right.x);
}

fn rect_area( rect: Rectangle) {
    println!("This is rect_area function");

    // Destructuring the rectangle struct
    
    let Rectangle{top_left: Point{x: x1, y: y1}, bottom_right: Point{x: x2, y: y2}} = rect;
    println!("X1: {} | X2: {} | Y1 {} | Y2 {}", x1, x2, y1, y2);

    let destructuring_area = ((x1-x2)*(y1-y2)).abs();
    println!("Using destructuring: The area of the rectagle is {}",destructuring_area); 
}

fn square (_point: Point , _value: f32) -> Rectangle {
    
    let point2 : Point = { Point { x: _point.x + _value , y: _point.y + _value}};

    let _rect = Rectangle { 
        top_left : _point, 
        bottom_right : point2,
    };
    return _rect;
}