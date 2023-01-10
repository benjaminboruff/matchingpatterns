#![allow(unused)]

fn main() {
    // let mut stack = Vec::new();

    // stack.push(1);
    // stack.push(2);
    // stack.push(3);

    // while let Some(top) = stack.pop() {
    //     println!("{}", top);
    // }

    // let v = vec!['a', 'b', 'c'];

    // for (index, value) in v.iter().enumerate() {
    //     println!("{} is at index {}", value, index);
    // }

    // let (a, b, c, ..) = (1, 2, 3, 4, 5, 6);
    // let point = (3, 5);
    // print_coordinates(&point);
    // let opt_val: Option<i32> = Some(12);
    // if let x = 5 {};

    // let p = Point { x: 1, y: 3 };

    // match p {
    //     Point { x, y: 0 } => println!("On the x axis at {}", x),
    //     Point { x: 0, y } => println!("On the y axis at {}", y),
    //     Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    // }

    let s = Some(String::from("hello"));

    if let Some(_) = s {
        println!("Found a string!");
    }

    println!("{:?}", s);
}

struct Point {
    x: i32,
    y: i32,
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}
