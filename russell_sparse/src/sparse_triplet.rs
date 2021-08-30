use super::*;
use russell_lab::*;
use std::fmt;

#[repr(C)]
pub(crate) struct ExternalSparseTriplet {
    data: [u8; 0],
    marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

extern "C" {
    fn new_sparse_triplet(max: i32) -> *mut ExternalSparseTriplet;
    fn drop_sparse_triplet(trip: *mut ExternalSparseTriplet);
    fn sparse_triplet_set(
        trip: *mut ExternalSparseTriplet,
        pos: i32,
        i: i32,
        j: i32,
        x: f64,
    ) -> i32;
    fn sparse_triplet_get(
        trip: *mut ExternalSparseTriplet,
        pos: i32,
        i: *mut i32,
        j: *mut i32,
        x: *mut f64,
    ) -> i32;
}

/// Holds triples (i,j,x) representing a sparse matrix
pub struct SparseTriplet {
    pub(crate) nrow: usize,     // [i32] number of rows
    pub(crate) ncol: usize,     // [i32] number of columns
    pub(crate) pos: usize,      // [i32] current index => nnz in the end
    pub(crate) max: usize,      // [i32] max allowed number of entries
    pub(crate) symmetric: bool, // symmetric matrix?, but WITHOUT both sides of the diagonal

    data: *mut ExternalSparseTriplet,
}

impl SparseTriplet {
    /// Creates a new SparseTriplet representing a sparse matrix
    ///
    /// ```text
    /// trip  :=  sparse(a)
    /// (max)    (nrow,ncol)
    /// ```
    ///
    /// # Input
    ///
    /// `nrow` -- The number of rows of the sparse matrix
    /// `ncol` -- The number of columns of the sparse matrix
    /// `max` -- The maximum number fo non-zero values in the sparse matrix
    ///
    /// # Example
    /// ```
    /// # fn main() -> Result<(), &'static str> {
    /// use russell_sparse::*;
    /// let trip = SparseTriplet::new(3, 3, 5)?;
    /// let correct: &str = "=========================\n\
    ///                      SparseTriplet\n\
    ///                      -------------------------\n\
    ///                      nrow      = 3\n\
    ///                      ncol      = 3\n\
    ///                      max       = 5\n\
    ///                      pos       = 0\n\
    ///                      symmetric = false\n\
    ///                      =========================";
    /// assert_eq!(format!("{}", trip), correct);
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(nrow: usize, ncol: usize, max: usize) -> Result<Self, &'static str> {
        if nrow == 0 || ncol == 0 || max == 0 {
            return Err("nrow, ncol, and max must all be greater than zero");
        }
        let max_i32 = to_i32(max);
        unsafe {
            let data = new_sparse_triplet(max_i32);
            if data.is_null() {
                return Err("c-code failed to allocate SparseTriplet");
            }
            Ok(SparseTriplet {
                nrow,
                ncol,
                pos: 0,
                max,
                symmetric: false,
                data,
            })
        }
    }

