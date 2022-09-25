# Basic Outline
This is a simple implementation of matrices using generic parameters. Includes addition, subtraction, and multiplication. The main module is `matrix_operations`, and the actual implementation of a matrix is defined within under `Matrix`.

Matrices are defined generically, and can be initialized with any type satisfying:

```rust
impl<T> Matrix<T> 
where T: Add<Output = T> + Mul<Output = T> + 
Sub<Output = T> + Clone + 
Default
```

Despite having a lot of restrictions on `T`, this is actually a very easy set of criteria to satisfy. Most numeric types satisfy this, as will my implementation of complex numbers in the `complex-plane` crate. The `Matrix` implementation directly implements the above traits, so you can use the common operators `+, -, *` to work with them.

# Examples
### Initializing New Matrices
There are a number of initializers for matrices. 
```rust
// Define a matrix given a 2d vector of some type following the above bounds
let mat_from_data: Matrix<i32> = Matrix::from_data(some_predefined_2d_i32_vec);

// Create a 2x3 matrix where each entry is 1.25
let mat_from_constant: Matrix<f32> = Matrix::from_constant((2, 3), 1.25);

// Create a 16x8 matrix where each entry is i8::default()
let mat_from_dimension: Matrix<i8> = Matrix::default_from_dimension((16, 8));

// Create a 2x2 matrix where each entry is f32::default()
let mat_from_rows_and_columns: Matrix<f32> = Matrix::default_from_rows_and_columns((2,2));
```

All of the above use predefined constants or `T::default()` for whatever type you're using. There are also initializers for diagonal matrices as well.

```rust
// Initialize a 10x10 diagonal unit matrix taking integral values
let unit_matrix_z_10x10: Matrix<i8> = Matrix::diagonal_from_constant((10,10), 1);

// Initialize a 5x5 diagonal matrix with diagonal values equal to f32::default()
let diagonal_default_mat: Matrix<f32> = Matrix::default_diagonal((5,5));
```

### Common Operations
```rust
// Take the transpose, M^T, of some matrix M
let m_t: Matrix<i32> = m.transpose();

// Add two matrices, m_1 and m_2
let sum = m_1 + m_2;

// Take the product of two matrices, m_1 and m_2
let product = m_1 * m_2

// Subtract m_1 from m_2
let difference = m_2 - m_1
```

# Project Roadmap
I'd like to continue improving upon this (as of now) extremely simple implementation. I'd like to add support for:
- Multithreading of all matrix operations
- Include ability to take determinants and invert matrices
- Overhaul implementation to include lifetimes
- Add support for casting matrices