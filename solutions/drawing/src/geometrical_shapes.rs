use raster::{Image, Color};
use rand::Rng;

pub trait Drawable {
    fn draw(&self, image: &mut Image);
    fn color(&self) -> Color;
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

/*------------------ Point ------------------*/
#[derive(Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0..width);
        let y = rng.gen_range(0..height);
        Self { x, y }
    }
}

impl Drawable for Point {
    fn draw(&self, image: &mut Image) {
        if self.x >= 0 && self.x < image.width && self.y >= 0 && self.y < image.height {
            image.display(self.x, self.y, self.color());
        }
    }

    fn color(&self) -> Color {
        Color::rgb(255, 0, 0)
    }
}

/*______________________________Line______________________________*/
pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Line {
    pub fn new(start: &Point, end: &Point) -> Self {
        Self {
            start: start.clone(),
            end: end.clone(),
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let p1 = Point::random(width, height);
        let p2 = Point::random(width, height);
        Self::new(&p1, &p2)
    }
}

impl Drawable for Line {
    fn draw(&self, image: &mut Image) {
        let dx = (self.end.x - self.start.x).abs();
        let dy = (self.end.y - self.start.y).abs();

        let sx = if self.start.x < self.end.x { 1 } else { -1 };
        let sy = if self.start.y < self.end.y { 1 } else { -1 };

        let mut err = dx - dy;
        let mut x = self.start.x;
        let mut y = self.start.y;

        loop {
            image.display(x, y, self.color());
            if x == self.end.x && y == self.end.y {
                break;
            }
            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x += sx;
            }
            if e2 < dx {
                err += dx;
                y += sy;
            }
        }
    }

    fn color(&self) -> Color {
        Color::rgb(0, 255, 0)
    }
}

/*______________________________Triangel______________________________*/

pub struct Triangle {
    pub first: Point,
    pub second: Point,
    pub third: Point,
}

impl Triangle {
    pub fn new(first: &Point, second: &Point, third: &Point) -> Self {
        Self {
            first: first.clone(),
            second: second.clone(),
            third: third.clone(),
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let p1 = Point::random(width, height);
        let p2 = Point::random(width, height);
        let p3 = Point::random(width, height);
        Self::new(&p1, &p2, &p3)
    }
}

impl Drawable for Triangle {
    fn draw(&self, image: &mut Image) {
        let l1 = Line::new(&self.first, &self.second);
        let l2 = Line::new(&self.second, &self.third);
        let l3 = Line::new(&self.third, &self.first);

        l1.draw(image);
        l2.draw(image);
        l3.draw(image);
    }

    fn color(&self) -> Color {
        Color::rgb(0, 0, 255)
    }
}

/*______________________________Rectangle____________________________________*/

pub struct Rectangle {
    pub first: Point,  // Top-left
    pub second: Point, // Top-right
    pub third: Point,  // Bottom-right
    pub forth: Point,  // Bottom-left
}

impl Rectangle {
    pub fn new(first: &Point, third: &Point) -> Self {
        // Assume first = top-left, third = bottom-right
        let second = Point::new(third.x, first.y);
        let forth = Point::new(first.x, third.y);

        Self {
            first: first.clone(),
            second,
            third: third.clone(),
            forth,
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let p1 = Point::random(width, height);
        let p2 = Point::random(width, height);

        // Make sure p1 is top-left, p2 is bottom-right
        let top_left = Point::new(p1.x.min(p2.x), p1.y.min(p2.y));
        let bottom_right = Point::new(p1.x.max(p2.x), p1.y.max(p2.y));

        Self::new(&top_left, &bottom_right)
    }
}

impl Drawable for Rectangle {
    fn draw(&self, image: &mut Image) {
        let l1 = Line::new(&self.first, &self.second);
        let l2 = Line::new(&self.second, &self.third);
        let l3 = Line::new(&self.third, &self.forth);
        let l4 = Line::new(&self.forth, &self.first);

        l1.draw(image);
        l2.draw(image);
        l3.draw(image);
        l4.draw(image);
    }

    fn color(&self) -> Color {
        Color::rgb(255, 0, 255)
    }
}

/*______________________________Circle____________________________________*/



pub struct Circle {
    pub center: Point,
    pub radius: i32,
    pub color: Color,
}

impl Circle {
    pub fn new(center: Point, radius: i32, color: Color) -> Self {
        Self {
            center,
            radius,
            color,
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let center = Point::random(width, height);
        let mut rng = rand::thread_rng();
        let radius = rng.gen_range(1..=width.min(height) / 4);
        let color = Circle::random_color(&mut rng);

        Self::new(center, radius, color)
    }

    fn random_color(rng: &mut impl Rng) -> Color {
        Color::rgb(
            rng.gen_range(0..=255),
            rng.gen_range(0..=255),
            rng.gen_range(0..=255),
        )
    }
}

impl Drawable for Circle {
    fn draw(&self, img: &mut Image) {
        let c = &self.center;
        let r = self.radius;

        for j in -r..=r {
            for i in -r..=r {
                let dist_sq = i * i + j * j;
                if dist_sq >= r * r - r && dist_sq <= r * r + r {
                    let x = c.x + i;
                    let y = c.y + j;

                    if x >= 0 && x < img.width && y >= 0 && y < img.height {
                        img.set_pixel(x, y, self.color.clone()).unwrap();
                    }
                }
            }
        }
    }

    fn color(&self) -> Color {
        self.color.clone()
    }
}