    /// Puts the next triple (i,j,x) into the Triplet
    ///
    /// # Example
    ///
    /// ```
    /// # fn main() -> Result<(), &'static str> {
    /// use russell_sparse::*;
    /// let mut trip = SparseTriplet::new(2, 2, 1)?;
    /// trip.put(0, 0, 1.0)?;
    /// let correct: &str = "=========================\n\
    ///                      SparseTriplet\n\
    ///                      -------------------------\n\
    ///                      nrow      = 2\n\
    ///                      ncol      = 2\n\
    ///                      max       = 1\n\
    ///                      pos       = 1 (FULL)\n\
    ///                      symmetric = false\n\
    ///                      =========================";
    /// assert_eq!(format!("{}", trip), correct);
    /// # Ok(())
    /// # }
    /// ```
    pub fn put(&mut self, i: usize, j: usize, x: f64) -> Result<(), &'static str> {
        if i >= self.nrow {
            return Err("i index must be smaller than nrow");
        }
        if j >= self.ncol {
            return Err("j index must be smaller than ncol");
        }
        if self.pos >= self.max {
            return Err("max number of entries reached");
        }
        let i_i32 = to_i32(i);
        let j_i32 = to_i32(j);
        let pos_i32 = to_i32(self.pos);
        unsafe {
            let res = sparse_triplet_set(self.data, pos_i32, i_i32, j_i32, x);
            if res == C_HAS_ERROR {
                return Err("c-code failed to put (i,j,x) triple");
            }
            self.pos += 1;
        }
        Ok(())
    }

    /// Returns the dimensions of the matrix represented by the (i,j,x) triples
    ///
    /// # Example
    ///
    /// ```
    /// # fn main() -> Result<(), &'static str> {
    /// use russell_sparse::*;
    /// let trip = SparseTriplet::new(2, 2, 1)?;
    /// assert_eq!(trip.dims(), (2, 2));
    /// # Ok(())
    /// # }
    /// ```
    pub fn dims(&self) -> (usize, usize) {
        (self.nrow, self.ncol)
    }

    /// Converts the triples data to a matrix, up to a limit
    ///
    /// # Input
    ///
    /// `a` -- (nrow_max, ncol_max) matrix to hold the triples data. Thus, the matrix may have less rows or less columns than the triplet data
    ///
    /// # Example
    ///
    /// ```
    /// # fn main() -> Result<(), &'static str> {
    /// // import
    /// use russell_lab::*;
    /// use russell_sparse::*;
    ///
    /// // define (4 x 4) sparse matrix with 6 non-zero values
    /// let mut trip = SparseTriplet::new(4, 4, 6)?;
    /// trip.put(0, 0, 1.0)?;
    /// trip.put(0, 1, 2.0)?;
    /// trip.put(1, 0, 3.0)?;
    /// trip.put(1, 1, 4.0)?;
    /// trip.put(2, 2, 5.0)?;
    /// trip.put(3, 3, 6.0)?;
    ///
    /// // convert the first (3 x 3) values
    /// let mut a = Matrix::new(3, 3);
    /// trip.to_matrix(&mut a)?;
    /// let correct = "┌       ┐\n\
    ///                │ 1 2 0 │\n\
    ///                │ 3 4 0 │\n\
    ///                │ 0 0 5 │\n\
    ///                └       ┘";
    /// assert_eq!(format!("{}", a), correct);
    ///
    /// // convert the first (4 x 4) values
    /// let mut b = Matrix::new(4, 4);
    /// trip.to_matrix(&mut b)?;
    /// let correct = "┌         ┐\n\
    ///                │ 1 2 0 0 │\n\
    ///                │ 3 4 0 0 │\n\
    ///                │ 0 0 5 0 │\n\
    ///                │ 0 0 0 6 │\n\
    ///                └         ┘";
    /// assert_eq!(format!("{}", b), correct);
    /// # Ok(())
    /// # }
    /// ```
    pub fn to_matrix(&self, a: &mut Matrix) -> Result<(), &'static str> {
        let (m, n) = a.dims();
        if m > self.nrow || n > self.ncol {
            return Err("wrong matrix dimensions");
        }
        let m_i32 = to_i32(m);
        let n_i32 = to_i32(n);
        let mut i_32: i32 = 0;
        let mut j_32: i32 = 0;
        let mut x: f64 = 0.0;
        for p in 0..self.pos {
            let p_i32 = to_i32(p);
            unsafe {
                let res = sparse_triplet_get(self.data, p_i32, &mut i_32, &mut j_32, &mut x);
                if res == C_HAS_ERROR {
                    return Err("c-code failed to get (i,j,x) triple");
                }
            }
            if i_32 < m_i32 && j_32 < n_i32 {
                a.set(i_32 as usize, j_32 as usize, x)?;
            }
        }
        Ok(())
    }
}

impl Drop for SparseTriplet {
    /// Tells the c-code to release memory
    fn drop(&mut self) {
        unsafe {
            drop_sparse_triplet(self.data);
        }
    }
}

impl fmt::Display for SparseTriplet {
    /// Implements the Display trait
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pos = if self.pos == self.max {
            format!("{} (FULL)", self.pos)
        } else {
            format!("{}", self.pos)
        };
        write!(
            f,
            "=========================\n\
             SparseTriplet\n\
             -------------------------\n\
             nrow      = {}\n\
             ncol      = {}\n\
             max       = {}\n\
             pos       = {}\n\
             symmetric = {}\n\
             =========================",
            self.nrow, self.ncol, self.max, pos, self.symmetric
        )?;
        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_fails_on_wrong_dims() {
        assert_eq!(
            SparseTriplet::new(0, 3, 5).err(),
            Some("nrow, ncol, and max must all be greater than zero")
        );
        assert_eq!(
            SparseTriplet::new(3, 0, 5).err(),
            Some("nrow, ncol, and max must all be greater than zero")
        );
        assert_eq!(
            SparseTriplet::new(3, 3, 0).err(),
            Some("nrow, ncol, and max must all be greater than zero")
        );
    }

