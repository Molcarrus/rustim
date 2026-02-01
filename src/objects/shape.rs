use crate::{Color, Point, objects::mobject::{Mobject, PathCommand}};

#[derive(Clone)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
    pub fill: Option<Color>,
    pub stroke: Option<(Color, f64)>,
}

impl Circle {
    pub fn new(center: Point, radius: f64) -> Self {
        Self { 
            center, 
            radius, 
            fill: None, 
            stroke: Some((Color::new(1.0, 1.0, 1.0, 1.0), 2.0)),
        }
    }
}

impl Mobject for Circle {
    fn center(&self) -> Point {
        self.center
    }
    
    fn shift(&mut self, delta: (f64, f64)) {
        self.center.x += delta.0;
        self.center.y += delta.1;
    }
    
    fn scale(&mut self, factor: f64) {
        self.radius *= factor;
    }
    
    fn rotate(&mut self, _angle: f64) {
        // circle is rotationally symmetric
    }
    
    fn set_fill(&mut self, color: Color) {
        self.fill = Some(color);
    }
    
    fn set_stroke(&mut self, color: Color, width: f64) {
        self.stroke = Some((color, width));
    }
    
    fn to_path(&self) -> Vec<super::mobject::PathCommand> {
        let k = 0.552284749831;
        let (r, c) = (self.radius, self.center);
        
        vec![
            PathCommand::MoveTo(Point::new(c.x + r, c.y)),
            PathCommand::CurveTo(
                Point::new(c.x + r, c.y + r * k), 
                Point::new(c.x + r * k, c.y + r),
                Point::new(c.x, c.y + r) 
            ),
            PathCommand::Close,
        ]
    }
    
    fn interpolate(&self, other: &Self, t: f64) -> Self {
        Self { 
            center: Point::new(
                self.center.x + (other.center.x - self.center.x) * t, 
                self.center.y + (other.center.y - self.center.y) * t), 
            radius: self.radius + (other.radius - self.radius) * t, 
            fill: self.fill, 
            stroke: self.stroke, 
        }
    }
}