//! Matrix
use super::{
    //MatrixColumnIterator,
    //MatrixColumnIteratorMut,
    MatrixColumnIntoIterator,
    MatrixIntoIterator,
    MatrixIterator,
    MatrixIteratorMut,
    //MatrixRowIterator,
    //MatrixRowIteratorMut,
    MatrixRowIntoIterator
};
use crate::{
    algebra::{
        abstr::{Addition, Field, Identity, Multiplication, Scalar},
        linear::{matrix::Substitute, Vector, matrix::Transpose},
        abstr::{AbsDiffEq, RelativeEq},
    },
    elementary::Power,
};
use rand::{self, Rng};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::clone::Clone;
use std::{fmt, fmt::Display};
use std::convert::From;

/// Macro to construct matrices
///
/// ```
/// # #[macro_use]
/// # extern crate mathru;
/// # fn main()
/// # {
/// use mathru::algebra::linear::Matrix;
///
/// // Construct a 2x3 matrix of f32
/// let mat: Matrix<f32> = matrix![1.0, 2.0, 3.0; 4.0, 5.0, 6.0];
/// let (m, n) = mat.dim();
///
/// assert_eq!(m, 2);
/// assert_eq!(n, 3);
/// # }
/// ```
#[macro_export]
macro_rules! matrix
{
    ($( $( $x: expr ),*);*) =>
    {
        {
            let data_nested_array = [ $( [ $($x),* ] ),* ];
            let rows = data_nested_array.len();
            let cols = data_nested_array[0].len();
            let mut data_array: Vec<_> = Vec::with_capacity(rows * cols);
            for j in 0..cols
            {
                for i in 0..rows
                {
                    data_array.push(data_nested_array[i][j]);
                }
            }
            Matrix::new(rows, cols, data_array)
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct Matrix<T>
{
    /// Num of rows which the matrix has
    pub(super) m: usize,
    /// Num of columns which the matrix ha
    pub(super) n: usize,
    /// Matrix entries
    pub(super) data: Vec<T>,
}

impl<T> From<Vector<T>> for Matrix<T> where T: Field + Scalar
{
    fn from(v: Vector<T>) -> Self
    {
        let (v_m, v_n): (usize, usize) = v.dim();

        Matrix::new(v_m, v_n, v.convert_to_vec())
    }
}

impl<T> IntoIterator for Matrix<T> where T: Field + Scalar
{
    type Item = T;
    type IntoIter = MatrixIntoIterator<T>;

    fn into_iter(self) -> Self::IntoIter
    {
        MatrixIntoIterator { iter: self.data.into_iter() }
    }
}

impl<T> Matrix<T>
{
    pub fn iter(&self) -> MatrixIterator<T>
    {
        MatrixIterator::new( self.data.iter() )
    }

    pub fn iter_mut(&mut self) -> MatrixIteratorMut<T>
    {
        MatrixIteratorMut::new(self.data.iter_mut())
    }

    pub fn row_into_iter(&self) -> MatrixRowIntoIterator<T>
    {
        MatrixRowIntoIterator::new(self)
    }
    // pub fn row_iter(&self) -> MatrixRowIterator<T>
    //     where T: Zero
    // {
    //     MatrixRowIterator::new(self.data.iter())
    // }

    // pub fn row_iter_mut(&mut self) -> MatrixRowIteratorMut<T>
    //     where T: Zero
    // {
    //     MatrixRowIteratorMut::new(self.data.iter_mut())
    // }

    pub fn column_into_iter(&self) -> MatrixColumnIntoIterator<T>
    {
         MatrixColumnIntoIterator::new(self)
    }

    // pub fn column_iter(&self) -> MatrixColumnIterator<T>
    // {
    //     MatrixColumnIterator::new(self.data.iter())
    // }

    // pub fn column_iter_mut(&mut self) -> MatrixColumnIteratorMut<T>
    // {
    //     MatrixColumnIteratorMut::new(self.data.iter_mut())
    // }
}

impl<T> Matrix<T>
    where T: Clone
{
    /// Applies the function f on every element in the matrix
    pub fn apply_mut(mut self: Matrix<T>, f: &dyn Fn(&T) -> T) -> Matrix<T>
    {
        self.data = self.data.iter().map(f).collect::<Vec<T>>();
        self
    }

    pub fn apply(self: &Matrix<T>, f: &dyn Fn(&T) -> T) -> Matrix<T>
    {
        (self.clone()).apply_mut(f)
    }

    pub fn mut_apply(self: &mut Matrix<T>, f: &dyn Fn(&mut T) -> T)
    {
        self.data = self.data.iter_mut().map(f).collect::<Vec<T>>();
    }
}

impl<T> Matrix<T>
    where T: Field + Scalar + Power
{
    /// Calculates the trace of a matrix
    ///
    /// # Arguments
    ///
    /// self: square matrix
    ///
    /// # Panics
    ///
    /// if it is not a square matrix
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Matrix;
    ///
    /// let a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, -2.0, 3.0, -7.0]);
    /// let tr: f64 = a.trace();
    ///
    /// assert_eq!(-6.0, tr);
    /// ```
    pub fn trace(&self) -> T
        where T: Field + Scalar
    {
        let (m, n): (usize, usize) = self.dim();
        if m != n
        {
            panic!("matrix is not square");
        }

        let mut sum: T = T::zero();
        for i in 0..m
        {
            sum += self[[i, i]];
        }

        sum
    }
}

#[cfg(feature = "native")]
impl<T> Matrix<T> where T: Scalar
{
    pub(super) fn swap_rows(&mut self, i: usize, j: usize)
    {
        for k in 0..self.n
        {
            let temp: T = self[[i, k]];
            self[[i, k]] = self[[j, k]];
            self[[j, k]] = temp;
        }
    }
}

impl<T> Matrix<T> where T: Field + Scalar
{
    // returns column vector
    pub fn get_column(&self, i: usize) -> Vector<T>
    {
        assert!(i < self.n);

        let mut v: Vector<T> = Vector::zero(self.m);

        for k in 0..self.m
        {
            v[k] = self[[k, i]];
        }

        v
    }

    /// return row vector
    ///
    /// i: row
    pub fn get_row(&self, i: usize) -> Vector<T>
    {
        assert!(i < self.m);

        let mut v: Vector<T> = Vector::zero(self.n);
        v = v.transpose();

        for k in 0..self.n
        {
            v[k] = self[[i, k]];
        }

        v
    }

    /// set column
    pub fn set_column(&mut self, column: &Vector<T>, i: usize)
    {
        let (m, _n) = column.dim();
        if m != self.m
        {
            panic!("Dimensions do not match");
        }

        for k in 0..self.m
        {
            self[[k, i]] = column[k];
        }
    }

    /// set row
    ///
    /// # Arguments
    /// * 'row'
    /// * 'i'
    ///
    /// # Panics
    pub fn set_row(&mut self, row: &Vector<T>, i: usize)
    {
        let (_m, n): (usize, usize) = row.dim();
        if n != self.n
        {
            panic!("Dimensions do not match");
        }

        for k in 0..self.n
        {
            self[[i, k]] = row[k];
        }
    }
}

impl<T> Matrix<T> where T: Field + Scalar + Power
{
    ///
    pub fn givens(m: usize, i: usize, j: usize, c: T, s: T) -> Self
    {
        if i >= m || j >= m
        {
            panic!("Index out of bounds");
        }

        let mut givens: Matrix<T> = Matrix::one(m);
        givens[[i, i]] = c;
        givens[[j, j]] = c;
        givens[[i, j]] = s;
        givens[[j, i]] = -s;
        givens
    }

    #[cfg(feature = "native")]
    /// function \[c,s \] = Givens(a,b)
    /// Givens rotation computation
    /// Determines cosine-sine pair (c,s) so that \[c s;-s c\]'*\[a;b\] = \[r;0\]
    /// GVL4: Algorithm 5.1.3
    pub fn givens_cosine_sine_pair(a: T, b: T) -> (T, T)
    {
        let exponent: T = T::from_f64(2.0);
        let exponent_sqrt: T = T::from_f64(0.5);

        let c: T;
        let s: T;

        if b == T::zero()
        {
            c = T::one();
            s = T::zero();
        }
        else
        {
            if b.abs() > a.abs()
            {
                let tau: T = -a / b;
                s = T::one() / (T::one() + tau.pow(exponent)).pow(exponent_sqrt);
                c = s * tau;
            }
            else
            {
                let tau: T = -b / a;
                c = T::one() / (T::one() + tau.pow(exponent)).pow(exponent_sqrt);
                s = c * tau;
            }
        }

        (c, s)
    }
}

impl<T> Matrix<T> where T: Field + Scalar + Power
{
    /// Returns the householder matrix
    ///
    /// # Arguments
    ///
    /// v: Column vector
    /// k: index 0 <= k < m
    ///
    /// # Panics
    ///
    /// if index out of bounds
    pub fn householder(v: &Vector<T>, k: usize) -> Self
    {
        let (v_m, _v_n): (usize, usize) = v.dim();
        if k >= v_m
        {
            panic!("Index k out of bounds");
        }

        if v_m == 0
        {
            panic!();
        }

        if v_m == 1
        {
            return Matrix::one(v_m);
        }

        let d: Vector<T> = v.get_slice(k, v_m - 1);

        let norm: T = T::from_f64(2.0);

        let alpha: T;

        let d_0: T = d[0];

        if d_0 >= T::zero()
        {
            alpha = -d.p_norm(&norm);
        }
        else
        {
            alpha = d.p_norm(&norm);
        }

        if alpha == T::zero()
        {
            println!("norm zero");
            let h: Matrix<T> = Matrix::one(v_m); // v_n
            return h;
        }

        let (d_m, _d_n) = d.dim();

        let mut v: Vector<T> = Vector::zero(d_m);

        v[0] = (T::from_f64(0.5) * (T::one() - d_0 / alpha)).pow(T::from_f64(0.5));
        let p: T = -alpha * v[0];

        if d_m - 1 >= 1
        {
            let temp: Vector<T> = d.get_slice(1, d_m - 1)
                                   .apply(&|e: &T| -> T { *e / (T::from_f64(2.0) * p) });
            v.set_slice(&temp, 1);
        }

        let mut w: Vector<T> = Vector::zero(v_m);

        w.set_slice(&v, k);

        let ident: Matrix<T> = Matrix::one(v_m);

        let two: T = T::from_f64(2.0);
        let w_dyadp: Matrix<T> = w.dyadp(&w);
        let h: Matrix<T> = &ident - &(&w_dyadp * &two);
        h
    }
}

impl<T> Matrix<T>
    where T: Field + Scalar
{
    /// Returns a slice of the matrix
    ///
    /// # Arugments
    ///
    /// 0 <= row_s < m \
    /// 0 <= row_e < m \
    /// 0 <= column_s < n \
    /// 0 <= column_e <= n \
    ///
    /// row_s: start row \
    /// row_e: end row \
    /// column_s: start column \
    /// column_e: end column \
    ///
    /// # Example
    ///
    /// ```
    /// # #[macro_use]
    /// # extern crate mathru;
    /// # fn main()
    /// # {
    /// use mathru::algebra::linear::{Matrix};
    ///
    /// let mut a: Matrix<f64> = matrix![1.0, -2.0; 3.0, -7.0];
    /// a = a.get_slice(0, 0, 1, 1);
    ///
    /// let a_ref: Matrix<f64> = Matrix::new(1, 1, vec![-2.0]);
    ///
    /// assert_eq!(a_ref, a);
    /// # }
    /// ```
    pub fn get_slice(&self,
                     row_s: usize,
                     row_e: usize,
                     column_s: usize,
                     column_e: usize)
                     -> Matrix<T>
    {
        assert!(row_s < self.m);
        assert!(row_e < self.m);
        assert!(column_s < self.n);
        assert!(column_e < self.n);

        let mut slice: Matrix<T> = Matrix::zero(row_e - row_s + 1, column_e - column_s + 1);

        for r in row_s..(row_e + 1)
        {
            for c in column_s..(column_e + 1)
            {
                slice[[r - row_s, c - column_s]] = self[[r, c]];
            }
        }
        slice
    }

    /// Replaces parts of the matrix with the given values
    ///
    /// # Arugments
    ///
    /// 0 <= row < m \
    /// 0 <= column < n
    ///
    /// # Example
    ///
    /// ```
    /// # #[macro_use]
    /// # extern crate mathru;
    /// # fn main()
    /// # {
    /// use mathru::algebra::linear::{Matrix};
    ///
    /// let mut a: Matrix<f64> = matrix![   1.0, 0.0;
    ///                                     3.0, -7.0];
    ///
    /// let b: Matrix<f64> = matrix![2.0, -1.0];
    /// a = a.set_slice(&b, 0, 0);
    ///
    /// let a_updated: Matrix<f64> = matrix![   2.0, -1.0;
    ///                                         3.0, -7.0];
    ///
    /// assert_eq!(a_updated, a);
    /// # }
    /// ```
    pub fn set_slice(mut self, slice: &Self, row: usize, column: usize) -> Matrix<T>
    {
        let (s_m, s_n): (usize, usize) = slice.dim();
        let (m, n): (usize, usize) = self.dim();
        assert!(row + s_m <= m);
        assert!(column + s_n <= n);

        for r in row..(row + s_m)
        {
            for c in column..(column + s_n)
            {
                self[[r, c]] = slice[[r - row, c - column]];
            }
        }
        self
    }
}

impl<T> Display for Matrix<T>
    where T: Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "\n").unwrap();
        for i in 0..self.m
        {
            for j in 0..self.n
            {
                write!(f, "{} ", self[[i, j]]).unwrap();
            }
            write!(f, "\n").unwrap();
        }
        write!(f, "\n")
    }
}



