fn main() {
    println!("Hello, world!");
}

struct Point {
    x: i32,
    y: i32,
    z: i32
}

impl Point {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    fn calculate_euclidean_distance_between(source: &Self, target: &Self) -> f32 {
        let dx = (target.x - source.x) as f32;
        let dy = (target.y - source.y) as f32;
        let dz = (target.z - source.z) as f32;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn euclidean_distance_is_calculatable() {
        let source = Point::new(1, 2, -5);
        let target = Point::new(4, 6, 7);
        let euclidean_distance = Point::calculate_euclidean_distance_between(&source, &target);
        assert_eq!(13f32, euclidean_distance);
    }
}