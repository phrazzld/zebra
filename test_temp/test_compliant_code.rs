// This file contains properly formatted and lint-free code for testing pre-commit hooks

/// A simple structure to demonstrate compliant code
pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    /// Creates a new Point with the given coordinates
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Calculates the distance from the origin (0, 0)
    pub fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    /// Calculates the distance between two points
    pub fn distance(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx.powi(2) + dy.powi(2)).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_from_origin() {
        let p = Point::new(3.0, 4.0);
        assert_eq!(p.distance_from_origin(), 5.0);
    }

    #[test]
    fn test_distance_between_points() {
        let p1 = Point::new(0.0, 0.0);
        let p2 = Point::new(3.0, 4.0);
        assert_eq!(p1.distance(&p2), 5.0);
    }
}