impl<T> PartialEq for Matrix<T>
    where T: PartialEq
{
    /// Checks if two matrices are equal
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Matrix;
    ///
    /// let a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let b: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    ///
    /// assert_eq!(true, a == b);
    /// ```
    fn eq(&self, other: &Self) -> bool
    {
        if self.dim() != other.dim()
        {
            return false;
        }

        if self.data == other.data
        {
            return true;
        }

        false
    }
}

impl<T> Matrix<T>
    where T: Clone + Copy
{
    /// Creates a new Matrix object
    ///
    /// Fortran like, column wise
    ///
    /// [
    ///   0, 1, 2]
    ///   3, 4, 5,
    ///   6, 7, 8
    /// ] => vec![ 0, 3, 6, 1, 4, 7, 2, 5, 8]
    pub fn new(m: usize, n: usize, data: Vec<T>) -> Self
    {
        // assert_eq!(m * n, data.len());
        Matrix { m, n, data }
    }
}

impl<T> Matrix<T>
{
    pub fn convert_to_vec(self) -> Vec<T>
    {
        self.data
    }
}

impl<T> Matrix<T>
    where T: Scalar + Clone + Copy
{
    pub fn new_random(m: usize, n: usize) -> Matrix<T>
    {
        let mut rng = rand::thread_rng();
        let data: Vec<T> = vec![T::from_f64(rng.gen()); m * n];
        Matrix::new(m, n, data)
    }
}

