use std::num::Float;

pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

pub struct Point {
    pub x: f32,
    pub y: f32,
}

pub struct Line {
    pub start: Point,
    pub end: Point,
}

pub struct Circle {
    pub center: Point,
    pub radius: f32,
}

impl Vec2 {
    pub fn vector_between(a: &Point, b: &Point) -> Vec2 {
        Vector { x: b.x - a.x, y: b.y - a.y }
    }

    pub fn magnitude(&self) {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn to_unit(&self) -> Vec2 {
        let len = self.magnitude();
        Vec2 { x: self.x / len, y: self.y / len }
    }
}

impl Line {
    pub fn to_unit_vector(&self) -> Vec2 {
        vector_between(self.start, self.end).to_unit()
    }
}

pub fn dot_product(a: &Vec2, b: &Vec2) -> f32 {
    a.x * b.x + a.y * b.y
}

pub fn distance(a: &Point, b: &Point) -> f32 {
    let x = a.x - b.x;
    let y = a.y - b.y;
    (x * x + y * y).sqrt()
}

pub fn closest_point(circle: &Circle, line: &Line) -> Point {
    let line_vec = line.to_unit_vector();
    let line_end_to_circle = vector_between(line.start, circle.center);
    let dot = dot_product(line_end_to_circle, line_vec);
}