    #[test]
    fn new_works() -> Result<(), &'static str> {
        let trip = SparseTriplet::new(3, 3, 5)?;
        assert_eq!(trip.nrow, 3);
        assert_eq!(trip.ncol, 3);
        assert_eq!(trip.pos, 0);
        assert_eq!(trip.max, 5);
        assert_eq!(trip.symmetric, false);
        Ok(())
    }

    #[test]
    fn display_trait_works() -> Result<(), &'static str> {
        let mut trip = SparseTriplet::new(3, 3, 1)?;
        let correct_1: &str = "=========================\n\
                             SparseTriplet\n\
                             -------------------------\n\
                             nrow      = 3\n\
                             ncol      = 3\n\
                             max       = 1\n\
                             pos       = 0\n\
                             symmetric = false\n\
                             =========================";
        assert_eq!(format!("{}", trip), correct_1);
        trip.put(0, 0, 1.0)?;
        let correct_2: &str = "=========================\n\
                             SparseTriplet\n\
                             -------------------------\n\
                             nrow      = 3\n\
                             ncol      = 3\n\
                             max       = 1\n\
                             pos       = 1 (FULL)\n\
                             symmetric = false\n\
                             =========================";
        assert_eq!(format!("{}", trip), correct_2);
        Ok(())
    }

    #[test]
    fn put_fails_on_wrong_values() -> Result<(), &'static str> {
        let mut trip = SparseTriplet::new(1, 1, 1)?;
        assert_eq!(
            trip.put(1, 0, 0.0),
            Err("i index must be smaller than nrow")
        );
        assert_eq!(
            trip.put(0, 1, 0.0),
            Err("j index must be smaller than ncol")
        );
        trip.put(0, 0, 0.0)?; // << all spots occupied
        assert_eq!(trip.put(0, 0, 0.0), Err("max number of entries reached"));
        Ok(())
    }

    #[test]
    fn put_works() -> Result<(), &'static str> {
        let mut trip = SparseTriplet::new(3, 3, 5)?;
        trip.put(0, 0, 1.0)?;
        assert_eq!(trip.pos, 1);
        trip.put(0, 1, 2.0)?;
        assert_eq!(trip.pos, 2);
        trip.put(1, 0, 3.0)?;
        assert_eq!(trip.pos, 3);
        trip.put(1, 1, 4.0)?;
        assert_eq!(trip.pos, 4);
        trip.put(2, 2, 5.0)?;
        assert_eq!(trip.pos, 5);
        Ok(())
    }

    #[test]
    fn dims_works() -> Result<(), &'static str> {
        let trip = SparseTriplet::new(3, 2, 1)?;
        assert_eq!(trip.dims(), (3, 2));
        Ok(())
    }

    #[test]
    fn to_matrix_fails_on_wrong_dims() -> Result<(), &'static str> {
        let trip = SparseTriplet::new(1, 1, 1)?;
        let mut a_2x1 = Matrix::new(2, 1);
        let mut a_1x2 = Matrix::new(1, 2);
        assert_eq!(trip.to_matrix(&mut a_2x1), Err("wrong matrix dimensions"));
        assert_eq!(trip.to_matrix(&mut a_1x2), Err("wrong matrix dimensions"));
        Ok(())
    }

    #[test]
    fn to_matrix_works() -> Result<(), &'static str> {
        let mut trip = SparseTriplet::new(3, 3, 5)?;
        trip.put(0, 0, 1.0)?;
        trip.put(0, 1, 2.0)?;
        trip.put(1, 0, 3.0)?;
        trip.put(1, 1, 4.0)?;
        trip.put(2, 2, 5.0)?;
        let mut a = Matrix::new(3, 3);
        trip.to_matrix(&mut a)?;
        assert_eq!(a.get(0, 0)?, 1.0);
        assert_eq!(a.get(0, 1)?, 2.0);
        assert_eq!(a.get(1, 0)?, 3.0);
        assert_eq!(a.get(1, 1)?, 4.0);
        assert_eq!(a.get(2, 2)?, 5.0);
        let mut b = Matrix::new(2, 1);
        trip.to_matrix(&mut b)?;
        assert_eq!(b.get(0, 0)?, 1.0);
        assert_eq!(b.get(1, 0)?, 3.0);
        Ok(())
    }
}
