use winit::dpi::{LogicalPosition, PhysicalPosition};

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Point2D {
    x: f32,
    y: f32,
}

impl From<Point2D> for iced_native::Point {
    fn from(point: Point2D) -> Self {
        Self {
            x: point.x,
            y: point.y,
        }
    }
}

impl From<PhysicalPosition<f64>> for Point2D {
    fn from(point: PhysicalPosition<f64>) -> Self {
        Self {
            x: point.x as f32,
            y: point.y as f32,
        }
    }
}

impl From<LogicalPosition<f64>> for Point2D {
    fn from(point: LogicalPosition<f64>) -> Self {
        Self {
            x: point.x as f32,
            y: point.y as f32,
        }
    }
}
