use std::num::Float;

#[derive(Clone)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone)]
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
    pub velocity: Vec2,
}

impl Vec2 {
    pub fn between(a: &Point, b: &Point) -> Vec2 {
        Vec2 { x: b.x - a.x, y: b.y - a.y }
    }

    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn to_unit(&self) -> Vec2 {
        let len = self.magnitude();
        Vec2 { x: self.x / len, y: self.y / len }
    }
}

impl Line {
    pub fn to_unit_vector(&self) -> Vec2 {
        Vec2::between(&self.start, &self.end).to_unit()
    }

    pub fn len(&self) -> f32 {
        distance(&self.start, &self.end)
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
    let line_end_to_circle = Vec2::between(&line.start, &circle.center);
    let dot = dot_product(&line_end_to_circle, &line_vec);

    if dot <= 0.0 {
        line.start.clone()
    } else if dot >= line.len() {
        line.end.clone()
    } else {
        Point {
            x: line.start.x + line_vec.x * dot,
            y: line.start.y + line_vec.y * dot,
        }
    }
}

impl Circle {
    fn bounce_vector(&self, line: &Line) -> Vec2 {
        // @todo: make it long enough to cover radius as well
        Vec2::between(&closest_point(self, line), &self.center)
    }

    pub fn bounce_circle(&mut self, line: &Line) {
        let bounce = self.bounce_vector(line);
        let bounce_normal = bounce.to_unit();
        let dot = dot_product(&self.velocity, &bounce_normal);
        self.velocity.x -= 2.0 * dot * bounce_normal.x;
        self.velocity.y -= 2.0 * dot * bounce_normal.y;
    }
}

