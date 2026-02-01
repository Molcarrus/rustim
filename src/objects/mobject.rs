use crate::{Color, Point};

pub trait Mobject: Clone + Send + Sync {
    fn center(&self) -> Point;
    
    fn shift(&mut self, delta: (f64, f64));
    
    fn scale(&mut self, factor: f64);
    
    fn rotate(&mut self, angle: f64);
    
    fn set_fill(&mut self, color: Color);
    
    fn set_stroke(&mut self, color: Color, width: f64);
    
    fn to_path(&self) -> Vec<PathCommand>;
    
    fn interpolate(&self, other: &Self, t: f64) -> Self;
}

#[derive(Clone, Debug)]
pub enum PathCommand {
    MoveTo(Point),
    LineTo(Point),
    CurveTo(Point, Point, Point),
    Close,
}