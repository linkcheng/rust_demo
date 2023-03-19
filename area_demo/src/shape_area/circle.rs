use std::f64::consts::PI;
use std::fmt::Display;
use crate::shape_area::CalculateArea;


#[derive(Debug)]
pub struct Circle<T: Into<f64>+Copy> {
    pub radius: T,
}

impl<T: Into<f64>+Copy+Display> CalculateArea for Circle<T> {
    fn area(&self) -> f64 {
        PI * self.radius.into() * self.radius.into()
    }
}