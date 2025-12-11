fn main() {
    println!("Hello, world!");
}

#[derive(PartialEq)]
struct Point {
    x: i32,
    y: i32,
    z: i32
}

impl Point {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    fn calculate_euclidean_distance_between(x: &Self, y: &Self) -> f32 {
        let dx = (y.x - x.x) as f32;
        let dy = (y.y - x.y) as f32;
        let dz = (y.z - x.z) as f32;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    fn find_closest_pairing(points: &[Self]) -> Option<(&Self, &Self)> {
        if points.len() < 2 {
            return None;
        }

        let mut closest_distance = f32::MAX;
        let mut closest_pair = (&points[0], &points[1]);
        
        for i in 0..points.len() - 1 {
            for j in i + 1..points.len() {
                let distance = Point::calculate_euclidean_distance_between(&points[i], &points[j]);
                if distance < closest_distance {
                    closest_distance = distance;
                    closest_pair = (&points[i], &points[j]);
                }
            }
        }
        
        Some(closest_pair)
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

    #[test]
    fn closest_points_are_found() {
        let example_data = [
            Point::new(162,817,812),
            Point::new(57,618,57),
            Point::new(906,360,560),
            Point::new(592,479,940),
            Point::new(352,342,300),
            Point::new(466,668,158),
            Point::new(542,29,236),
            Point::new(431,825,988),
            Point::new(739,650,466),
            Point::new(52,470,668),
            Point::new(216,146,977),
            Point::new(819,987,18),
            Point::new(117,168,530),
            Point::new(805,96,715),
            Point::new(346,949,466),
            Point::new(970,615,88),
            Point::new(941,993,340),
            Point::new(862,61,35),
            Point::new(984,92,344),
            Point::new(425,690,689),
        ];

        let closet_points = Point::find_closest_pairing(&example_data);

        assert!(closet_points.is_some(), "Expected a pair of closest points to be found.");

        let (p1, p2) = closet_points.unwrap();
        let expected_closest_points = [
            Point::new(162,817,812),
            Point::new(425,690,689),
        ];

        assert!(expected_closest_points.contains(&p1));
        assert!(expected_closest_points.contains(&p2));
    }
}