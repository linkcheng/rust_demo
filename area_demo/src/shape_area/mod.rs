pub mod circle;
pub mod rectangle;
pub mod triangle;

pub trait CalculateArea {
    fn area(&self) -> f64;
}