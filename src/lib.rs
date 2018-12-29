use std::ops::Mul;

pub type Vec2D = [f64; 2];

fn thicken_point(point: Vec2D, thickness: f64) -> Vec<Vec2D> {
    return vec![
        translate(point, [thickness/2.0, 0.0]),
        translate(point, [0.0, thickness/2.0]),
        translate(point, [-thickness/2.0, 0.0]),
        translate(point, [0.0, -thickness/2.0]),
    ];
}

/// # Panics
///
/// This function panics if thickness is zero
pub fn thicken(line: &[Vec2D], thickness: f64) -> Vec<Vec2D> {
    assert!(thickness != 0.0);

    if line.len() == 0 {
        return Vec::new();
    }

    if line.len() == 1 {
        return thicken_point(line[0], thickness);
    }

    let thickness = thickness.abs();

    let mut result = Vec::with_capacity(line.len()*2);
    let mut invtail = Vec::with_capacity(line.len());

    // add first point
    let (first, last, _) = parallels(line[0], line[1], thickness);

    result.push(first);
    invtail.push(last);

    // add last point
    let (last, first, _) = parallels(line[1], line[0], thickness);
    result.push(first);
    invtail.push(last);

    // empty the invtail stack into the result
    result.extend(invtail.iter().rev());

    result
}

/// Returns two points and a vector that define two parallel lines `thickness`
/// pixels from the line defined by `p1` and `p2`
fn parallels(p1: Vec2D, p2: Vec2D, thickness: f64) -> (Vec2D, Vec2D, Vec2D) {
    let direction = unit_vector(p1, p2);
    let norm = normal(direction);

    (translate(
        p1,
        scale(
            norm,
            thickness/2.0
        )
    ), translate(
        p1,
        scale(
            norm,
            -thickness/2.0
        )
    ), direction)
}

/// Converts a point and a vector to a ax+by=c form
fn to_eq(point: Vec2D, direction: Vec2D) -> [f64; 3] {
    if direction[0] == 0.0 {
        [1.0, 0.0, point[0]]
    } else {
        let m = direction[1]/direction[0];
        let b = point[1] - m * point[0];

        [-m, 1.0, b]
    }
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

fn translate([x1, y1]: Vec2D, [x2, y2]: Vec2D) -> Vec2D {
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
    fn point() {
        let line = vec![
            [0.0, 0.0],
        ];

        assert_eq!(thicken(&line, 2.0), vec![
            [1.0, 0.0],
            [0.0, 1.0],
            [-1.0, 0.0],
            [0.0, -1.0],
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
    fn test_to_eq() {
        assert_eq!(
            to_eq([2.0, 1.0], [0.0, 1.0]),
            [1.0, 0.0, 2.0]
        );

        assert_eq!(
            to_eq([1.0, 3.0], [1.0, 0.0]),
            [0.0, 1.0, 3.0]
        );

        assert_eq!(
            to_eq([3.0, 1.0], [1.0, 1.0]),
            [-1.0, 1.0, -2.0]
        );
    }

    #[test]
    #[ignore]
    fn test_solve22() {
        assert_eq!(solve22(
            [2.0, 1.0], [0.0, 1.0],
            [1.0, 3.0], [1.0, 0.0]
        ), [2.0, 3.0]);
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
