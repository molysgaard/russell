use super::Matrix;
use crate::Norm;
use russell_openblas::{dlange, to_i32};

/// Computes the matrix norm
///
/// # Example
///
/// ```
/// use russell_lab::{mat_norm, Matrix, Norm};
///
/// fn main() {
///     let a = Matrix::from(&[
///         [-2.0,  2.0],
///         [ 1.0, -4.0],
///     ]);
///     assert_eq!(mat_norm(&a, Norm::One), 6.0);
///     assert_eq!(mat_norm(&a, Norm::Inf), 5.0);
///     assert_eq!(mat_norm(&a, Norm::Fro), 5.0);
///     assert_eq!(mat_norm(&a, Norm::Max), 4.0);
/// }
/// ```
pub fn mat_norm(a: &Matrix, kind: Norm) -> f64 {
    let (m, n) = a.dims();
    if m == 0 || n == 0 {
        return 0.0;
    }
    let norm = match kind {
        Norm::Euc | Norm::Fro => b'F',
        Norm::Inf => b'I',
        Norm::Max => b'M',
        Norm::One => b'1',
    };
    let (m_i32, n_i32) = (to_i32(m), to_i32(n));
    dlange(norm, m_i32, n_i32, &a.as_data())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::{mat_norm, Matrix};
    use crate::Norm;
    use russell_chk::approx_eq;

    #[test]
    fn mat_norm_works() {
        let a_0x0 = Matrix::new(0, 0);
        let a_0x1 = Matrix::new(0, 1);
        let a_1x0 = Matrix::new(1, 0);
        assert_eq!(mat_norm(&a_0x0, Norm::One), 0.0);
        assert_eq!(mat_norm(&a_0x0, Norm::Inf), 0.0);
        assert_eq!(mat_norm(&a_0x0, Norm::Euc), 0.0);
        assert_eq!(mat_norm(&a_0x0, Norm::Fro), 0.0);
        assert_eq!(mat_norm(&a_0x0, Norm::Max), 0.0);
        assert_eq!(mat_norm(&a_0x1, Norm::One), 0.0);
        assert_eq!(mat_norm(&a_0x1, Norm::Inf), 0.0);
        assert_eq!(mat_norm(&a_0x1, Norm::Fro), 0.0);
        assert_eq!(mat_norm(&a_0x1, Norm::Max), 0.0);
        assert_eq!(mat_norm(&a_1x0, Norm::One), 0.0);
        assert_eq!(mat_norm(&a_1x0, Norm::Inf), 0.0);
        assert_eq!(mat_norm(&a_1x0, Norm::Fro), 0.0);
        assert_eq!(mat_norm(&a_1x0, Norm::Max), 0.0);
        #[rustfmt::skip]
        let a = Matrix::from(&[
            [ 5.0, -4.0, 2.0],
            [-1.0,  2.0, 3.0],
            [-2.0,  1.0, 0.0],
        ]);
        assert_eq!(mat_norm(&a, Norm::One), 8.0);
        assert_eq!(mat_norm(&a, Norm::Inf), 11.0);
        assert_eq!(mat_norm(&a, Norm::Euc), 8.0);
        assert_eq!(mat_norm(&a, Norm::Fro), 8.0);
        assert_eq!(mat_norm(&a, Norm::Max), 5.0);

        // example from https://netlib.org/lapack/lug/node75.html
        #[rustfmt::skip]
        let diff = Matrix::from(&[
            [ 0.56, -0.36, -0.04],
            [ 0.91, -0.87, -0.66],
            [-0.36,  0.23,  0.93],
        ]);
        assert_eq!(mat_norm(&diff, Norm::Inf), 2.44);
        assert_eq!(mat_norm(&diff, Norm::One), 1.83);
        approx_eq(mat_norm(&diff, Norm::Fro), 1.87, 0.01);
    }
}
