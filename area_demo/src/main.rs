mod shape_area;

use std::fmt::Debug;
use shape_area::{circle, triangle, rectangle, CalculateArea};


fn main() {
    let circle = circle::Circle{radius: 1};
    print_area(circle);

    let triangle = triangle::Triangle{base: 1, height: 1.0};
    print_area(triangle);

    let rectangle = rectangle::Rectangle{length: 1, height: 0.5};
    print_area(rectangle);
}

fn print_area<T: CalculateArea + Debug>(shape: T) {
    println!("shape is {:?} area is: {}", shape, shape.area());
}

// mod shape_area {
//     use std::f64::consts::PI;
//     use std::fmt::Display;
//
//     #[derive(Debug)]
//     pub struct Circle<T: Into<f64>+Copy> {
//         pub radius: T,
//     }
//
//     #[derive(Debug)]
//     pub struct Rectangle<T: Into<f64>+Copy+Display, U: Into<f64>+Copy+Display> {
//         pub length: T,
//         pub height: U
//     }
//
//     #[derive(Debug)]
//     pub struct Triangle<T, U>
//         where
//             T: Into<f64>+Copy+Display,
//             U: Into<f64>+Copy+Display
//     {
//         pub base: T,
//         pub height: U
//     }
//
//     pub trait CalculateArea {
//         fn area(&self) -> f64;
//     }
//
//     impl<T: Into<f64>+Copy+Display> CalculateArea for Circle<T> {
//         fn area(&self) -> f64 {
//             PI * self.radius.into() * self.radius.into()
//         }
//     }
//
//     impl<T, U> CalculateArea for Rectangle<T, U>
//         where
//             T: Into<f64>+Copy+Display,
//             U: Into<f64>+Copy+Display
//     {
//         fn area(&self) -> f64 {
//             self.length.into() * self.height.into()
//         }
//     }
//
//     impl<T, U> CalculateArea for Triangle<T, U>
//         where
//             T: Into<f64>+Copy+Display,
//             U: Into<f64>+Copy+Display
//     {
//         fn area(&self) -> f64 {
//             0.5 * self.base.into() * self.height.into()
//         }
//     }
// }