impl<T> Matrix<T>
    where T: Field + Scalar
{
    /// Returns the zero matrix(additive neutral element)
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Matrix;
    ///
    /// let a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let b: Matrix<f64> = &a + &Matrix::zero(2, 2);
    /// ```
    pub fn zero(m: usize, n: usize) -> Self
    {
        Matrix { m, n, data: vec![T::zero(); m * n] }
    }
}

impl<T> Identity<Addition> for Matrix<T>
    where T: Identity<Addition>
{
    /// Returns the additive neutral element)
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Matrix;
    ///
    /// ```
    fn id() -> Self
    {
        //        Matrix {
        //            m: m,
        //            n: n,
        //            data: vec![Identity::<Addition>::id(); m * n],
        //        }

        unimplemented!();
    }
}

impl<T> Matrix<T>
    where T: Field + Scalar
{
    /// Returns the eye matrix(multiplicative neutral element)
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Matrix;
    ///
    /// let a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let b: Matrix<f64> = &a * &Matrix::one(2);
    /// ```
    pub fn one(size: usize) -> Self
    {
        let mut data: Vec<T> = vec![Identity::<Addition>::id(); size * size];

        for i in 0..size
        {
            data[i * size + i] = Identity::<Multiplication>::id();
        }

        Matrix { m: size,
                 n: size,
                 data }
    }

    pub fn ones(m: usize, n: usize) -> Self
    {
        Matrix { m,
                 n,
                 data: vec![Identity::<Multiplication>::id(); m * n] }
    }
}

