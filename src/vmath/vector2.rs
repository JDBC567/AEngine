#![allow(dead_code)]

use crate::vmath::utils::Print;
use std::fmt::Formatter;

#[derive(Copy, Clone)]
pub struct Vector2<T: std::marker::Copy + std::clone::Clone> {
    pub x: T,
    pub y: T,
}

impl<T: std::marker::Copy + std::clone::Clone> Vector2<T> {
    pub fn new(x_: T, y_: T) -> Vector2<T> {
        Vector2 { x: x_, y: y_ }
    }
}

impl<T: std::fmt::Display + std::marker::Copy + std::clone::Clone> std::fmt::Display
    for Vector2<T>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.x, self.y)
    }
}

impl<T: std::marker::Copy + std::clone::Clone> Print for Vector2<T> {}
