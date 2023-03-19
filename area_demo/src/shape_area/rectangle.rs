use std::fmt::Display;
use crate::shape_area::CalculateArea;

#[derive(Debug)]
pub struct Rectangle<T: Into<f64>+Copy+Display, U: Into<f64>+Copy+Display> {
    pub length: T,
    pub height: U
}

impl<T, U> CalculateArea for Rectangle<T, U>
    where
        T: Into<f64>+Copy+Display,
        U: Into<f64>+Copy+Display
{
    fn area(&self) -> f64 {
        self.length.into() * self.height.into()
    }
}