impl<T> Matrix<T>
    where T: Field + Scalar + Power + AbsDiffEq
{
    /// Calculates the pseudo inverse matrix
    ///
    /// A^+ = (A^TA)^-1A^T
    pub fn pinv(&self) -> Result<Matrix<T>, ()>
    {
        let r: Matrix<T> = self.dec_qr()?.r();
        let x: Matrix<T> = r.clone()
                            .transpose()
                            .substitute_forward(self.clone().transpose())?;
        r.substitute_backward(x)
    }
}

impl<T> Matrix<T>
{
    /// Returns the matrix dimension
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Matrix;
    ///
    /// let a: Matrix<f64> = Matrix::new(4, 2, vec![1.0, 0.0, 3.0, 0.0, 1.0, -7.0, 0.5, 0.25]);
    /// let (m, n) = a.dim();
    ///
    /// assert_eq!(4, m);
    /// assert_eq!(2, n);
    /// ```
    pub fn dim(&self) -> (usize, usize)
    {
        (self.m, self.n)
    }

    /// Returns the number of rows
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::{Matrix};
    ///
    /// let a: Matrix<f64> = Matrix::new(4, 2, vec![1.0, 0.0, 3.0, 0.0, 1.0, -7.0, 0.5, 0.25]);
    /// let m: usize = a.nrows();
    ///
    /// assert_eq!(4, m);
    pub fn nrows(&self) -> usize
    {
        self.m
    }

