use std::ops::Mul;

pub type Vec2D = [f64; 2];

/// # Panics
///
/// This function panics if thickness is zero
pub fn thicken(line: &[Vec2D], thickness: f64) -> Vec<Vec2D> {
    assert!(thickness != 0.0);

    let thickness = thickness.abs();

    let result = Vec::with_capacity(line.len()*2);

    for (p1, p2) in line.iter().zip(line.iter().skip(1)) {
        let n = normal(unit_vector(*p1, *p2));

        println!("translated p1: {:?}", add(*p1, scale(n, thickness/2.0)));
    }

    result
}

/// Returns the unit vector that defines this line
fn unit_vector([x1, y1]: Vec2D, [x2, y2]: Vec2D) -> Vec2D {
    let d = d([x1, y1], [x2, y2]);

    [(x2-x1)/d, (y2-y1)/d]
}

fn d([x1, y1]: Vec2D, [x2, y2]: Vec2D) -> f64 {
    ((x2-x1).powi(2) + (y2-y1).powi(2)).sqrt()
}

/// Computes a normal vector
fn normal([x, y]: Vec2D) -> Vec2D {
    [-y, x]
}

fn scale([x, y]: Vec2D, factor: f64) -> Vec2D {
    [x*factor, y*factor]
}

fn add([x1, y1]: Vec2D, [x2, y2]: Vec2D) -> Vec2D {
    [x1+x2, y1+y2]
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON:f64 = 1e-10;

    #[test]
    fn distance() {
        assert!(d([0.0, 0.0], [3.0, 4.0]) - 5.0 < EPSILON);
    }

    #[test]
    fn build_unit_vector() {
        let v = unit_vector([0.0, 0.0], [2.0, 2.0]);

        assert!(v[0] - 0.5_f64.sqrt() < EPSILON);
        assert!(v[1] - 0.5_f64.sqrt() < EPSILON);
    }

    #[test]
    fn rotations() {
        assert_eq!(normal([0.0, 1.0]), [-1.0, 0.0]);
        assert_eq!(normal([-1.0, 0.0]), [0.0, -1.0]);
        assert_eq!(normal([0.0, -1.0]), [1.0, 0.0]);
        assert_eq!(normal([1.0, 0.0]), [0.0, 1.0]);
    }

    #[test]
    fn horizontal() {
        let line = vec![
            [0.0, 0.0],
            [5.0, 0.0],
        ];

        assert_eq!(thicken(&line, 2.0), vec![
            [0.0, 1.0],
            [5.0, 1.0],
            [5.0, -1.0],
            [0.0, -1.0],
        ]);
    }

    #[test]
    #[ignore]
    fn vertical() {
        let line = vec![
            [0.0, 0.0],
            [0.0, 5.0],
        ];

        assert_eq!(thicken(&line, 2.0), vec![
            [-1.0, 0.0],
            [-1.0, 5.0],
            [1.0, 5.0],
            [1.0, 0.0],
        ]);
    }

    #[test]
    #[ignore]
    fn elbow() {
        let line = vec![
            [0.0, 0.0],
            [2.0, 0.0],
            [2.0, 2.0],
        ];

        assert_eq!(thicken(&line, 2.0), vec![
            [0.0, 1.0],
            [1.0, 1.0],
            [1.0, 2.0],
            [3.0, 2.0],
            [3.0, -1.0],
            [0.0, -1.0],
        ]);
    }

    #[test]
    #[ignore]
    fn tilted_elbow() {
        let line = vec![
            [0.0, 0.0],
            [2.0, 2.0],
            [4.0, 0.0],
        ];

        assert_eq!(thicken(&line, 2.0), vec![
            [4.0, 0.0],
        ]);
    }
}
