//! Fourier optics: 2-D convolution (image formation), Airy disc radius.

/// Airy disc first zero radius at image plane: r = 1.22 λ f/# 
pub fn airy_disc_radius(wavelength: f64, f_number: f64) -> f64 {
    1.22 * wavelength * f_number
}

/// Rayleigh resolution criterion: minimum resolvable separation = Airy disc radius.
pub fn rayleigh_resolution(wavelength: f64, f_number: f64) -> f64 {
    airy_disc_radius(wavelength, f_number)
}

/// 2-D discrete convolution for small kernels (NxN image with KxK kernel).
/// Simple direct implementation — use FFT-based for large images.
pub fn convolve2d(image: &[Vec<f64>], kernel: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let rows = image.len();
    let cols = image[0].len();
    let kr = kernel.len();
    let kc = kernel[0].len();
    let pr = kr / 2;
    let pc = kc / 2;
    let mut out = vec![vec![0.0; cols]; rows];
    for r in 0..rows {
        for c in 0..cols {
            let mut sum = 0.0;
            for i in 0..kr {
                for j in 0..kc {
                    let ir = r as isize + i as isize - pr as isize;
                    let ic = c as isize + j as isize - pc as isize;
                    if ir >= 0 && ir < rows as isize && ic >= 0 && ic < cols as isize {
                        sum += image[ir as usize][ic as usize] * kernel[i][j];
                    }
                }
            }
            out[r][c] = sum;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn identity_kernel_passes_image() {
        let image = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0], vec![7.0, 8.0, 9.0]];
        let kernel = vec![vec![0.0, 0.0, 0.0], vec![0.0, 1.0, 0.0], vec![0.0, 0.0, 0.0]];
        let out = convolve2d(&image, &kernel);
        for r in 0..3 {
            for c in 0..3 {
                assert_abs_diff_eq!(out[r][c], image[r][c], epsilon = 1e-12);
            }
        }
    }
}
