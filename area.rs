
pub trait GetArea<T> {
    fn getArea(&self) -> T;
}

pub Struct Triangle<T> {
    x: T,
    y: T,
}

impl GetArea<T> for Triangle<T> {
    fn getArea(&self) -> T {
        (self.x * self.y) / 2
    }
}

pub Struct Rectangle<T> {
    x: T,
    y: T,
}

impl GetArea<T> for Rectangle<T> {
    fn getArea(&self) -> T {
        self.x * self.y
    }
}

pub fn getItemArea(item<T>: impl GetArea) -> T{
    item.getArea()
}