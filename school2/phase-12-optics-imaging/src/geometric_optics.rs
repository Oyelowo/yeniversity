//! Geometric optics: thin lens equation, magnification, ray transfer matrix.

/// Thin lens equation: 1/f = 1/do + 1/di
/// Returns image distance di given object distance do and focal length f.
pub fn image_distance(object_dist: f64, focal_length: f64) -> f64 {
    1.0 / (1.0 / focal_length - 1.0 / object_dist)
}

/// Lateral magnification m = -di/do
pub fn magnification(object_dist: f64, image_dist: f64) -> f64 {
    -image_dist / object_dist
}

/// System ray transfer matrix (ABCD matrix) for free propagation over distance d.
pub fn free_space_matrix(d: f64) -> [[f64; 2]; 2] {
    [[1.0, d], [0.0, 1.0]]
}

/// ABCD matrix for a thin lens of focal length f.
pub fn thin_lens_matrix(f: f64) -> [[f64; 2]; 2] {
    [[1.0, 0.0], [-1.0 / f, 1.0]]
}

/// Multiply two 2×2 ray transfer matrices.
pub fn mat_mul(a: [[f64; 2]; 2], b: [[f64; 2]; 2]) -> [[f64; 2]; 2] {
    [
        [a[0][0]*b[0][0]+a[0][1]*b[1][0], a[0][0]*b[0][1]+a[0][1]*b[1][1]],
        [a[1][0]*b[0][0]+a[1][1]*b[1][0], a[1][0]*b[0][1]+a[1][1]*b[1][1]],
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn thin_lens_object_at_2f_gives_image_at_2f() {
        let f = 0.1;
        let di = image_distance(2.0 * f, f);
        assert_abs_diff_eq!(di, 2.0 * f, epsilon = 1e-10);
    }

    #[test]
    fn magnification_at_2f_is_minus_one() {
        let f = 0.1;
        let di = image_distance(2.0 * f, f);
        assert_abs_diff_eq!(magnification(2.0 * f, di), -1.0, epsilon = 1e-10);
    }
}
