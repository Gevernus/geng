use super::*;

mod extra;
mod ops;
mod projection;
mod transform;

/// 3x3 matrix
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Mat3<T>(pub(crate) [[T; 3]; 3]);

impl<T> Mat3<T> {
    pub fn map<U, F: Fn(T) -> U>(self, f: F) -> Mat3<U> {
        Mat3(self.0.map(|row| row.map(&f)))
    }
}

impl<T: Copy> Mat3<T> {
    /// Construct a matrix.
    ///
    /// # Examples
    /// ```
    /// use batbox::prelude::*;
    /// let matrix = Mat3::new([
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    ///     [7, 8, 9],
    /// ]);
    /// ```
    pub fn new(values: [[T; 3]; 3]) -> Self {
        Self { 0: values }.transpose()
    }

    pub fn row(&self, row_index: usize) -> Vec3<T> {
        vec3(
            self[(row_index, 0)],
            self[(row_index, 1)],
            self[(row_index, 2)],
        )
    }
}

impl<T> Index<(usize, usize)> for Mat3<T> {
    type Output = T;
    fn index(&self, (row, col): (usize, usize)) -> &T {
        &self.0[col][row]
    }
}

impl<T> IndexMut<(usize, usize)> for Mat3<T> {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut T {
        &mut self.0[col][row]
    }
}

impl<T> Mat3<T> {
    pub fn as_flat_array(&self) -> &[T; 9] {
        unsafe { mem::transmute(self) }
    }
    pub fn as_flat_array_mut(&mut self) -> &mut [T; 9] {
        unsafe { mem::transmute(self) }
    }
}

impl<T: Num + Copy> Mat3<T> {
    /// Construct zero matrix.
    ///
    /// # Examples
    /// ```
    /// use batbox::prelude::*;
    /// let matrix = Mat3::<i32>::zero();
    /// for i in 0..3 {
    ///     for j in 0..3 {
    ///         assert_eq!(matrix[(i, j)], 0);
    ///     }
    /// }
    /// ```
    pub fn zero() -> Self {
        Mat3([[T::ZERO; 3]; 3])
    }

    /// Construct identity matrix.
    ///
    /// # Examples
    /// ```
    /// use batbox::prelude::*;
    /// let matrix = Mat3::<i32>::identity();
    /// for i in 0..3 {
    ///     for j in 0..3 {
    ///         assert_eq!(matrix[(i, j)], if i == j { 1 } else { 0 });
    ///     }
    /// }
    /// ```
    pub fn identity() -> Self {
        let mut result = Self::zero();
        for i in 0..3 {
            result[(i, i)] = T::ONE;
        }
        result
    }
}

impl<T: Float> ApproxEq for Mat3<T> {
    fn approx_distance_to(&self, other: &Self) -> f32 {
        let mut dist = 0.0;
        for i in 0..3 {
            for j in 0..3 {
                dist = partial_max(dist, (other[(i, j)] - self[(i, j)]).abs().as_f32());
            }
        }
        dist
    }
}
