use std::fmt::Display;
use crate::shape_area::CalculateArea;

#[derive(Debug)]
pub struct Triangle<T, U>
    where
        T: Into<f64>+Copy+Display,
        U: Into<f64>+Copy+Display
{
    pub base: T,
    pub height: U
}

impl<T, U> CalculateArea for Triangle<T, U>
    where
        T: Into<f64>+Copy+Display,
        U: Into<f64>+Copy+Display
{
    fn area(&self) -> f64 {
        0.5 * self.base.into() * self.height.into()
    }
}