// An attribute to hide warnings for unused code.
#![allow(dead_code)]

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

// It calculates the area of a Rectangle
fn rect_area(r: Rectangle) -> f32 {
    // Destructure the point using a `let` binding
    let Point { x: x1, y: y1 } = r.top_left;
    let Point { x: x2, y: y2 } = r.bottom_right;
    
    let width = (x2 - x1).abs();
    let height = (y2 - y1).abs();
    
    width * height
}

// It calculates the area of a Rectangle
fn rect_area_by_ref(r: &Rectangle) -> f32 {
    // Destructure the point using a `let` binding
    let Point { x: x1, y: y1 } = r.top_left;
    let Point { x: x2, y: y2 } = r.bottom_right;
    
    let width = (x2 - x1).abs();
    let height = (y2 - y1).abs();
    
    width * height
}

fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    // In this case, using regular annotation, check:
    //    https://doc.rust-lang.org/stable/rust-by-example/primitives.html
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, ..point };

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
    
    // New rectangle to calculate its area
    let rectangle2 = Rectangle {
        top_left: Point { x: 5.0, y: 5.0 },
        bottom_right: Point { x: 10.0, y: 0.0 },
    };
    
    println!("rect_area: {}", rect_area(rectangle2));

    // Error! rectable2 was moved before
    // println!("rect_area: {}", rect_area(rectangle2));
    // FIXME ^ Comment out this line
    
    // New rectangle to test the function rect_area_by_ref
    let rectangle3 = Rectangle {
        top_left: Point { x: 1.0, y: 3.0 },
        bottom_right: Point { x: 3.0, y: 1.0 },
    };

    println!("rect_area_by_ref - first call: {}", rect_area_by_ref(&rectangle3));
    println!("rect_area_by_ref - second call: {}", rect_area_by_ref(&rectangle3));

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}

