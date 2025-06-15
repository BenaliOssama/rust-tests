#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl Circle {
    pub fn new(a: f64, b: f64, c: f64) -> Self {
        Self {
            center: Point(a, b),
            radius: c,
        }
    }

    pub fn area(&self) -> f64 {
        self.radius * self.radius * std::f64::consts::PI
    }

    pub fn diameter(&self) -> f64 {
        self.radius * 2.
    }

    pub fn intersect(&self, other: Circle) -> bool {
        self.center.distance(other.center) <= self.radius + other.radius
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point(pub f64, pub f64);

impl Point {
    pub fn new(a: f64, b: f64) -> Self {
        Self(a, b)
    }

    pub fn distance(self, other: Point) -> f64 {
        ((self.0 - other.0).abs().powi(2) + (self.1 - other.1).abs().powi(2)).sqrt()
    }
}
