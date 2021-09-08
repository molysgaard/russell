use super::*;
use russell_lab::*;
use russell_openblas::idamax;

/// Verifies the linear system a ⋅ x = rhs
pub struct VerifyLinSys {
    pub max_abs_a: f64,      // max abs a
    pub max_abs_ax: f64,     // max abs a ⋅ x
    pub max_abs_diff: f64,   // max abs diff = a ⋅ x - rhs
    pub relative_error: f64, // max_abs_diff / (max_abs_a + 1)
    pub time_check: u128,    // elapsed time spent in the `new` method
}

impl VerifyLinSys {
    /// Creates a new verification dataset
    ///
    /// ```text
    /// diff : = |a ⋅ x - rhs|
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// # fn main() -> Result<(), &'static str> {
    /// // import
    /// use russell_lab::*;
    /// use russell_sparse::*;
    ///
    /// // set sparse matrix (3 x 3) with 4 non-zeros
    /// let mut trip = SparseTriplet::new(3, 3, 4, false)?;
    /// trip.put(0, 0, 1.0);
    /// trip.put(0, 2, 4.0);
    /// trip.put(1, 1, 2.0);
    /// trip.put(2, 2, 3.0);
    ///
    /// // check matrix
    /// let (m, n) = trip.dims();
    /// let mut a = Matrix::new(m, n);
    /// trip.to_matrix(&mut a)?;
    /// let correct_a = "┌       ┐\n\
    ///                  │ 1 0 4 │\n\
    ///                  │ 0 2 0 │\n\
    ///                  │ 0 0 3 │\n\
    ///                  └       ┘";
    /// assert_eq!(format!("{}", a), correct_a);
    ///
    /// // verify lin-sys
    /// let x = Vector::from(&[1.0, 1.0, 1.0]);
    /// let rhs = Vector::from(&[5.0, 2.0, 3.0]);
    /// let verify = VerifyLinSys::new(&trip, &x, &rhs)?;
    /// assert_eq!(verify.max_abs_a, 4.0);
    /// assert_eq!(verify.max_abs_ax, 5.0);
    /// assert_eq!(verify.max_abs_diff, 0.0);
    /// assert_eq!(verify.relative_error, 0.0);
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(trip: &SparseTriplet, x: &Vector, rhs: &Vector) -> Result<Self, &'static str> {
        if x.dim() != trip.ncol || rhs.dim() != trip.nrow {
            return Err("vector dimensions are incompatible");
        }
        // start stopwatch
        let mut sw = Stopwatch::new("");

        // compute max_abs_a
        let nnz = to_i32(trip.pos);
        let idx = idamax(nnz, &trip.values_aij, 1);
        let max_abs_a = f64::abs(trip.values_aij[idx as usize]);

        // compute max_abs_ax
        let mut ax = trip.mat_vec_mul(&x)?;
        let max_abs_ax = ax.norm(EnumVectorNorm::Max);

        // compute max_abs_diff
        update_vector(&mut ax, -1.0, &rhs)?; // ax := ax - rhs
        let max_abs_diff = ax.norm(EnumVectorNorm::Max);

        // compute relative_error
        let relative_error = max_abs_diff / (max_abs_a + 1.0);

        // stop stopwatch
        let time_check = sw.stop();

        // results
        Ok(VerifyLinSys {
            max_abs_a,
            max_abs_ax,
            max_abs_diff,
            relative_error,
            time_check,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_works() -> Result<(), &'static str> {
        // | 1  3 -2 |
        // | 3  5  6 |
        // | 2  4  3 |
        let mut trip = SparseTriplet::new(3, 3, 9, false)?;
        trip.put(0, 0, 1.0);
        trip.put(0, 1, 3.0);
        trip.put(0, 2, -2.0);
        trip.put(1, 0, 3.0);
        trip.put(1, 1, 5.0);
        trip.put(1, 2, 6.0);
        trip.put(2, 0, 2.0);
        trip.put(2, 1, 4.0);
        trip.put(2, 2, 3.0);
        let x = Vector::from(&[-15.0, 8.0, 2.0]);
        let rhs = Vector::from(&[5.0, 7.0, 8.0]);
        let verify = VerifyLinSys::new(&trip, &x, &rhs)?;
        assert_eq!(verify.max_abs_a, 6.0);
        assert_eq!(verify.max_abs_ax, 8.0);
        assert_eq!(verify.max_abs_diff, 0.0);
        assert_eq!(verify.relative_error, 0.0);
        assert!(verify.time_check > 0);
        Ok(())
    }
}
