#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]

//! Contains matrix definitions and the generic implementation
pub mod matrix_operations {
    //! Generic implementation for Mul, Add, and Sub from core::ops
    use core::ops::Add;
    use core::ops::Mul;
    use core::ops::Sub;

    /// Generic struct definition
    #[derive(Debug, Clone, PartialEq)]
    pub struct Matrix<T> {
        rows: u32,
        columns: u32,
        dimension: (u32, u32),
        data: Vec<Vec<T>>,
    }

    impl<T> Matrix<T>
    where
        T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Default + Clone,
    {
        /// Create an NxM matrix given a 2d vector with elements of type T
        pub fn from_data(data: Vec<Vec<T>>) -> Self {
            let rows: u32 = data.len() as u32;
            let columns: u32 = data.get(0).unwrap().len() as u32;

            Matrix {
                rows: rows,
                columns: columns,
                dimension: (rows.clone(), columns.clone()),
                data: data,
            }
        }

        /// Create an NxM matrix where every entry is populated with the value passed in the constant param
        pub fn from_constant(dimension: (u32, u32), constant: T) -> Self {
            Matrix {
                rows: dimension.0,
                columns: dimension.1,
                dimension: dimension,
                data: Matrix::data_from_constant(dimension.0, dimension.1, constant),
            }
        }

        /// Create an NxM diagonal matrix with diagonal taking values from the supplied constant
        pub fn diagonal_from_constant(dimension: (u32, u32), constant: T) -> Self {
            let mut m: Matrix<T> = Matrix::default_from_dimension(dimension);
            let min_dim: u32 = std::cmp::min(dimension.0, dimension.1);

            for i in 0..min_dim {
                m.data[i as usize][i as usize] = constant.clone();
            }

            return m;
        }

        /// Initialize a new NxM unit matrix, assuming T::default() is the unit for T.
        pub fn default_diagonal(dimension: (u32, u32)) -> Self {
            Self::diagonal_from_constant(dimension, T::default())
        }

        /// Create an NxM matrix by dimension where every entry is populated with default values for T
        pub fn default_from_dimension(dimension: (u32, u32)) -> Self {
            Matrix {
                rows: dimension.0,
                columns: dimension.1,
                dimension: dimension,
                data: Matrix::data_from_zeroes(dimension.0, dimension.1),
            }
        }
        /// Create an NxM matrix by row and column numbers where every entry is populated with default values for T
        pub fn default_from_rows_and_columns(rows: u32, columns: u32) -> Self {
            Matrix {
                rows: rows,
                columns: columns,
                dimension: (rows, columns),
                data: Matrix::data_from_zeroes(rows, columns),
            }
        }

        fn data_from_zeroes(r: u32, c: u32) -> Vec<Vec<T>> {
            return vec![vec![T::default(); c as usize]; r as usize];
        }

        fn data_from_constant(r: u32, c: u32, v: T) -> Vec<Vec<T>> {
            return vec![vec![v; c as usize]; r as usize];
        }

        /// Transpose matrix (i.e retrieve M^T for a matrix M)
        pub fn transpose(self) -> Matrix<T> {
            let mut transposed_matrix: Matrix<T> =
                Matrix::default_from_dimension((self.columns, self.rows));

            for i in 0..self.rows {
                let iu: usize = i as usize;
                for j in 0..self.columns {
                    let ju: usize = j as usize;
                    transposed_matrix.data[ju][iu] = self.data[iu][ju].clone();
                }
            }

            return transposed_matrix;
        }
    }

    impl<T> Add for Matrix<T>
    where
        T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Default + Clone,
    {
        type Output = Matrix<T>;

        fn add(self, rhs: Self) -> Self::Output {
            if self.dimension != rhs.dimension {
                panic!("Matrices must have the same dimension to perform addition");
            }

            let mut resultant: Matrix<T> = Matrix::default_from_dimension(self.dimension);
            for i in 0..self.rows {
                let iu: usize = i as usize;
                for j in 0..self.columns {
                    let ju: usize = j as usize;
                    resultant.data[iu][ju] = self.data[iu][ju].clone() + rhs.data[iu][ju].clone();
                }
            }

            return resultant;
        }
    }

    impl<T> Mul for Matrix<T>
    where
        T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Default + Clone,
    {
        type Output = Matrix<T>;

        fn mul(self, rhs: Self) -> Self::Output {
            if self.columns != rhs.rows {
                panic!("Incompatible dimensions for matrix multiplication.")
            }

            let mut resultant: Matrix<T> =
                Matrix::<T>::default_from_rows_and_columns(self.rows, rhs.columns);

            for i in 0..self.rows {
                let iu: usize = i as usize;
                for j in 0..rhs.columns {
                    let ju: usize = j as usize;
                    for k in 0..self.columns {
                        let ku: usize = k as usize;
                        resultant.data[iu][ju] = resultant.data[iu][ju].clone()
                            + self.data[iu][ku].clone() * rhs.data[ku][ju].clone();
                    }
                }
            }

            return resultant;
        }
    }

    impl<T> Sub for Matrix<T>
    where
        T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Default + Clone,
    {
        type Output = Matrix<T>;

        fn sub(self, rhs: Self) -> Self::Output {
            if self.dimension != rhs.dimension {
                panic!("Matrices must have the same dimension to perform subtraction");
            }

            let mut resultant: Matrix<T> = Matrix::default_from_dimension(self.dimension);
            for i in 0..self.rows {
                let iu: usize = i as usize;
                for j in 0..self.columns {
                    let ju: usize = j as usize;
                    resultant.data[iu][ju] = self.data[iu][ju].clone() - rhs.data[iu][ju].clone();
                }
            }

            return resultant;
        }
    }

    impl<T> Default for Matrix<T>
    where
        T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Default + Clone,
    {
        fn default() -> Self {
            Self {
                rows: 3,
                columns: 3,
                dimension: (3, 3),
                data: Matrix::data_from_zeroes(3, 3),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::matrix_operations::Matrix;

    #[test]
    fn test_addition() {
        let mat_ones: Matrix<i32> = Matrix::from_constant((3, 3), 1);
        assert_eq!(
            mat_ones.clone() + mat_ones,
            Matrix::from_constant((3, 3), 2)
        );
    }

    #[test]
    fn test_multiplication() {
        let mat_tens: Matrix<i32> = Matrix::from_constant((5, 5), 10);
        let mat_ones: Matrix<i32> = Matrix::from_constant((5, 5), 1);

        let product: Matrix<i32> = mat_tens * mat_ones;

        assert_eq!(product, Matrix::from_constant((5, 5), 50));
    }

    #[test]
    fn test_subtraction() {
        let mat_ones: Matrix<i32> = Matrix::from_constant((3, 3), 1);
        assert_eq!(
            mat_ones.clone() - mat_ones,
            Matrix::default_from_dimension((3, 3))
        );
    }

    #[test]
    fn test_diagonal_matrix_initialization() {
        let unit_matrix: Matrix<f32> = Matrix::diagonal_from_constant((5, 5), 1.0);
        let mat_tens: Matrix<f32> = Matrix::from_constant((5, 5), 10.0);

        assert_eq!(mat_tens.clone() * unit_matrix, mat_tens);
    }
}
