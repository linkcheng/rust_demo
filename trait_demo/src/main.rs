use std::cmp::PartialOrd;
use std::fmt::Display;

fn main() {
    println!("Hello, world!");

    let nums = vec![13, 22, 134, 41, 55];
    let result = largest(&nums);
    println!("largest = {}", result);

    let p = Point { x: 5, y: 1.0 };
    // p.summarize();
    display(&p);

    let str1 = "rust";
    let str2 = "python";
    let result = longest(str1, str2);
    println!("longest = {}", result);
}

fn _largest(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &i in list {
        if i > largest {
            largest = i;
        }
    }
    largest
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &i in list {
        if i > largest {
            largest = i;
        }
    }
    largest
}

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

trait Summary {
    fn summarize(&self);
}

impl<T: Display, U: Display> Summary for Point<T, U> {
    fn summarize(&self) {
        println!("x={}, y={}", self.x, self.y);
    }
}

fn _display1(item: &impl Summary) {
    item.summarize();
}

fn display<T: Summary>(item: &T) {
    item.summarize();
}

fn _notify<T>(item: &T)
where
    T: Summary,
{
    item.summarize();
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