    /// Returns the number of columns
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Matrix;
    ///
    /// let a: Matrix<f64> = Matrix::new(4, 2, vec![1.0, 0.0, 3.0, 0.0, 1.0, -7.0, 0.5, 0.25]);
    /// let n: usize = a.ncols();
    ///
    /// assert_eq!(2, n);
    /// ```
    pub fn ncols(&self) -> usize
    {
        self.n
    }
}

impl<T> AbsDiffEq for Matrix<T>
    where T: Field + Scalar + AbsDiffEq<Epsilon = T>
{
    type Epsilon = T;

    fn default_epsilon() -> T
    {
        T::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Matrix<T>, epsilon: T) -> bool
    {
        for (a, b) in self.iter().zip(other.iter())
        {
            if a.abs_diff_ne(b, epsilon)
            {
                return false;
            }
        }

        true
    }
}

impl<T> RelativeEq for Matrix<T>
    where T: Field + Scalar + AbsDiffEq<Epsilon = T> + RelativeEq
{
    fn default_max_relative() -> T
    {
        T::default_max_relative()
    }

    /// A test for equality that uses a relative comparison if the values are far apart.
    fn relative_eq(&self, other: &Matrix<T>, epsilon: Self::Epsilon, max_relative: Self::Epsilon) -> bool
    {
        for (a, b) in self.iter().zip(other.iter())
        {
            if a.relative_ne(b, epsilon, max_relative)
            {
                return false;
            }
        }
        true
    }
}